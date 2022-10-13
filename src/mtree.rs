use crate::utils::is_power_2;

pub type Leaf = String;
pub type Root = String;

/// # 🍃 Leaf of Tree
pub struct Node {
    pub right: Option<Leaf>,
    pub left: Option<Leaf>,
}

/// # 🌳 Merkle Tree
/// - You can pass raw data
/// - They will hashed by `keccak-256`
#[derive(Debug)]
pub struct MerkleTree {
    pub leafs: Vec<Leaf>,
    pub root: Root,
}

impl MerkleTree {
    pub fn new(leafs: Vec<&str>) -> Self {
        match is_power_2(leafs.len() as u32) {
            true => MerkleTree {
                leafs: leafs
                    .iter()
                    .map(|i| MerkleTree::to_keccak256(i.to_string()))
                    .collect(),
                root: MerkleTree::make_root(
                    leafs
                        .iter()
                        .map(|i| MerkleTree::to_keccak256(i.to_string()))
                        .collect(),
                ),
            },
            false => {
                panic!("{} is not a power-2 leaf", leafs.len())
            }
        }
    }

    fn make_root(leafs: Vec<Leaf>) -> String {
        if leafs.len() == 1 {
            return leafs[0].to_owned();
        } else {
            let mut new_leafs: Vec<String> = vec![];

            for i in leafs.chunks(2) {
                new_leafs.push(MerkleTree::to_keccak256(i.concat()));
            }
            return MerkleTree::make_root(new_leafs);
        }
    }
}

impl MerkleTree {
    pub fn to_keccak256(message: String) -> Leaf {
        use tiny_keccak::{Hasher, Keccak};

        let mut k256 = Keccak::v256();
        let mut result = [0; 32];

        k256.update(message.as_bytes());
        k256.finalize(&mut result);

        hex::encode(result)
    }
}
