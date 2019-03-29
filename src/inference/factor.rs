extern crate alloc;
use alloc::vec::Vec;

pub type KeyType = u64;

pub trait Factor {
    fn keys(&mut self) -> &mut Vec<KeyType>;
}
