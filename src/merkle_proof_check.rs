use crate::utils2::{hash_function, Node};

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

#[cfg(test)]
mod tests {
    use super::merkle_proof_check;

    mod merkle_proof_check_leaves_even {}

    mod merkle_proof_check_leaves_odd {}

    mod merkle_proof_check_leaves_base_2 {
        use crate::utils2::{Node, Side};

        use super::merkle_proof_check;

        const SETUP_ROOT: [u8; 32] = [
            104, 32, 63, 144, 233, 208, 125, 197, 133, 146, 89, 215, 83, 110, 135, 166, 186, 157,
            52, 95, 37, 82, 181, 185, 222, 41, 153, 221, 206, 156, 225, 191,
        ];

        const SETUP_LEAF: [u8; 32] = [
            58, 194, 37, 22, 141, 245, 66, 18, 162, 92, 28, 1, 253, 53, 190, 191, 234, 64, 143,
            218, 194, 227, 29, 221, 111, 128, 164, 187, 249, 165, 241, 203,
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
            let result = merkle_proof_check(SETUP_PROOF.to_vec(), SETUP_LEAF);

            assert_eq!(result, SETUP_ROOT);
        }
    }
}
