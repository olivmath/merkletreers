#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use merkletreers::wasm::{hash_leaf, WasmMerkleTree};

fn make_leaves(count: usize) -> Vec<Vec<u8>> {
    (0..count)
        .map(|i| {
            let input = vec![i as u8];
            let hashed = hash_leaf(&input);
            hashed
        })
        .collect()
}

#[wasm_bindgen_test]
fn test_build_tree_5_leaves_and_verify_root() {
    let leaves = make_leaves(5);
    let js_leaves = serde_wasm_bindgen::to_value(&leaves).unwrap();

    let tree = WasmMerkleTree::new(js_leaves).unwrap();
    let root = tree.root();

    assert_eq!(root.len(), 32);

    let js_leaves2 = serde_wasm_bindgen::to_value(&leaves).unwrap();
    let tree2 = WasmMerkleTree::new(js_leaves2).unwrap();
    assert_eq!(tree2.root(), root);
}

#[wasm_bindgen_test]
fn test_make_and_check_proof() {
    let leaves = make_leaves(5);
    let target_leaf = leaves[2].clone();
    let js_leaves = serde_wasm_bindgen::to_value(&leaves).unwrap();

    let tree = WasmMerkleTree::new(js_leaves).unwrap();
    let root = tree.root();

    let proof = tree.make_proof(&target_leaf).unwrap();
    let computed_root = tree.check_proof(proof, &target_leaf).unwrap();

    assert_eq!(computed_root, root);
}

#[wasm_bindgen_test]
fn test_invalid_proof_returns_different_root() {
    let leaves = make_leaves(5);
    let target_leaf = leaves[2].clone();
    let wrong_leaf = leaves[0].clone();
    let js_leaves = serde_wasm_bindgen::to_value(&leaves).unwrap();

    let tree = WasmMerkleTree::new(js_leaves).unwrap();
    let root = tree.root();

    let proof = tree.make_proof(&target_leaf).unwrap();
    let computed_root = tree.check_proof(proof, &wrong_leaf).unwrap();

    assert_ne!(computed_root, root);
}
