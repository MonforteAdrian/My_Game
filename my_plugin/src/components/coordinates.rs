use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates  {
    pub x: u16,
    pub y: u16,
    //pub z: u16,
}

// Coordinates + Coordinates
impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            //z: self.z + rhs.z,
        }
    }
}

// Coordinates - Coordinates
impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
            //z: self.z.saturating_sub(rhs.z),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //write!(f, "({}, {}, {})", self.x, self.y, self.z)
        write!(f, "({}, {})", self.x, self.y)
    }
}
