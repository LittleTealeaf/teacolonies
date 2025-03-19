use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub};

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct HF(f64);

impl Hash for HF {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ser) = serde_json::to_string(&self.0).ok() {
            ser.hash(state);
        } else {
            let rounded_value = (self.0 * 10000.0).round() / 10000.0;
            rounded_value.to_bits().hash(state);
        }
    }
}

impl Add for HF {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        HF(self.0 + other.0)
    }
}

impl Sub for HF {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        HF(self.0 - other.0)
    }
}

impl Mul for HF {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        HF(self.0 * other.0)
    }
}

impl Div for HF {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        HF(self.0 / other.0)
    }
}
