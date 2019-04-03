pub trait Conditional<FactorType> {
    fn num_frontals(&self) -> usize;
    fn num_parents(&self) -> usize;

    fn frontals<'a>(&self) -> Box<Iterator<Item = &'a FactorType> + 'a>;
    fn parents<'a>(&self) -> Box<Iterator<Item = &'a FactorType> + 'a>;
}
