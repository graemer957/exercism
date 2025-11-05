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
    pub fn build(mut sides: [T; 3]) -> Option<Self> {
        if sides.iter().any(|&side| side == T::default()) {
            return None;
        }

        // By sorting we can simplify the `is_` functions
        sides.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
        let [a, b, c] = sides;
        if a + b < c {
            return None;
        }

        Some(Self { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c
    }
}
