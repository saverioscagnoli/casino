use std::collections::VecDeque;

use rand::distr::uniform::{SampleRange, SampleUniform};

pub fn rng<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    rand::random_range(range)
}

pub trait Pick<T> {
    fn pick(&self) -> &T;
    fn pick_mut(&mut self) -> &mut T;
}

impl<T> Pick<T> for Vec<T> {
    fn pick(&self) -> &T {
        &self[rng(0..self.len())]
    }

    fn pick_mut(&mut self) -> &mut T {
        let idx = rng(0..self.len());
        &mut self[idx]
    }
}

impl<T> Pick<T> for VecDeque<T> {
    fn pick(&self) -> &T {
        &self[rng(0..self.len())]
    }

    fn pick_mut(&mut self) -> &mut T {
        let idx = rng(0..self.len());
        &mut self[idx]
    }
}

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for Vec<T> {
    fn shuffle(&mut self) {
        let len = self.len();
        for i in 0..len {
            let j = rng(0..len);
            self.swap(i, j);
        }
    }
}

impl<T> Shuffle for VecDeque<T> {
    fn shuffle(&mut self) {
        let len = self.len();
        for i in 0..len {
            let j = rng(0..len);
            self.swap(i, j);
        }
    }
}
