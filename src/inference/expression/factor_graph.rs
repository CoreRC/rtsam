use crate::inference::factor::*;
use crate::inference::factor_graph::*;

pub struct ExpressionFactor {}

impl NonlinearFactor for ExpressionFactor {}

impl Factor for ExpressionFactor {
    fn keys(&mut self) -> &mut Vec<KeyType> {
        unimplemented!()
    }
}

impl SimpleFactorGraph<ExpressionFactor> {}
