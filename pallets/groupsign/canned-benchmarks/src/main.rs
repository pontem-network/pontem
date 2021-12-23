use sp_core::{sr25519, Pair, blake2_256};
use sp_runtime::{MultiSignature, MultiSigner};
use codec::{Encode};

// We need to configure runtime, as otherwise it's quite painful to make a call

pub type AccountId = sp_core::sr25519::Public;

// ==== /Mock config
#[derive(Encode)]
pub struct TestCase {
    message: Vec<u8>,
    signatures: Vec<MultiSignature>,
    signers: Vec<MultiSigner>,
}

pub fn generate_test_preimage(call: &Vec<u8>, signers: &[AccountId]) -> [u8; 32] {
    let mut call_preimage = call.encode();
    call_preimage.extend(signers.encode());
    blake2_256(call_preimage.as_ref())
}

pub fn generate_example(signature_number: u32, length: u32) -> TestCase {
    let message = "Mow "
        .as_bytes()
        .iter()
        .cycle()
        .take(length as usize)
        .copied()
        .collect();

    let signer_pairs = (0..signature_number)
        .map(|_| sr25519::Pair::generate().0)
        .collect::<Vec<_>>();
    let preimage = generate_test_preimage(
        &message,
        &signer_pairs.iter().map(|s| s.public()).collect::<Vec<_>>(),
    );

    let (signers, signatures) = signer_pairs
        .iter()
        .map(|pair| {
            (
                MultiSigner::Sr25519(pair.public()),
                MultiSignature::Sr25519(pair.sign(&preimage)),
            )
        })
        .unzip();

    TestCase {
        message,
        signers,
        signatures,
    }
}

const MAX_TEST_SIGNERS: usize = 24;
const MAX_TEST_LENGTH: usize = 256;
const MAX_TEST_LENGTH_STEP: usize = 32;

pub fn main() {
    let a = (1..MAX_TEST_SIGNERS)
        .map(|signature_number| {
            (1..MAX_TEST_LENGTH)
                .step_by(MAX_TEST_LENGTH_STEP)
                .map(move |length| (signature_number, length))
        })
        .flatten()
        .map(|(signature_number, length)| {
            println!("Generating, sig#={}; l={}", signature_number, length);
            generate_example(signature_number as u32, length as u32)
        })
        .collect::<Vec<_>>();
    {
        let mut file =
            std::fs::File::create("benchmark_examples.codec").expect("Cannot create output file");

        a.encode_to(&mut file);
    }
}
