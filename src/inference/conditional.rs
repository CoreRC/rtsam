pub trait Conditional<FactorType> {
    fn num_frontals(&self) -> usize;
    fn num_parents(&self) -> usize;

    fn frontals<'a>(&self) -> Box<dyn Iterator<Item = &'a u64> + 'a>;
    fn parents<'a>(&self) -> Box<dyn Iterator<Item = &'a u64> + 'a>;
}
