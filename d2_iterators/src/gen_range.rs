//whole source https://www.udemy.com/course/rust-programming-recipes/learn/lecture/16730102#overview

use std::ops::AddAssign;
pub trait Rangeable: AddAssign + PartialOrd + Copy {}

impl <T:AddAssign + PartialOrd + Copy> Rangeable for T{}

pub struct GenRangeIterator<T:Rangeable> {
    curr: T,
    stop: T,
    step: T,
}

impl<T: Rangeable> GenRangeIterator<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        GenRangeIterator {
            curr: start,
            stop,
            step,
        }
    }
}


//has to be implemented
impl<T: Rangeable> Iterator for GenRangeIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut m = 0.0;
        let it = GenRangeIterator::new(5.0, 12., 2.5);
        for s in it {
            m += s;
        }
        assert_eq!(m, 5.0 + 7.5 + 10.0)
    }

    #[test]
    fn filter() {
        //filter() creates and returns a new iterator
        let v: i32 = GenRangeIterator::new(3, 13, 3).filter(|x| x % 2 == 0).sum();
        assert_eq!(v, 6 + 12);
    }
}
