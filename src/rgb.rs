use std::ops::Sub;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Rgb {
    #[must_use]
    pub const fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }
}

impl Sub for Rgb {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}
