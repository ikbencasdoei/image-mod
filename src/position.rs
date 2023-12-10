use std::ops::{Add, AddAssign, Div, Mul, Sub};

use eframe::egui::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const ZERO: Self = Self::from_u32(0, 0);
    pub const ONE: Self = Self::from_u32(1, 1);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn from_u32(x: u32, y: u32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }

    pub const fn from_i32(x: i32, y: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }

    pub fn try_into_u32(self) -> Result<(u32, u32), &'static str> {
        let (x, y) = self.into_i32();
        if x.is_negative() || y.is_negative() {
            Err("negative value")
        } else {
            Ok((x as u32, y as u32))
        }
    }

    pub fn into_i32(self) -> (i32, i32) {
        let Position { x, y } = self;
        (x as i32, y as i32)
    }

    pub fn into_f32(self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Vec2> for Position {
    fn from(value: Vec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl Mul for Position {
    type Output = Position;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Position {
    type Output = Position;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Position {
    type Output = Position;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
