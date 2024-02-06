use crate::{Hash, Leaf};
use tiny_keccak::{Hasher, Keccak};

pub fn hash_it(data: &[u8], buffer: &mut Hash) {
    let mut k256 = Keccak::v256();

    k256.update(data);
    k256.finalize(buffer);
}

pub fn hash_function(left: &Leaf, right: &Leaf, buffer: &mut Hash) {
    let mut concat = [0u8; 64];
    concat[..32].copy_from_slice(left);
    concat[32..].copy_from_slice(right);

    hash_it(&concat, buffer)
}

pub fn is_power_of_two(number: u32) -> bool {
    let left: bool = number & (number - 1) == 0;
    let right: bool = number != 0;

    let r = left && right;
    r
}
