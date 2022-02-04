use crate::inference::factor::*;
use crate::inference::factor_graph::*;

pub struct ExpressionFactor {}

impl NonlinearFactor for ExpressionFactor {}

impl Factor for ExpressionFactor {
    fn num_keys(&self) -> usize {
        unimplemented!()
    }

    fn key_at(&self, index: usize) -> KeyType {
        unimplemented!()
    }
}

impl SimpleFactorGraph<ExpressionFactor> {}
