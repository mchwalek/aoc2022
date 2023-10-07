pub trait Slideable {
    fn len(&self) -> usize;
    fn slice(&self, start: usize, end: usize) -> Self;
    fn sliding_window_iter(self, n: usize) -> SlidingWindow<Self> where Self: Sized {
        SlidingWindow::new(self, n)
    }
}

impl Slideable for &str {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        &self[start..end]
    }
}

impl<T> Slideable for &[T] {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        &self[start..end]
    }
}

pub struct SlidingWindow<T>
where
    T: Slideable
{
    data: T,
    n: usize,
    index: usize
}

impl<T> SlidingWindow<T>
where
    T: Slideable
{
    fn new(data: T, n: usize) -> Self {
        SlidingWindow { data, n, index: 0 }
    }
}

impl<T> Iterator for SlidingWindow<T>
where
    T: Slideable
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.n > self.data.len() {
            return None;
        }

        let window = Some(self.data.slice(self.index, self.index + self.n));
        self.index += 1;
        window
    }
}

#[cfg(test)]
mod tests {
    use crate::day6_part1::lib::*;

    #[test]
    fn creates_an_iterator_with_sliding_sequence_of_n_items() {
        let str = "abcd".to_string();
        let str_iter = str.sliding_window_iter(2);
        assert_eq!(vec!["ab", "bc", "cd"], str_iter.collect::<Vec<_>>());

        let arr = [1, 2, 3, 4];
        let arr_iter = arr.sliding_window_iter(2);
        assert_eq!(vec![[1, 2], [2, 3], [3, 4]], arr_iter.collect::<Vec<_>>());
    }
}