use node::Node;

pub mod merkle_proof;
pub mod merkle_proof_check;
pub mod merkle_proof_mixed;
pub mod merkle_root;
pub mod node;
pub mod tree;
pub mod utils;

pub type Proof = Vec<Node>;
pub type Hash = [u8; 32];
pub type Leaf = [u8; 32];
pub type Root = [u8; 32];
