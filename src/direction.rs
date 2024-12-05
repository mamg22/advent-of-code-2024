use std::ops::Neg;

use num_traits::Num;

use crate::vector::Vector2d;

pub trait AsVector {
    fn as_vector<T: Num + Neg<Output = T>>(self) -> Vector2d<T>;
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl AsVector for Direction {
    fn as_vector<T: Num + Neg<Output = T>>(self) -> Vector2d<T> {
        match self {
            Direction::North => Vector2d::new(T::zero(), -T::one()),
            Direction::East => Vector2d::new(T::one(), T::zero()),
            Direction::South => Vector2d::new(T::zero(), T::one()),
            Direction::West => Vector2d::new(-T::one(), T::zero()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl AsVector for Direction8 {
    fn as_vector<T: Num + Neg<Output = T>>(self) -> Vector2d<T> {
        match self {
            Direction8::North => Vector2d::new(T::zero(), -T::one()),
            Direction8::NorthEast => Vector2d::new(T::one(), -T::one()),
            Direction8::East => Vector2d::new(T::one(), T::zero()),
            Direction8::SouthEast => Vector2d::new(T::one(), T::one()),
            Direction8::South => Vector2d::new(T::zero(), T::one()),
            Direction8::SouthWest => Vector2d::new(-T::one(), T::one()),
            Direction8::West => Vector2d::new(-T::one(), T::zero()),
            Direction8::NorthWest => Vector2d::new(-T::one(), -T::one()),
        }
    }
}