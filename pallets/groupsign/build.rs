#[cfg(std)]
mod build_benchmarks {
    use sp_core::{sr25519, Pair, blake2_256};
    use codec::{Encode};

    use sp_core::{H256};
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

    // We need to configure runtime, as otherwise it's quite painful to make a call

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;
    pub type AccountId = sp_core::sr25519::Public;

    // Configure a mock runtime to test the pallet.
    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        }
    );

    frame_support::parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const SS58Prefix: u8 = 42;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = AccountId;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = SS58Prefix;
        type OnSetCode = ();
    }

    // ==== /Mock config
    #[derive(Encode)]
    pub struct TestCase {
        call: Call,
        caller: AccountId,
        signatures: Vec<sr25519::Signature>,
        signers: Vec<AccountId>,
    }

    pub fn generate_preimage(caller: &AccountId, call: &Call, signers: &[AccountId]) -> [u8; 32] {
        let mut call_preimage = call.encode();
        call_preimage.extend(<Test as frame_system::Config>::BlockNumber::min_value().encode());
        call_preimage.extend(<Test as frame_system::Config>::BlockNumber::max_value().encode());
        call_preimage.extend(caller.encode());
        call_preimage.extend(<Test as frame_system::Config>::Index::min_value().encode());
        call_preimage.extend(signers.encode());
        blake2_256(call_preimage.as_ref())
    }

    pub fn generate_example(signature_number: u32, length: u32) -> TestCase {
        let call = Call::System(frame_system::Call::remark {
            remark: "Mow "
                .as_bytes()
                .iter()
                .cycle()
                .take(length as usize)
                .copied()
                .collect(),
        });

        let caller = sr25519::Pair::generate().0.public();
        let signer_pairs = (0..signature_number)
            .map(|_| sr25519::Pair::generate().0)
            .collect::<Vec<_>>();
        let preimage = generate_preimage(
            &caller,
            &call,
            &signer_pairs.iter().map(|s| s.public()).collect::<Vec<_>>(),
        );

        let (signers, signatures) = signer_pairs
            .iter()
            .map(|pair| (pair.public(), pair.sign(&preimage)))
            .unzip();

        TestCase {
            call,
            caller,
            signers,
            signatures,
        }
    }

    const MAX_TEST_SIGNERS: usize = 12;
    const MAX_TEST_LENGTH: usize = 256;
    const MAX_TEST_LENGTH_STEP: usize = 64;

    pub fn main() {
        println!("cargo:rerun-if-changed=build.rs");

        let a = (0..MAX_TEST_SIGNERS)
            .map(|signature_number| {
                (0..MAX_TEST_LENGTH)
                    .step_by(MAX_TEST_LENGTH_STEP)
                    .map(move |length| (signature_number, length))
            })
            .flatten()
            .map(|(signature_number, length)| {
                generate_example(signature_number as u32, length as u32)
            })
            .collect::<Vec<_>>();

        let mut file =
            std::fs::File::create("benchmark_examples.codec").expect("Cannot create output file");

        a.encode_to(&mut file);
    }
}

pub fn main() {
    #[cfg(std)]
    {
        use build_benchmarks::main;
        main();
    }
}
