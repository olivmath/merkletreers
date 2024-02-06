use crate::utils::{hash_function, Node};

pub fn merkle_proof_check(proof: Vec<Node>, leaf: [u8; 32]) -> [u8; 32] {
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
