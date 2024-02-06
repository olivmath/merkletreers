use crate::node::Node;
use crate::utils::hash_function;
use crate::{Leaf, Proof};

pub fn merkle_proof_mixed_tree(leaves: &[Leaf], leaf: Leaf) -> Proof {
    let mut proof: Proof = Vec::new();
    let mut leaf_index = leaves.iter().position(|x| x == &leaf).unwrap_or_else(|| {
        panic!("Leaf does not exist in the tree");
    });

    let mut current_leaves = leaves.to_vec();
    while current_leaves.len() > 1 {
        let next_leaves = up_layer(&current_leaves);

        if leaf_index % 2 == 0 {
            if leaf_index + 1 < current_leaves.len() {
                proof.push(Node {
                    data: current_leaves[leaf_index + 1],
                    side: 1.into(),
                });
            }
        } else {
            proof.push(Node {
                data: current_leaves[leaf_index - 1],
                side: 0.into(),
            });
        }

        current_leaves = next_leaves;
        leaf_index /= 2;
    }

    proof
}

fn up_layer(leaves: &[Leaf]) -> Vec<Leaf> {
    let mut new_layer: Vec<[u8; 32]> = vec![];

    for pair in leaves.chunks(2) {
        if pair.len() == 1 {
            new_layer.push(*pair.first().unwrap());
        } else {
            let mut buffer = [0u8; 32];
            hash_function(pair.first().unwrap(), &pair[1], &mut buffer);
            new_layer.push(buffer);
        }
    }

    new_layer
}
