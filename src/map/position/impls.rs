use crate::Position;
use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

impl Add<Self> for Position {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

// TODO make direction to work
//impl Add<EdgeDirection> for Position {
//    type Output = Self;
//
//    #[inline]
//    fn add(self, rhs: EdgeDirection) -> Self::Output {
//        self.add_dir(rhs)
//    }
//}
//
//impl Add<VertexDirection> for Position {
//    type Output = Self;
//
//    #[inline]
//    fn add(self, rhs: VertexDirection) -> Self::Output {
//        self.add_diag_dir(rhs)
//    }
//}

impl AddAssign for Position {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

//impl AddAssign<EdgeDirection> for Position {
//    #[inline]
//    fn add_assign(&mut self, rhs: EdgeDirection) {
//        *self = self.add(rhs);
//    }
//}
//
//impl AddAssign<VertexDirection> for Position {
//    #[inline]
//    fn add_assign(&mut self, rhs: VertexDirection) {
//        *self = self.add(rhs);
//    }
//}

impl Sum for Position {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Self::const_add)
    }
}

impl<'a> Sum<&'a Self> for Position {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, &b| Self::const_add(a, b))
    }
}

impl Sub<Self> for Position {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

//impl Sub<EdgeDirection> for Position {
//    type Output = Self;
//
//    #[inline]
//    fn sub(self, rhs: EdgeDirection) -> Self::Output {
//        self.sub(Self::from(rhs))
//    }
//}
//
//impl Sub<VertexDirection> for Position {
//    type Output = Self;
//
//    #[inline]
//    fn sub(self, rhs: VertexDirection) -> Self::Output {
//        self.sub(Self::from(rhs))
//    }
//}

impl SubAssign for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

//impl SubAssign<EdgeDirection> for Position {
//    #[inline]
//    fn sub_assign(&mut self, rhs: EdgeDirection) {
//        *self = self.sub(rhs);
//    }
//}
//
//impl SubAssign<VertexDirection> for Position {
//    #[inline]
//    fn sub_assign(&mut self, rhs: VertexDirection) {
//        *self = self.sub(rhs);
//    }
//}

impl Mul<Self> for Position {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x.mul(rhs.x), y: self.y.mul(rhs.y), z: self.z.mul(rhs.z) }
    }
}

impl MulAssign for Position {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl Product for Position {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1, 1, 0), Self::mul)
    }
}

impl<'a> Product<&'a Self> for Position {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::new(1, 1, 0), |a, &b| Self::mul(a, b))
    }
}
impl Div<Self> for Position {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: if rhs.x != 0 && self.x != 0 { self.x.div(rhs.x) } else { self.x },
            y: if rhs.y != 0 && self.y != 0 { self.y.div(rhs.y) } else { self.y },
            z: if rhs.z != 0 && self.z != 0 { self.z.div(rhs.z) } else { self.z },
        }
    }
}

impl DivAssign for Position {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl Rem<Self> for Position {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        self - (self / rhs) * rhs
    }
}

impl RemAssign for Position {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl Neg for Position {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl BitAnd for Position {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { x: self.x.bitand(rhs.x), y: self.y.bitand(rhs.y), z: self.z.bitand(rhs.z) }
    }
}

impl BitOr for Position {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { x: self.x.bitor(rhs.x), y: self.y.bitor(rhs.y), z: self.z.bitor(rhs.z) }
    }
}

impl BitXor for Position {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { x: self.x.bitxor(rhs.x), y: self.y.bitxor(rhs.y), z: self.z.bitxor(rhs.z) }
    }
}
