/// Make changes in canned-benchmarks/benchlib.rs first, and then copy them over to groupsign/src/benchmarking/benchlib.rs
/// Anything contained there should be no-crypto compatible.
///


#[cfg(feature = "runtime-benchmarks")]
use sp_std::prelude::*;
#[cfg(feature = "runtime-benchmarks")]
use sp_io::{hashing::blake2_256};
#[cfg(not(feature = "runtime-benchmarks"))]
use sp_core::{blake2_256};

use sp_runtime::{AccountId32, MultiSignature, verify_encoded_lazy, MultiSigner, traits::IdentifyAccount};
use codec::{Encode, Decode};


#[derive(Encode, Decode, Clone, Copy)]
pub struct TestInfo {
    pub sr: u32,
    pub ed: u32,
    pub ec: u32,
    pub len: u32,
}

#[derive(Encode, Decode)]
pub struct Signers {
    pub sr: Vec<AccountId32>,
    pub ed: Vec<AccountId32>,
    pub ec: Vec<AccountId32>
}

// ==== /Mock config
#[derive(Encode, Decode)]
pub struct TestCase {
    pub signatures: Vec<MultiSignature>,
    pub info: TestInfo
}

#[derive(Encode, Decode)]
pub struct CannedBenchmarks {
    pub test_cases: Vec<TestCase>,
    pub signers: Signers
}

impl CannedBenchmarks {

    /// Since we can't just give `benchmarking` module a list of tests, we map one back and forth.
    /// This one converts parameters provided by benchmarking process, and gets associated test back.
    ///
    pub fn get_by_parameters(&self, sr: u32, ed: u32, ec: u32, len: u32) -> (Vec<u8>, Vec<AccountId32>, Vec<MultiSignature>) {
        let index = get_index_from_parameters(sr, ed, ec, len);
        let test: &TestCase = &self.test_cases[index as usize];

        let sr_sigs = self.signers.sr[..sr as usize].iter().map(|f| f);
        let ed_sigs = self.signers.ed[..ed as usize].iter().map(|f| f);
        let ec_sigs = self.signers.ec[..ec as usize].iter().map(|f| f);
        let signers = sr_sigs.chain(ed_sigs).chain(ec_sigs).cloned().collect::<Vec<_>>();

        (make_message(len * MAX_TEST_LENGTH_STEP as u32), signers, test.signatures.clone())
    }
}

#[allow(dead_code)]
pub fn generate_test_preimage<AccountId: Encode>(
    call: &Vec<u8>,
    signers: &[AccountId],
) -> [u8; 32] {
    let mut call_preimage = call.encode();
    call_preimage.extend(signers.encode());
    blake2_256(call_preimage.as_ref())
}

/// Generates a reproducible message given it's length.
pub fn make_message(len: u32) -> Vec<u8> {
    "Mow "
    .as_bytes()
    .to_vec()
    .iter()
    .cycle()
    .take(len as usize)
    .copied()
    .collect::<Vec<_>>()
}

/// Returns reproducible part of a benchmark
#[allow(dead_code)]
pub fn get_benchmarking_sequence() -> Vec<TestInfo> {
    let mut sequence: Vec<TestInfo> = vec![];
    for sr in 0..MAX_TEST_SIGNERS+1 {
    for ed in 0..MAX_TEST_SIGNERS+1 {
    for ec in 0..MAX_TEST_SIGNERS+1 {
    for len in (0..MAX_TEST_LENGTH).step_by(MAX_TEST_LENGTH_STEP) {
        sequence.push(TestInfo { sr, ed, ec, len })
    }}}};
    sequence
}

pub const MAX_TEST_SIGNERS: u32 = 8;
pub const MAX_TEST_LENGTH: u32 = 1024*10 + 1;
pub const MAX_TEST_LENGTH_STEP: usize = 1024 * 2;

/// Upper bound for length benchmarking range
pub const LEN_STEPS: u32 = MAX_TEST_LENGTH / MAX_TEST_LENGTH_STEP as u32 + 1;
/// Upper bound for signatures benchmarking range
pub const SIG_STEPS: u32 = MAX_TEST_SIGNERS + 1;

/// Converts benchmarking parameters back to index.
/// Length in this context -- real length divied by MAX_TEST_LENGTH_STEP
pub fn get_index_from_parameters(sr: u32, ed: u32, ec: u32, len: u32) -> usize {
    (
        sr * (SIG_STEPS * SIG_STEPS * LEN_STEPS)
        + ed * (SIG_STEPS * LEN_STEPS)
        + ec * (LEN_STEPS)
        + len
    ) as usize
}

/// Actually goes through all the tests and verifies
#[test]
fn test_full_check() {
    let benchmarks = include_bytes!("../../src/benchmarking/benchmark_examples.codec");
    let tests = CannedBenchmarks::decode(&mut &benchmarks[..]).expect("Failed to decode test data");
    for a in 0..SIG_STEPS {
    for b in 0..SIG_STEPS {
    for c in 0..SIG_STEPS {
    for d in 0..LEN_STEPS {
        let (message, signers, signatures) = tests.get_by_parameters(a, b, c, d);
        let case = &tests.test_cases[get_index_from_parameters(a,b,c,d)];
        println!("{:3} {:3} {:3} {:5}", a, b, c, d);

        assert_eq!(message.len(), d as usize * MAX_TEST_LENGTH_STEP);
        assert_eq!(message.len(), case.info.len as usize);
        assert_eq!(signers.len(), (a + b + c) as usize);
        assert_eq!(signatures.len(), (a + b + c) as usize);

        let preimage = generate_test_preimage(&message, &signers);

        // Check that we can verify it after decoding
        let verified = Iterator::zip(signatures.into_iter(), signers.clone().into_iter())
            .map(|(sig, signer)| verify_encoded_lazy(&sig, &preimage, &signer))
            .all(|f| f);
        assert!(verified);
    }}}}
}

#[test]
fn test_range_conversions() {
    for (i, &t@TestInfo { sr, ed, ec, len }) in get_benchmarking_sequence().iter().enumerate() {
        let ri = get_index_from_parameters(sr, ed, ec, len / MAX_TEST_LENGTH_STEP as u32);
        println!("checking {:>5} == {:<5} | {:3} {:3} {:3} {:5}", i, ri, t.sr, t.ed, t.ec, t.len);
        assert_eq!(i, ri)
    }
}
