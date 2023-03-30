extern crate alloc;

pub type KeyType = u64;

pub struct KeyIterator<'a> {
    owner: &'a dyn Factor,
    index: usize,
}

impl<'a> Iterator for KeyIterator<'a> {
    type Item = KeyType;

    fn next(&mut self) -> Option<Self::Item> {
        self.owner.key_at(self.index - 1).ok()
    }
}

pub trait Factor: std::fmt::Debug {
    fn num_keys(&self) -> usize;
    fn key_at(&self, index: usize) -> Result<KeyType, std::io::Error>;
}

pub trait NonlinearFactor: Factor {}
