use p3_baby_bear::BabyBear;
use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_fri::{FriConfig, TwoAdicFriPcs};
use p3_keccak::Keccak256Hash;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher32};
use p3_uni_stark::StarkConfig;

pub type Val = BabyBear;
pub type Challenge = BinomialExtensionField<Val, 4>;
pub type ByteHash = Keccak256Hash;
pub type FieldHash = SerializingHasher32<ByteHash>;
pub type MyCompress = CompressionFunctionFromHasher<u8, ByteHash, 2, 32>;
pub type ValMmcs = FieldMerkleTreeMmcs<Val, u8, FieldHash, MyCompress, 32>;
pub type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
pub type Dft = Radix2DitParallel;
pub type Challenger = SerializingChallenger32<Val, HashChallenger<u8, ByteHash, 32>>;
pub type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
pub type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

pub fn default_config() -> MyConfig {
    let byte_hash = ByteHash {};
    let field_hash = FieldHash::new(Keccak256Hash {});

    let compress = MyCompress::new(byte_hash);

    let val_mmcs = ValMmcs::new(field_hash, compress.clone());

    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    let dft = Dft {};

    let fri_config = FriConfig {
        log_blowup: 2,
        num_queries: 42,
        proof_of_work_bits: 16,
        mmcs: challenge_mmcs,
    };
    let pcs = Pcs::new(dft, val_mmcs, fri_config);

    MyConfig::new(pcs)
}

pub fn default_challenger() -> Challenger {
    let byte_hash = ByteHash {};
    type Challenger = SerializingChallenger32<Val, HashChallenger<u8, ByteHash, 32>>;

    Challenger::from_hasher(vec![], byte_hash)
}
