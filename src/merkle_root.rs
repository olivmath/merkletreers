use crate::utils2::hash_function;

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

#[cfg(test)]
mod tests {
    use super::merkle_root;

    mod merkle_root_leaves_even {}

    mod merkle_root_leaves_odd {}

    mod merkle_root_leaves_base_2 {
        use super::merkle_root;

        const SETUP_ROOT: [u8; 32] = [
            104, 32, 63, 144, 233, 208, 125, 197, 133, 146, 89, 215, 83, 110, 135, 166, 186, 157,
            52, 95, 37, 82, 181, 185, 222, 41, 153, 221, 206, 156, 225, 191,
        ];

        const SETUP_LEAVES: [[u8; 32]; 4] = [
            // keccak256(a)
            [
                58, 194, 37, 22, 141, 245, 66, 18, 162, 92, 28, 1, 253, 53, 190, 191, 234, 64, 143,
                218, 194, 227, 29, 221, 111, 128, 164, 187, 249, 165, 241, 203,
            ],
            // keccak256(b)
            [
                181, 85, 61, 227, 21, 224, 237, 245, 4, 217, 21, 10, 248, 45, 175, 165, 196, 102,
                127, 166, 24, 237, 10, 111, 25, 198, 155, 65, 22, 108, 85, 16,
            ],
            // keccak256(c)
            [
                11, 66, 182, 57, 60, 31, 83, 6, 15, 227, 221, 191, 205, 122, 173, 204, 168, 148,
                70, 90, 90, 67, 143, 105, 200, 125, 121, 11, 34, 153, 185, 178,
            ],
            // keccak256(d)
            [
                241, 145, 142, 133, 98, 35, 110, 177, 122, 220, 133, 2, 51, 47, 76, 156, 130, 188,
                20, 225, 155, 252, 10, 161, 10, 182, 116, 255, 117, 179, 210, 243,
            ],
        ];

        #[test]
        fn test_make_root() {
            let result = merkle_root(&SETUP_LEAVES);

            assert_eq!(result, SETUP_ROOT);
        }
    }
}
