
use benchlib::make_message;
use sp_core::{sr25519, Pair, ed25519, ecdsa};
use sp_runtime::{MultiSignature, MultiSigner, traits::IdentifyAccount, verify_encoded_lazy};
use codec::{Encode};

mod benchlib;
use benchlib::*;

pub struct GenContext {
    messages: Vec<Vec<u8>>,
    sr: Vec<MultiPair>,
    ed: Vec<MultiPair>,
    ec: Vec<MultiPair>
}

impl GenContext {
    fn get_message(&self, len: u32) -> &Vec<u8> {
        self.messages.get(len as usize / MAX_TEST_LENGTH_STEP).expect(format!("Non-standard message length : {}", len).as_str())
    }
    fn get_signers(&self) -> Signers {
        Signers {
            sr: self.sr.iter().map(<MultiPair>::public).map(<MultiSigner>::into_account).collect::<Vec<_>>(),
            ed: self.ed.iter().map(<MultiPair>::public).map(<MultiSigner>::into_account).collect::<Vec<_>>(),
            ec: self.ec.iter().map(<MultiPair>::public).map(<MultiSigner>::into_account).collect::<Vec<_>>()
        }
    }
    fn get_pairs(&self, info: &TestInfo) -> Vec<&MultiPair> {
        let sr_sigs = self.sr[..info.sr as usize].iter();
        let ed_sigs = self.ed[..info.ed as usize].iter();
        let ec_sigs = self.ec[..info.ec as usize].iter();
        sr_sigs.chain(ed_sigs).chain(ec_sigs).collect::<Vec<_>>()
    }

    fn generate() -> GenContext {
        GenContext {
            messages: (0..MAX_TEST_LENGTH).into_iter().step_by(MAX_TEST_LENGTH_STEP).map(|l| make_message(l)).collect::<Vec<_>>(),
            sr: (0..MAX_TEST_SIGNERS).into_iter().map(|_| gen_sig(0)).collect::<Vec<_>>(),
            ed: (0..MAX_TEST_SIGNERS).into_iter().map(|_| gen_sig(1)).collect::<Vec<_>>(),
            ec: (0..MAX_TEST_SIGNERS).into_iter().map(|_| gen_sig(2)).collect::<Vec<_>>()
        }
    }
}

#[derive(Clone)]
enum MultiPair {
    Sr25519(sr25519::Pair),
    Ed25519(ed25519::Pair),
    Ecdsa(ecdsa::Pair),
}

impl From<MultiPair> for MultiSigner {
    fn from(pair: MultiPair) -> Self {
        match pair {
            MultiPair::Sr25519(a) => MultiSigner::from(a.public()),
            MultiPair::Ed25519(a) => MultiSigner::from(a.public()),
            MultiPair::Ecdsa(a) => MultiSigner::from(a.public()),
        }
    }
}

/// :shrug:
impl From<&MultiPair> for MultiSigner {
    fn from(pair: &MultiPair) -> Self {
        pair.clone().into()
    }
}


impl MultiPair {
    fn public(&self) -> MultiSigner {
        self.clone().into()
    }
    fn sign(&self, message: &[u8]) -> MultiSignature {
        match self {
            MultiPair::Sr25519(a) => MultiSignature::from(a.sign(message)),
            MultiPair::Ed25519(a) => MultiSignature::from(a.sign(message)),
            MultiPair::Ecdsa(a) => MultiSignature::from(a.sign(message)),
        }
    }
}


fn gen_sig(sigtype: i32) -> MultiPair {
    match sigtype {
        0 => MultiPair::Sr25519(sr25519::Pair::generate().0),
        1 => MultiPair::Ed25519(ed25519::Pair::generate().0),
        2 => MultiPair::Ecdsa(ecdsa::Pair::generate().0),
        _ => panic!("sigtype is bad"),
    }
}


pub fn generate_example(info: TestInfo, context: &GenContext) -> TestCase {
    let message = context.get_message(info.len);
    assert_eq!(message.len() as u32, info.len);
    let signer_pairs = context.get_pairs(&info);

    let preimage = generate_test_preimage(
        &message,
        &signer_pairs.iter().map(|&s| s.public().into_account()).collect::<Vec<_>>(),
    );

    let signatures: Vec<MultiSignature> = signer_pairs
        .iter()
        .map(|&pair| pair.sign(&preimage) )
        .collect::<Vec<_>>();
        let verifiable = signer_pairs.iter().cloned().zip(&signatures).map(|(pair, signature)| {
            verify_encoded_lazy(signature, &preimage, &pair.public().into_account())
        }).all(|f| f);
        assert!(verifiable);

    TestCase {
        signatures,
        info
    }
}


pub fn main() {

    let sequence = get_benchmarking_sequence();
    let context = GenContext::generate();

    let num_tests = sequence.len();
    let cases =
    sequence.iter().enumerate().map(|(i, info@TestInfo{sr, ed, ec, len, ..})| {
        println!(
            "> {:6}/{:<6} sum={:<3} | len={:<5} | sr={:<3} | ed={:<3} | ec={:<3}",
            i+1, num_tests, sr+ed+ec, len, sr, ed, ec
        );
        generate_example(info.clone(), &context)
    })
    .collect::<Vec<_>>();

    // Writing a can of benchmarks
    {
        let mut file =
            std::fs::File::create("pallets/groupsign/src/benchmarking/benchmark_examples.codec").expect("Cannot create output file");

        let can = CannedBenchmarks {
            test_cases: cases,
            signers: context.get_signers()
        };

        can.encode_to(&mut file);
    }
}
