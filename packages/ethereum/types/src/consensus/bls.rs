//! BLS12-381 signature types for Ethereum beacon chain.

use alloy_primitives::FixedBytes;

// See spec: <https://github.com/ethereum/consensus-specs/blob/ffa95b7b72149960c5aded5c95fb40d64bcab199/specs/phase0/beacon-chain.md#bls-signatures>
// And: <https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-bls-signature-04>
/// The Domain Separation Tag for `hash_to_point` in Ethereum beacon chain BLS12-381 signatures.
///
/// This is also the name of the ciphersuite that defines beacon chain BLS signatures.
pub const BLS_DST_SIG: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

/// The number of bytes in a BLS12-381 public key.
pub const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;

/// The number of bytes in a BLS12-381 secret key.
pub const BLS_SECRET_KEY_BYTES_LEN: usize = 32;

/// The number of bytes in a BLS12-381 signature.
pub const BLS_SIGNATURE_BYTES_LEN: usize = 96;

/// The bytes representing a BLS12-381 public key.
#[allow(clippy::module_name_repetitions)]
pub type BlsPublicKey = FixedBytes<BLS_PUBLIC_KEY_BYTES_LEN>;

/// The bytes representing a BLS12-381 signature.
#[allow(clippy::module_name_repetitions)]
pub type BlsSignature = FixedBytes<BLS_SIGNATURE_BYTES_LEN>;
