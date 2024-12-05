use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, PartialOrd, Hash, Eq, Clone, Copy, Debug)]
pub struct Vector2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Add for Vector2d<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector2d<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Vector2d<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Vector2d<T>
where
    T: Mul<Output = T> + Copy + Clone,
{
    pub fn scalar_mul(self, factor: T) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl<T> Vector2d<T> {
    pub fn try_components_into<U>(self) -> Result<Vector2d<U>, <T as TryInto<U>>::Error>
    where
        T: TryInto<U>,
    {
        let x: U = self.x.try_into()?;
        let y: U = self.y.try_into()?;
        Ok(Vector2d { x, y })
    }
}
