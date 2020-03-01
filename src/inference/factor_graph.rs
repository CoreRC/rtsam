extern crate alloc;
use crate::inference::Factor;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub trait FactorGraph<FactorType> {}

#[derive(Debug)]
pub struct SimpleFactorGraph<FactorType>
where
    FactorType: Factor,
{
    factors: Vec<Arc<FactorType>>,
}

impl<FactorType> FactorGraph<FactorType> for SimpleFactorGraph<FactorType> where FactorType: Factor {}

pub trait EliminateableFactorGraph<FactorType>: FactorGraph<FactorType>
where FactorType: Factor {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    #[derive(Debug)]
    struct TestFactor {
        inner: String,
        _keys: Vec<u64>,
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
        fn keys(&mut self) -> &mut Vec<crate::inference::factor::KeyType> {
            &mut self._keys
        }
    }

    impl SimpleFactorGraph<TestFactor> {
        pub fn test(&self) {
            for i in self.factors.iter() {
                format!("{:?}", i);
            }
        }
    }

    #[test]
    fn test_factor_graph_threaded() {
        let mut fg = SimpleFactorGraph::<TestFactor> {
            factors: Vec::<Arc<TestFactor>>::new(),
        };

        fg.factors.push(Arc::new(TestFactor {
            inner: "TETS".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TETS1".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TETS2".into(),
            ..Default::default()
        }));
        fg.factors.push(Arc::new(TestFactor {
            inner: "TETS3".into(),
            ..Default::default()
        }));

        let ptr = Arc::new(Mutex::new(fg));

        thread::spawn(move || {
            ptr.lock().unwrap().test();
        });
    }
}
