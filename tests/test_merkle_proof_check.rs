use merkletreers::hasher::Keccak256Hasher;
use merkletreers::merkle_proof_check::merkle_proof_check;
use merkletreers::node::{Node, Side};
use merkletreers::tree::MerkleTree;
use merkletreers::{Leaf, Root};

#[cfg(test)]
mod tests {
    use super::*;

    mod merkle_proof_check_leaves_even {
        use super::*;

        const SETUP_ROOT: Root = [
            144, 18, 241, 225, 138, 135, 121, 13, 46, 1, 250, 172, 231, 90, 170, 202, 56, 229, 61,
            244, 55, 205, 206, 44, 5, 82, 70, 77, 218, 74, 244, 156,
        ];

        const SETUP_LEAF: Leaf = [
            168, 152, 44, 137, 216, 9, 135, 251, 154, 81, 14, 37, 152, 30, 233, 23, 2, 6, 190, 33,
            175, 60, 142, 14, 179, 18, 239, 29, 51, 130, 231, 97,
        ];

        const SETUP_PROOF: [Node; 2] = [
            Node {
                data: [
                    209, 232, 174, 183, 149, 0, 73, 110, 243, 220, 46, 87, 186, 116, 106, 131, 21,
                    208, 72, 183, 166, 100, 162, 191, 148, 141, 180, 250, 145, 150, 4, 131,
                ],
                side: Side::RIGHT,
            },
            Node {
                data: [
                    104, 32, 63, 144, 233, 208, 125, 197, 133, 146, 89, 215, 83, 110, 135, 166,
                    186, 157, 52, 95, 37, 82, 181, 185, 222, 41, 153, 221, 206, 156, 225, 191,
                ],
                side: Side::LEFT,
            },
        ];

        #[test]
        fn merkle_proof_check_leaves_even_make_proof() {
            let result = merkle_proof_check(SETUP_PROOF.to_vec(), SETUP_LEAF, &Keccak256Hasher);

            assert_eq!(result, SETUP_ROOT);
        }
    }

    mod merkle_proof_check_leaves_odd {
        use super::*;

        const SETUP_ROOT: Root = [
            29, 208, 210, 166, 174, 70, 109, 102, 92, 178, 110, 26, 49, 240, 124, 87, 174, 93, 247,
            210, 188, 85, 156, 213, 130, 109, 65, 123, 233, 20, 26, 93,
        ];

        const SETUP_LEAF: Leaf = [
            241, 145, 142, 133, 98, 35, 110, 177, 122, 220, 133, 2, 51, 47, 76, 156, 130, 188, 20,
            225, 155, 252, 10, 161, 10, 182, 116, 255, 117, 179, 210, 243,
        ];

        const SETUP_PROOF: [Node; 3] = [
            Node {
                data: [
                    11, 66, 182, 57, 60, 31, 83, 6, 15, 227, 221, 191, 205, 122, 173, 204, 168,
                    148, 70, 90, 90, 67, 143, 105, 200, 125, 121, 11, 34, 153, 185, 178,
                ],
                side: Side::LEFT,
            },
            Node {
                data: [
                    128, 91, 33, 216, 70, 177, 137, 239, 174, 176, 55, 125, 107, 176, 210, 1, 179,
                    135, 42, 54, 62, 96, 124, 37, 8, 143, 2, 91, 12, 106, 225, 248,
                ],
                side: Side::LEFT,
            },
            Node {
                data: [
                    168, 152, 44, 137, 216, 9, 135, 251, 154, 81, 14, 37, 152, 30, 233, 23, 2, 6,
                    190, 33, 175, 60, 142, 14, 179, 18, 239, 29, 51, 130, 231, 97,
                ],
                side: Side::RIGHT,
            },
        ];

        #[test]
        fn merkle_proof_check_leaves_odd_make_proof() {
            let result = merkle_proof_check(SETUP_PROOF.to_vec(), SETUP_LEAF, &Keccak256Hasher);

            assert_eq!(result, SETUP_ROOT);
        }
    }

    mod merkle_proof_check_leaves_base_2 {
        use super::*;

        const SETUP_ROOT: Root = [
            104, 32, 63, 144, 233, 208, 125, 197, 133, 146, 89, 215, 83, 110, 135, 166, 186, 157,
            52, 95, 37, 82, 181, 185, 222, 41, 153, 221, 206, 156, 225, 191,
        ];

        const SETUP_LEAF: Leaf = [
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
        fn merkle_proof_check_leaves_base_2_make_proof() {
            let result = merkle_proof_check(SETUP_PROOF.to_vec(), SETUP_LEAF, &Keccak256Hasher);

            assert_eq!(result, SETUP_ROOT);
        }
    }

    mod verify_proof_static {
        use super::*;

        const SETUP_ROOT: Root = [
            144, 18, 241, 225, 138, 135, 121, 13, 46, 1, 250, 172, 231, 90, 170, 202, 56, 229, 61,
            244, 55, 205, 206, 44, 5, 82, 70, 77, 218, 74, 244, 156,
        ];

        const SETUP_LEAF: Leaf = [
            168, 152, 44, 137, 216, 9, 135, 251, 154, 81, 14, 37, 152, 30, 233, 23, 2, 6, 190, 33,
            175, 60, 142, 14, 179, 18, 239, 29, 51, 130, 231, 97,
        ];

        const SETUP_PROOF: [Node; 2] = [
            Node {
                data: [
                    209, 232, 174, 183, 149, 0, 73, 110, 243, 220, 46, 87, 186, 116, 106, 131, 21,
                    208, 72, 183, 166, 100, 162, 191, 148, 141, 180, 250, 145, 150, 4, 131,
                ],
                side: Side::RIGHT,
            },
            Node {
                data: [
                    104, 32, 63, 144, 233, 208, 125, 197, 133, 146, 89, 215, 83, 110, 135, 166,
                    186, 157, 52, 95, 37, 82, 181, 185, 222, 41, 153, 221, 206, 156, 225, 191,
                ],
                side: Side::LEFT,
            },
        ];

        const WRONG_ROOT: Root = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        #[test]
        fn verify_proof_returns_true_for_valid_proof() {
            let result = MerkleTree::verify_proof(SETUP_PROOF.to_vec(), SETUP_LEAF, SETUP_ROOT);
            assert!(result);
        }

        #[test]
        fn verify_proof_returns_false_for_invalid_root() {
            let result = MerkleTree::verify_proof(SETUP_PROOF.to_vec(), SETUP_LEAF, WRONG_ROOT);
            assert!(!result);
        }

        #[test]
        fn verify_proof_returns_false_for_wrong_leaf() {
            let wrong_leaf: Leaf = [0u8; 32];
            let result = MerkleTree::verify_proof(SETUP_PROOF.to_vec(), wrong_leaf, SETUP_ROOT);
            assert!(!result);
        }

        #[test]
        fn verify_proof_with_hasher_works() {
            let result = MerkleTree::verify_proof_with_hasher(
                SETUP_PROOF.to_vec(),
                SETUP_LEAF,
                SETUP_ROOT,
                &Keccak256Hasher,
            );
            assert!(result);
        }
    }
}
