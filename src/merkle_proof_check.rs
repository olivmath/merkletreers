use crate::hasher::Hashable;
use crate::{Leaf, Proof};

pub fn merkle_proof_check<H: Hashable>(proof: Proof, leaf: Leaf, hasher: &H) -> Leaf {
    let mut current_hash = leaf;

    for node in proof {
        let mut buffer = [0u8; 32];

        if node.side == 1.into() {
            hasher.hash_nodes(&current_hash, &node.data, &mut buffer);
        } else {
            hasher.hash_nodes(&node.data, &current_hash, &mut buffer);
        }

        current_hash = buffer;
    }

    current_hash
}
