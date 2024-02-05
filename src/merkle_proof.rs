use crate::{merkle_root::merkle_root, utils::Node};

pub fn merkle_proof(leaves: &[[u8; 32]], leaf: [u8; 32]) -> Vec<Node> {
    let mut proof: Vec<Node> = Vec::new();

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
            let right_root = merkle_root(right);

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
            let left_root = merkle_root(left);
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

#[cfg(test)]
mod tests {
    use super::merkle_proof;

    mod merkle_proof_leaves_even {}

    mod merkle_proof_leaves_odd {}

    mod merkle_proof_leaves_base_2 {

        use crate::utils::{Node, Side};

        use super::merkle_proof;

        const SETUP_LEAF: [u8; 32] = [
            58, 194, 37, 22, 141, 245, 66, 18, 162, 92, 28, 1, 253, 53, 190, 191, 234, 64, 143,
            218, 194, 227, 29, 221, 111, 128, 164, 187, 249, 165, 241, 203,
        ];

        const SETUP_LEAVES: [[u8; 32]; 4] = [
            // keccak(a)
            [
                58, 194, 37, 22, 141, 245, 66, 18, 162, 92, 28, 1, 253, 53, 190, 191, 234, 64, 143,
                218, 194, 227, 29, 221, 111, 128, 164, 187, 249, 165, 241, 203,
            ],
            // keccak(b)
            [
                181, 85, 61, 227, 21, 224, 237, 245, 4, 217, 21, 10, 248, 45, 175, 165, 196, 102,
                127, 166, 24, 237, 10, 111, 25, 198, 155, 65, 22, 108, 85, 16,
            ],
            // keccak(c)
            [
                11, 66, 182, 57, 60, 31, 83, 6, 15, 227, 221, 191, 205, 122, 173, 204, 168, 148,
                70, 90, 90, 67, 143, 105, 200, 125, 121, 11, 34, 153, 185, 178,
            ],
            // keccak(d)
            [
                241, 145, 142, 133, 98, 35, 110, 177, 122, 220, 133, 2, 51, 47, 76, 156, 130, 188,
                20, 225, 155, 252, 10, 161, 10, 182, 116, 255, 117, 179, 210, 243,
            ],
        ];

        const SETUP_PROOF: [Node; 2] = [
            Node {
                data: [
                    181, 85, 61, 227, 21, 224, 237, 245, 4, 217, 21, 10, 248, 45, 175, 165, 196,
                    102, 127, 166, 24, 237, 10, 111, 25, 198, 155, 65, 22, 108, 85, 16,
                ],
                side: Side::RIGHT,
            },
            Node {
                data: [
                    210, 83, 165, 45, 76, 176, 13, 226, 137, 94, 133, 242, 82, 158, 41, 118, 230,
                    170, 170, 92, 24, 16, 107, 104, 171, 102, 129, 62, 20, 65, 86, 105,
                ],
                side: Side::RIGHT,
            },
        ];

        #[test]
        fn make_proof() {
            let result = merkle_proof(&SETUP_LEAVES, SETUP_LEAF);

            assert_eq!(result, SETUP_PROOF.to_vec());
        }
    }
}
