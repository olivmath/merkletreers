use crate::utils::hash_function;
use crate::{Leaf, Proof};

pub fn merkle_proof_check(proof: Proof, leaf: Leaf) -> Leaf {
    let mut current_hash = leaf;

    for node in proof {
        let mut buffer = [0u8; 32];

        if node.side == 1.into() {
            hash_function(&current_hash, &node.data, &mut buffer);
        } else {
            hash_function(&node.data, &current_hash, &mut buffer);
        }

        current_hash = buffer;
    }

    current_hash
}
