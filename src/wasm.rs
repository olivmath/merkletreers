use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::node::{Node, Side};
use crate::tree::MerkleTree;
use crate::utils::hash_it;
use crate::Leaf;

#[derive(Serialize, Deserialize)]
struct JsNode {
    data: Vec<u8>,
    side: String,
}

#[wasm_bindgen]
pub struct WasmMerkleTree {
    inner: MerkleTree,
}

#[wasm_bindgen]
impl WasmMerkleTree {
    #[wasm_bindgen(constructor)]
    pub fn new(leaves: JsValue) -> Result<WasmMerkleTree, JsError> {
        let raw_leaves: Vec<Vec<u8>> = serde_wasm_bindgen::from_value(leaves)
            .map_err(|e| JsError::new(&format!("Invalid leaves: {}", e)))?;

        let leaves: Vec<Leaf> = raw_leaves
            .iter()
            .map(|l| {
                let arr: [u8; 32] = l
                    .as_slice()
                    .try_into()
                    .expect("Each leaf must be exactly 32 bytes");
                arr
            })
            .collect();

        Ok(WasmMerkleTree {
            inner: MerkleTree::new(leaves),
        })
    }

    pub fn root(&self) -> Vec<u8> {
        self.inner.root.to_vec()
    }

    #[wasm_bindgen(js_name = makeProof)]
    pub fn make_proof(&self, leaf: &[u8]) -> Result<JsValue, JsError> {
        let leaf_arr: Leaf = leaf
            .try_into()
            .map_err(|_| JsError::new("Leaf must be exactly 32 bytes"))?;

        let proof = self.inner.make_proof(leaf_arr);
        let js_nodes: Vec<JsNode> = proof
            .into_iter()
            .map(|n| JsNode {
                data: n.data.to_vec(),
                side: match n.side {
                    Side::LEFT => "LEFT".to_string(),
                    Side::RIGHT => "RIGHT".to_string(),
                },
            })
            .collect();

        serde_wasm_bindgen::to_value(&js_nodes)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    #[wasm_bindgen(js_name = checkProof)]
    pub fn check_proof(&self, proof: JsValue, leaf: &[u8]) -> Result<Vec<u8>, JsError> {
        let leaf_arr: Leaf = leaf
            .try_into()
            .map_err(|_| JsError::new("Leaf must be exactly 32 bytes"))?;

        let js_nodes: Vec<JsNode> = serde_wasm_bindgen::from_value(proof)
            .map_err(|e| JsError::new(&format!("Invalid proof: {}", e)))?;

        let proof_nodes: Vec<Node> = js_nodes
            .into_iter()
            .map(|n| {
                let data: [u8; 32] = n
                    .data
                    .as_slice()
                    .try_into()
                    .expect("Proof node data must be 32 bytes");
                let side = match n.side.as_str() {
                    "LEFT" => Side::LEFT,
                    "RIGHT" => Side::RIGHT,
                    _ => panic!("Side must be LEFT or RIGHT"),
                };
                Node { data, side }
            })
            .collect();

        let root = self.inner.check_proof(proof_nodes, leaf_arr);
        Ok(root.to_vec())
    }
}

#[wasm_bindgen(js_name = hashLeaf)]
pub fn hash_leaf(data: &[u8]) -> Vec<u8> {
    let mut buffer = [0u8; 32];
    hash_it(data, &mut buffer);
    buffer.to_vec()
}
