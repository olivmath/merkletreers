use crate::Hash;

#[derive(PartialEq, Debug, Clone)]
pub enum Side {
    LEFT = 0,
    RIGHT = 1,
}
impl From<u8> for Side {
    fn from(num: u8) -> Self {
        match num {
            0 => Side::LEFT,
            1 => Side::RIGHT,
            _ => panic!("Invalid value for Side enum, must be either `0` or `1`"),
        }
    }
}

/// # 🍃 Leaf of Tree
#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    pub data: Hash,
    pub side: Side,
}
