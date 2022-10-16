use super::merkletree::Leaf;

/// # Verify if `x: int` is power of 2
/// - params `x: int`
/// - return `bool`

/// ```rust
/// use merkletreers::utils::is_power_2;
///
/// assert_eq!(is_power_2(2), true);
/// assert_eq!(is_power_2(3), false);
/// assert_eq!(is_power_2(16), true);
/// assert_eq!(is_power_2(900), false);
/// ```
pub fn is_power_2(number: u32) -> bool {
    let left: bool = number & (number - 1) == 0;
    let right: bool = number != 0;

    left && right
}

/// # Hash `data: str` using keccak256
/// - params `data: str`
/// - return `hexadecimal: str`
///
/// ```rust
///  use merkletreers::utils::to_keccak256;
///
/// assert_eq!(
///     to_keccak256("merkle".to_string()),
///     "326fe0d8a70ab934a7bf9d1323c6d87ee37bbe70079f82e72203b1e07c0c185c"
/// );
///
/// assert_eq!(
///     to_keccak256("tree".to_string()),
///     "b2510336c6497719adadc7ade198c988520f3349445f074dc729df0f3c2b12ad"
/// );
///
/// assert_eq!(
///     to_keccak256("bitcoin".to_string()),
///     "7dee6e1aa550de37364ec77e03e62ea56bf42037b8297280de9d844d88444e4d"
/// );
///
/// assert_eq!(
///     to_keccak256("ethereum".to_string()),
///     "541111248b45b7a8dc3f5579f630e74cb01456ea6ac067d3f4d793245a255155"
/// );
/// ```
pub fn to_keccak256(message: String) -> Leaf {
    use tiny_keccak::{Hasher, Keccak};

    let mut k256 = Keccak::v256();
    let mut result = [0; 32];

    k256.update(message.as_bytes());
    k256.finalize(&mut result);

    hex::encode(result)
}

/// # Half a `arr: Vec<T>` in a pairs
/// - params `arr: Vec<T>`
/// - return `Vec<T>: (Vec<T>, Vec<T>)
///
/// ```rust
/// use merkletreers::utils::half;
///
/// assert_eq!(
///     half(&vec![1,2,3,4]),
///     (vec![1, 2], vec![3, 4])
/// );
///
/// assert_eq!(
///     half(&vec!["a", "b", "c", "d", "e", "f"]),
///     (vec!["a", "b", "c"], vec!["d", "e", "f"])
/// );
/// ```
pub fn half<T: std::clone::Clone>(arr: &Vec<T>) -> (Vec<T>, Vec<T>) {
    let m = arr.len() / 2;
    let left = &arr[0..m];
    let right = &arr[m..];

    (left.to_vec(), right.to_vec())
}
