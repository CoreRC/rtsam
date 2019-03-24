#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod core;
pub mod geometry;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
