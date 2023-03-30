extern crate alloc;

use crate::inference::Factor;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub struct FactorIterator<'a, FactorGraphType>
where
    Self: Iterator,
{
    owner: &'a FactorGraphType,
    index: usize,
}

pub trait FactorGraph
where
    Self: Sized,
{
    type FactorType: ?Sized;

    fn new() -> Self;

    fn insert(&mut self, f: Self::FactorType)
    where
        Self::FactorType: Sized;

    fn merge(&mut self, other: &mut Self);

    fn insert_shared(&mut self, f: Arc<Self::FactorType>);

    fn iter(&self) -> Box<dyn Iterator<Item = &Self::FactorType> + '_>
    where
        Self::FactorType: Sized;

    fn size(&self) -> usize;
}

#[derive(Debug)]
pub struct SimpleFactorGraph<FactorType: ?Sized>
where
    FactorType: Factor,
{
    pub factors: Vec<Arc<FactorType>>,
}

impl<FACTOR: ?Sized> FactorGraph for SimpleFactorGraph<FACTOR>
where
    FACTOR: Factor,
{
    type FactorType = FACTOR;

    fn new() -> Self {
        Self { factors: vec![] }
    }

    fn insert(&mut self, f: Self::FactorType)
    where
        Self::FactorType: Sized,
    {
        self.factors.push(Arc::new(f));
    }

    fn merge(&mut self, other: &mut Self) {
        self.factors.append(&mut other.factors);
    }

    fn insert_shared(&mut self, f: Arc<Self::FactorType>) {
        self.factors.push(f);
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Self::FactorType> + '_>
    where
        Self::FactorType: Sized,
    {
        Box::new(FactorIterator {
            owner: self,
            index: 0,
        })
    }

    fn size(&self) -> usize {
        self.factors.len()
    }
}

impl<'a, FACTOR> Iterator for FactorIterator<'a, SimpleFactorGraph<FACTOR>>
where
    FACTOR: Factor,
{
    type Item = &'a FACTOR;

    fn next(&mut self) -> Option<Self::Item> {
        match self.owner.factors.get(self.index) {
            Some(a) => {
                self.index += 1;
                Some(&(*a))
            }
            None => None,
        }
    }
}

pub trait EliminateableFactorGraph<FactorType>: FactorGraph
where
    FactorType: Factor,
{
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::inference::factor::KeyType;
    use alloc::sync::Arc;
    use std::io::ErrorKind;
    use std::sync::Mutex;
    use std::thread;

    #[derive(Debug)]
    pub struct TestFactor {
        pub inner: String,
        pub _keys: Vec<u64>,
    }

    impl Default for TestFactor {
        fn default() -> TestFactor {
            TestFactor {
                inner: "".into(),
                _keys: Vec::new(),
            }
        }
    }

    impl Factor for TestFactor {
        fn num_keys(&self) -> usize {
            self._keys.len()
        }

        fn key_at(&self, index: usize) -> Result<KeyType, std::io::Error> {
            self._keys
                .get(index)
                .cloned()
                .ok_or(std::io::Error::new(ErrorKind::OutOfMemory, "Range"))
        }
    }

    impl SimpleFactorGraph<TestFactor> {
        pub fn test(&self) -> String {
            let mut res = String::new();
            for i in self.factors.iter() {
                res += format!("Inside thread: {:?}\n", i).as_str();
            }
            res
        }
    }

    #[test]
    fn test_factor_graph_threaded() {
        let mut fg = SimpleFactorGraph::<TestFactor> {
            factors: Vec::<Arc<TestFactor>>::new(),
        };

        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST0".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST1".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST2".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST3".into(),
            ..Default::default()
        }));

        // test iterator
        for i in fg.iter() {
            println!("{}", i.inner);
        }

        let ptr = Arc::new(Mutex::new(fg));

        {
            let ptr = ptr.clone();
            let handle = thread::spawn(move || {
                return ptr.lock().unwrap().test();
            });
            println!("{}", handle.join().unwrap());
        }
    }

    #[test]
    fn test_trait_object() {
        let mut fg = SimpleFactorGraph::<dyn Factor> { factors: vec![] };

        fg.insert_shared(Arc::new(TestFactor {
            inner: "TEST0".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST1".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST2".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TEST3".into(),
            ..Default::default()
        }));

        println!("{:#?}", fg);
    }
}
