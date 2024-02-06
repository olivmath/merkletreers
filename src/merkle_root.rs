use crate::utils::hash_function;

pub fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    let mut node = [0u8; 32];

    let mut tmp = leaves.to_vec();

    while tmp.len() > 1 {
        let mut next_level: Vec<[u8; 32]> = vec![];

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
