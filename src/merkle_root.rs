use crate::utils::hash_function;
use crate::{Hash, Leaf, Root};

pub fn merkle_root(leaves: &[Leaf]) -> Root {
    let mut node: Hash = [0u8; 32];

    let mut tmp = leaves.to_vec();

    while tmp.len() > 1 {
        let mut next_level: Vec<Leaf> = vec![];

        for leaf_pair in tmp.chunks(2) {
            match leaf_pair {
                [left, right] => hash_function(left, right, &mut node),
                [left] => node.copy_from_slice(left),
                _ => unreachable!(),
            };
            next_level.push(node);
        }

        tmp = next_level;
    }
    node
}
