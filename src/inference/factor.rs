extern crate alloc;


pub type KeyType = u64;

pub struct KeyIterator<'a> {
    owner: &'a dyn Factor,
    index: usize,
}

impl<'a> Iterator for KeyIterator<'a> {
    type Item = KeyType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.owner.num_keys() {
            return None;
        }

        self.index += 1;
        Some(self.owner.key_at(self.index - 1))
    }
}

pub trait Factor {
    fn num_keys(&self) -> usize;
    fn key_at(&self, index: usize) -> KeyType;
}

pub trait NonlinearFactor: Factor {}
