use super::super::utils::{half, is_power_2, to_keccak256};
use super::{node::Node, Leaf, Root};

/// # 🌳 Merkle Tree
/// - You can pass raw data
/// - They will hashed by `keccak-256`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MerkleTree {
    pub leafs: Vec<Leaf>,
    pub root: Root,
}

impl MerkleTree {
    pub fn new(leafs: Vec<&str>) -> Self {
        match is_power_2(leafs.len() as u32) {
            true => MerkleTree {
                leafs: leafs.iter().map(|i| to_keccak256(i.to_string())).collect(),
                root: MerkleTree::make_root(
                    leafs.iter().map(|i| to_keccak256(i.to_string())).collect(),
                ),
            },
            false => {
                panic!("{} is not a power-2 leaf", leafs.len())
            }
        }
    }

    /// # Merkle Root of `x: list[str]` using keccak256
    /// - params `x: lsit[str]`
    /// - return `hexadecimal: list[str]`
    ///
    /// ```rust
    /// use merkletreers::merkletree::tree::MerkleTree;
    ///
    ///  let tree1 = MerkleTree::new(vec!["a", "b", "c", "d"]);
    ///  let tree2 = MerkleTree::new(vec!["a", "b"]);
    ///
    ///  assert_eq!(
    ///     tree1.root,
    ///     "115cbb4775ed495f3d954dfa47164359a97762b40059d9502895def16eed609c"
    /// );
    ///
    ///  assert_eq!(
    ///     tree2.root,
    ///     "414e3a845393ef6d68973ddbf5bd85ff524443cf0e06a361624f3d51b879ec1c"
    /// );
    /// ```
    fn make_root(leafs: Vec<Leaf>) -> String {
        if leafs.len() == 1 {
            return leafs[0].to_owned();
        } else {
            let mut new_leafs: Vec<String> = vec![];

            for i in leafs.chunks(2) {
                new_leafs.push(to_keccak256(i.concat()));
            }
            return MerkleTree::make_root(new_leafs);
        }
    }

    /// # Make a proof
    /// - if the `leaf` index is less than half the size of the `leafs`
    /// list then the right side must reach root and vice versa
    fn make_proof(leafs: Vec<Leaf>, leaf: Leaf, proof: &mut Vec<Node>) -> Vec<Node> {
        if leafs.len() == 2 {
            proof.push(Node::right(&leafs[1]));
            proof.push(Node::left(&leafs[0]));
            return proof.to_vec();
        } else {
            let index = match leafs.iter().position(|_leaf| _leaf == &leaf) {
                Some(index) => index,
                None => panic!("leaf: {} does not exist in the tree: {:?}", leaf, leafs),
            };

            let (left, right) = half(&leafs);

            if index < leafs.len() / 2 {
                proof.push(Node::right(&MerkleTree::make_root(right)));
                return MerkleTree::make_proof(left, leaf, proof);
            } else {
                proof.push(Node::left(&MerkleTree::make_root(left)));
                return MerkleTree::make_proof(right, leaf, proof);
            }
        }
    }
}

impl MerkleTree {
    pub fn proof(self, leaf: &str) -> Vec<Node> {
        let mut proof = MerkleTree::make_proof(self.leafs, to_keccak256(leaf.to_string()), &mut vec![]);
        proof.reverse();
        return proof
    }
}
