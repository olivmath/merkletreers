use crate::hasher::Hashable;
use crate::merkle_proof_mixed::merkle_proof_mixed_tree;
use crate::merkle_root::merkle_root;
use crate::node::Node;
use crate::utils::is_power_of_two;
use crate::{Leaf, Proof};

pub fn merkle_proof<H: Hashable>(leaves: &[Leaf], leaf: Leaf, hasher: &H) -> Proof {
    let mut proof: Proof = Vec::new();

    if !is_power_of_two(leaves.len() as u32) {
        return merkle_proof_mixed_tree(leaves, leaf, hasher);
    }

    let mut current_leaves = leaves;
    while current_leaves.len() > 1 {
        let index = match current_leaves.iter().position(|x| x == &leaf) {
            Some(i) => i,
            None => panic!("Leaf does not exist in the tree"),
        };
        if current_leaves.len() == 2 {
            if index == 1 {
                // proof.push(Node(data = leaves[0], side = Side.LEFT))
                let left: [u8; 32] = current_leaves[0]
                    .as_slice()
                    .try_into()
                    .expect("Failed to convert LEFT to array");
                proof.push(Node {
                    data: left,
                    side: 0.into(),
                });
                break;
            } else {
                // proof.append(Node(data = leaves[1], side = Side.RIGHT))
                let right: [u8; 32] = current_leaves[1]
                    .as_slice()
                    .try_into()
                    .expect("Failed to convert RIGHT to array");
                proof.push(Node {
                    data: right,
                    side: 1.into(),
                });
                break;
            }
        }

        let half_size = current_leaves.len() / 2;

        // divide a lista em 2, left e right
        let (left, right) = current_leaves.split_at(half_size);

        // se o index estiver em left
        if index < half_size {
            // faça a merkle root de right
            // TODO: make root must be a async (using other thread)
            let right_root = merkle_root(right, hasher);

            // faça o node passando right e 1 (direita)
            // adicione o node na lista de prova
            proof.push(Node {
                data: right_root,
                side: 1.into(),
            });

            current_leaves = left;
        } else {
            // se o index estiver em right
            // faça a merkle root de left
            // TODO: make root must be a async (using other thread)
            let left_root = merkle_root(left, hasher);
            // faça o node passando left e 0 (esquerda)

            // adicione o node na lista de prova
            proof.push(Node {
                data: left_root,
                side: 0.into(),
            });

            current_leaves = right;
        }
    }

    proof.reverse();
    proof
}
