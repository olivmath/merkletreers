use merkletreers::utils::{half, is_power_2, to_keccak256};

#[test]
fn test_is_power_2() {
    assert_eq!(is_power_2(2), true);
    assert_eq!(is_power_2(3), false);
    assert_eq!(is_power_2(16), true);
    assert_eq!(is_power_2(900), false);
}

#[test]
fn test_to_keccak() {
    assert_eq!(
        to_keccak256("merkle".to_string()),
        "326fe0d8a70ab934a7bf9d1323c6d87ee37bbe70079f82e72203b1e07c0c185c"
    );
    assert_eq!(
        to_keccak256("tree".to_string()),
        "b2510336c6497719adadc7ade198c988520f3349445f074dc729df0f3c2b12ad"
    );
    assert_eq!(
        to_keccak256("bitcoin".to_string()),
        "7dee6e1aa550de37364ec77e03e62ea56bf42037b8297280de9d844d88444e4d"
    );
    assert_eq!(
        to_keccak256("ethereum".to_string()),
        "541111248b45b7a8dc3f5579f630e74cb01456ea6ac067d3f4d793245a255155"
    );
}

#[test]
fn test_half() {
    assert_eq!(half(&vec![1, 2, 3, 4]), (vec![1, 2], vec![3, 4]));
    assert_eq!(
        half(&vec!["a", "b", "c", "d", "e", "f"]),
        (vec!["a", "b", "c"], vec!["d", "e", "f"])
    );
}
