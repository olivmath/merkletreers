/// Investigation of Issue #11: Duplicate leaves behavior
///
/// This test investigates what happens when there are duplicate leaves in the tree.
/// Questions to answer:
/// 1. How are the proofs generated?
/// 2. How is the root generated?
/// 3. What happens when we try to make a proof for a duplicate leaf?

use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_leaves_root_generation() {
        // Example from issue #11: ["m","e","r","k","l","e","t","r","e","e","r","s"]
        // Note: "e" appears 4 times, "r" appears 3 times
        let data = ["m", "e", "r", "k", "l", "e", "t", "r", "e", "e", "r", "s"];

        let leaves = data
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        // Create tree with duplicate leaves
        let tree = MerkleTree::new(leaves.clone());

        // Root should be generated successfully
        println!("Root with duplicates: {:?}", tree.root);
        assert_ne!(tree.root, [0u8; 32], "Root should not be zero");

        // Let's also test with a smaller example for clarity
        let simple_duplicates = ["a", "b", "a", "c"];
        let simple_leaves = simple_duplicates
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let simple_tree = MerkleTree::new(simple_leaves);
        println!("Simple root with duplicates: {:?}", simple_tree.root);
        assert_ne!(simple_tree.root, [0u8; 32], "Simple root should not be zero");
    }

    #[test]
    fn test_duplicate_leaves_proof_generation() {
        // When we have duplicates, the proof is generated for the FIRST occurrence
        let data = ["a", "b", "a", "c"];

        let leaves = data
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let tree = MerkleTree::new(leaves.clone());

        // Hash of "a"
        let mut leaf_a = [0u8; 32];
        hash_it("a".as_bytes(), &mut leaf_a);

        // Make proof for "a" - this will find the FIRST occurrence at index 0
        let proof = tree.make_proof(leaf_a);

        println!("Proof for duplicate 'a': {:?}", proof);

        // Save root before check_proof consumes tree
        let expected_root = tree.root;

        // Verify the proof
        let computed_root = tree.check_proof(proof, leaf_a);
        println!("Computed root: {:?}", computed_root);
        println!("Expected root: {:?}", expected_root);

        // The proof is valid because it proves the FIRST occurrence
        assert_eq!(
            computed_root, expected_root,
            "Proof verification should succeed for first occurrence"
        );

        // Important note: We cannot distinguish between different positions of the same leaf value
        // The proof will always be for the FIRST occurrence found by iter().position()
    }

    #[test]
    fn test_duplicate_leaves_multiple_proofs() {
        // Test what happens when we try to make proofs for all instances of a duplicate
        let data = ["a", "b", "c", "a", "d", "a"];

        let leaves = data
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let tree = MerkleTree::new(leaves.clone());

        // Hash of "a"
        let mut leaf_a = [0u8; 32];
        hash_it("a".as_bytes(), &mut leaf_a);

        // Try to make proof for "a"
        // The current implementation will find the FIRST occurrence at index 0
        let proof = tree.make_proof(leaf_a);

        println!("Number of proof nodes: {}", proof.len());
        println!("Proof for first 'a': {:?}", proof);

        // Save root before check_proof consumes tree
        let expected_root = tree.root;

        // Verify the proof
        let computed_root = tree.check_proof(proof.clone(), leaf_a);
        println!("Computed root: {:?}", computed_root);
        println!("Expected root: {:?}", expected_root);

        assert_eq!(
            computed_root, expected_root,
            "Proof verification should succeed for first occurrence"
        );

        // The issue is: we cannot distinguish between different positions of the same leaf value
        // The proof will always be for the FIRST occurrence
    }

    #[test]
    fn test_unique_leaves_vs_duplicate_leaves() {
        // Compare behavior with unique vs duplicate leaves

        // Unique leaves
        let unique_data = ["a", "b", "c", "d"];
        let unique_leaves = unique_data
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let unique_tree = MerkleTree::new(unique_leaves.clone());

        // Duplicate leaves (same data but "a" appears twice)
        let duplicate_data = ["a", "b", "a", "d"];
        let duplicate_leaves = duplicate_data
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let duplicate_tree = MerkleTree::new(duplicate_leaves.clone());

        // Roots should be different because the tree structure is different
        assert_ne!(
            unique_tree.root, duplicate_tree.root,
            "Different leaf arrangements should produce different roots"
        );

        println!("Unique tree root: {:?}", unique_tree.root);
        println!("Duplicate tree root: {:?}", duplicate_tree.root);
    }

    #[test]
    fn test_issue_11_exact_example() {
        // Exact example from issue #11
        let data = ["m", "e", "r", "k", "l", "e", "t", "r", "e", "e", "r", "s"];

        let leaves = data
            .iter()
            .map(|d| {
                let mut buffer = [0u8; 32];
                hash_it(d.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        println!("Number of leaves: {}", leaves.len());

        let tree = MerkleTree::new(leaves.clone());

        println!("Root: {:?}", tree.root);
        let expected_root = tree.root;

        // Try to make proof for each unique letter
        let unique_letters = ["m", "e", "r", "k", "l", "t", "s"];

        for letter in unique_letters.iter() {
            let mut leaf = [0u8; 32];
            hash_it(letter.as_bytes(), &mut leaf);

            let tree_for_proof = MerkleTree::new(leaves.clone());
            let proof = tree_for_proof.make_proof(leaf);

            let tree_for_check = MerkleTree::new(leaves.clone());
            let computed_root = tree_for_check.check_proof(proof.clone(), leaf);

            println!(
                "Letter '{}': proof length = {}, verification = {}",
                letter,
                proof.len(),
                computed_root == expected_root
            );

            assert_eq!(
                computed_root, expected_root,
                "Proof verification should succeed for '{}'",
                letter
            );
        }
    }
}
