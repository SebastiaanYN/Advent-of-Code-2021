#[derive(Clone, Debug)]
pub struct Transpose<I> {
    iters: I,
    idx: usize,
}

impl<I> Transpose<I> {
    pub fn new(iters: I) -> Self {
        Transpose { iters, idx: 0 }
    }
}

impl<I1, I2> Iterator for Transpose<I2>
where
    I1: Iterator,
    I2: Iterator<Item = I1> + Clone,
{
    type Item = Vec<I1::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self
            .iters
            .clone()
            .map(|mut iter| iter.nth(self.idx))
            .collect();

        self.idx += 1;

        res
    }
}

pub trait TransposeImpl: Iterator {
    fn transpose<I1: Iterator>(self) -> Transpose<Self>
    where
        Self: Sized,
        Self: Iterator<Item = I1>,
    {
        Transpose::new(self)
    }
}

impl<T: ?Sized> TransposeImpl for T where T: Iterator {}
