use crate::hasher::{Hashable, Keccak256Hasher};
use crate::Hash;

/// Helper function to hash data using Keccak256 (for backward compatibility)
pub fn hash_it(data: &[u8], buffer: &mut Hash) {
    let hasher = Keccak256Hasher;
    hasher.hash(data, buffer);
}

pub fn is_power_of_two(number: u32) -> bool {
    let left: bool = number & (number - 1) == 0;
    let right: bool = number != 0;

    left && right
}
