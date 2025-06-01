use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Add<Output = T> + Copy + Default + PartialOrd,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let default = T::default();

        match (sides[0], sides[1], sides[2]) {
            (a, b, c) if a == default || b == default || c == default => None,
            (a, b, c) if a + b >= c && a + c >= b && b + c >= a => Some(Self { a, b, c }),
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
