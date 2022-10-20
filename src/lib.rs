use std::f64::consts::PI;

pub fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(
    x: T,
    y: T,
) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

pub fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
    div_rem(x, y)
}

struct DMSAngle {
    sign: i8,
    degree: u32,
    minute: u32,
    second: f64,
}

impl DMSAngle {
    pub fn from_decimal(dec_angle: f64) -> Self {
        let (deg, min) = div_rem(dec_angle, 360.0);
        let (min, sec) = div_rem(min, 60.0);
        Self {
            sign: if dec_angle >= 0.0 { 1_i8 } else { -1_i8 },
            degree: deg as u32,
            minute: min as u32,
            second: sec,
        }
    }

    pub fn from_radian(rad_angle: f64) -> Self {
        Self::from_decimal(rad_angle / PI * 180.0)
    }

    pub fn to_decimal(&self) -> f64 {
        (self.degree as f64 + self.minute as f64 / 60.0 + self.second / 3600.0) * self.sign as f64
    }

    pub fn to_radian(&self) -> f64 {
        Self::to_decimal(self) / 180.0 * PI
    }
}
