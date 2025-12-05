use crate::{Hash, Leaf};
use tiny_keccak::{Hasher as KeccakHasher, Keccak};

/// Trait for custom hash functions
///
/// Implement this trait to use a custom hash function with the Merkle Tree
pub trait Hashable: Clone {
    /// Hash a single piece of data
    fn hash(&self, data: &[u8], buffer: &mut Hash);

    /// Hash two leaves together (used for building the tree)
    fn hash_nodes(&self, left: &Leaf, right: &Leaf, buffer: &mut Hash) {
        let mut concat = [0u8; 64];
        concat[..32].copy_from_slice(left);
        concat[32..].copy_from_slice(right);

        self.hash(&concat, buffer);
    }
}

/// Default hasher using Keccak256
#[derive(Clone, Copy, Debug)]
pub struct Keccak256Hasher;

impl Hashable for Keccak256Hasher {
    fn hash(&self, data: &[u8], buffer: &mut Hash) {
        let mut k256 = Keccak::v256();
        k256.update(data);
        k256.finalize(buffer);
    }
}

impl Default for Keccak256Hasher {
    fn default() -> Self {
        Keccak256Hasher
    }
}
