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
