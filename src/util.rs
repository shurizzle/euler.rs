pub struct Fibonacci {
    a: u128,
    b: u128,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a + self.b;
        self.a = self.b;
        self.b = tmp;
        Some(tmp)
    }
}

pub trait FindMax {
    type Item;

    fn find_max(self) -> Option<Self::Item>;
}

impl<I> FindMax for I
where
    I::Item: Ord,
    I: IntoIterator,
{
    type Item = <Self as IntoIterator>::Item;

    fn find_max(self) -> Option<Self::Item> {
        self.into_iter().reduce(|accum, item| accum.max(item))
    }
}

pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl<I> Palindrome for I
where
    I: ToString,
{
    fn is_palindrome(&self) -> bool {
        let s = self.to_string();
        let x = s.as_bytes();
        let limit = x.len() / 2;
        let mut i = 0;

        while i < limit {
            if x[i] != x[x.len() - i - 1] {
                return false;
            }
            i += 1;
        }

        true
    }
}

pub struct Matrix<T> {
    width: usize,
    height: usize,
    inner: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new_init(inner: Vec<Vec<T>>) -> Self {
        assert!(!inner.is_empty());
        let width = inner[0].len();
        assert!(width > 0);

        for x in inner.iter() {
            assert_eq!(x.len(), width);
        }

        Self {
            width,
            height: inner.len(),
            inner,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.inner.get(y).and_then(|line| line.get(x))
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Default> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let inner = (0..height)
            .map(|_| {
                let mut x = Vec::with_capacity(width);
                for _ in 0..width {
                    x.push(Default::default());
                }
                x
            })
            .collect();

        Self {
            width,
            height,
            inner,
        }
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new_fill(width: usize, height: usize, init: &T) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let inner = (0..height).map(|_| vec![init.clone(); width]).collect();

        Self {
            width,
            height,
            inner,
        }
    }
}
