use std::cmp::{Eq, PartialOrd};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops;
use std::str::FromStr;

use num::{Integer, Signed};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Coord3D<T: Integer + PartialOrd + Eq + Hash + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Integer + PartialOrd + Eq + Copy + Hash> Coord3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn mul_scalar(&self, v: T) -> Self {
        Self::new(self.x * v, self.y * v, self.z * v)
    }

    pub fn is_nonnegative(&self) -> bool {
        self.x >= T::zero() && self.y >= T::zero() && self.z >= T::zero()
    }

    pub fn manhattan_distance(&self, other: &Self) -> T {
        let n_x = {
            if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            }
        };

        let n_y = {
            if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            }
        };

        let n_z = {
            if self.z > other.z {
                self.z - other.z
            } else {
                other.z - self.z
            }
        };

        n_x + n_y + n_z
    }

    pub fn squared_euclidean_distance(&self, other: &Self) -> T {
        let n_x = {
            if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            }
        };

        let n_y = {
            if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            }
        };

        let n_z = {
            if self.z > other.z {
                self.z - other.z
            } else {
                other.z - self.z
            }
        };

        n_x * n_x + n_y * n_y + n_z * n_z
    }
}

impl<T: Integer + PartialOrd + Eq + Copy + Hash> ops::Add<Coord3D<T>> for Coord3D<T> {
    type Output = Self;

    fn add(self, _rhs: Coord3D<T>) -> Self {
        Coord3D::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl<T: Integer + Signed + PartialOrd + Eq + Copy + Hash> ops::Sub<Coord3D<T>> for Coord3D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Coord3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCoord3DUsizeError;

impl FromStr for Coord3D<usize> {
    type Err = ParseCoord3DUsizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",");
        Ok(Coord3D::new(
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ))
    }
}
