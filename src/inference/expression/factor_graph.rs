use crate::inference::factor::*;
use crate::inference::factor_graph::*;

#[derive(Debug, Clone)]
pub struct ExpressionFactor {}

impl NonlinearFactor for ExpressionFactor {}

impl Factor for ExpressionFactor {
    fn num_keys(&self) -> usize {
        unimplemented!()
    }

    fn key_at(&self, index: usize) -> Result<KeyType, std::io::Error> {
        unimplemented!()
    }
}

impl SimpleFactorGraph<ExpressionFactor> {}
