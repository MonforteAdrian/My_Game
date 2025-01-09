use std::ops::{Mul, Neg, Shl, Shr};

use super::{EdgeDirection, VertexDirection};
use crate::Position;

impl Neg for VertexDirection {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl Neg for EdgeDirection {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl Shr<u8> for EdgeDirection {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        self.rotate_cw(rhs)
    }
}

impl Shr<u8> for VertexDirection {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        self.rotate_cw(rhs)
    }
}

impl Shl<u8> for EdgeDirection {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        self.rotate_ccw(rhs)
    }
}

impl Shl<u8> for VertexDirection {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        self.rotate_ccw(rhs)
    }
}

impl Mul<i32> for EdgeDirection {
    type Output = Position;

    fn mul(self, rhs: i32) -> Self::Output {
        Position::from(self).mul(Position::splat(rhs))
    }
}

impl Mul<i32> for VertexDirection {
    type Output = Position;

    fn mul(self, rhs: i32) -> Self::Output {
        Position::from(self).mul(Position::splat(rhs))
    }
}
