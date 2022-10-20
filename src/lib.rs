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

#[derive(Debug, PartialEq)]
struct DMSAngle {
    sign: i8,
    degree: u32,
    minute: u32,
    second: f64,
}

#[derive(Debug, PartialEq)]
struct HMSAngle {
    sign: i8,
    hour: u32,
    minute: u32,
    second: f64,
}

impl DMSAngle {
    pub fn from_decimal(dec_angle: f64) -> Self {
        let (deg, deg_remainder) = div_rem(dec_angle, 1.0);
        let (min, min_remainder) = div_rem(deg_remainder * 60.0, 1.0);
        Self {
            sign: if dec_angle.is_sign_positive() {
                1_i8
            } else {
                -1_i8
            },
            degree: deg as u32,
            minute: min as u32,
            second: min_remainder * 60.0,
        }
    }

    pub fn to_hmsangle(&self) -> HMSAngle {
        HMSAngle::from_decimal(self.to_decimal() / 15.0)
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

impl HMSAngle {
    pub fn from_decimal(dec_hour: f64) -> Self {
        let (hour, hour_remainder) = div_rem(dec_hour, 1.0);
        let (min, min_remainder) = div_rem(hour_remainder * 60.0, 1.0);
        Self {
            sign: if dec_hour.is_sign_positive() {
                1_i8
            } else {
                -1_i8
            },
            hour: hour as u32,
            minute: min as u32,
            second: min_remainder * 60.0,
        }
    }

    pub fn to_dmsangle(&self) -> DMSAngle {
        DMSAngle::from_decimal(self.to_decimal() * 15.0)
    }

    pub fn from_radian(rad_angle: f64) -> Self {
        Self::from_decimal(rad_angle / PI * 12.0)
    }

    pub fn to_decimal(&self) -> f64 {
        (self.hour as f64 + self.minute as f64 / 60.0 + self.second / 3600.0) * self.sign as f64
    }

    pub fn to_radian(&self) -> f64 {
        Self::to_decimal(self) / 12.0 * PI
    }
}

#[cfg(test)]
mod tests {
    use crate::{DMSAngle, HMSAngle};
    use std::f64::consts::PI;

    #[test]
    fn test_dmsangle() {
        assert_eq!(0.0, -0.0);
        assert_eq!(
            DMSAngle {
                sign: 1_i8,
                degree: 0_u32,
                minute: 0_u32,
                second: 0.0
            },
            DMSAngle::from_decimal(0.0)
        );
        assert_eq!(
            DMSAngle {
                sign: -1_i8,
                degree: 0_u32,
                minute: 0_u32,
                second: 0.0
            },
            DMSAngle::from_decimal(-0.0)
        );
        assert_eq!(
            DMSAngle {
                sign: 1_i8,
                degree: 180_u32,
                minute: 0_u32,
                second: 0.0
            },
            DMSAngle::from_decimal(180.0)
        );
        assert_eq!(
            DMSAngle {
                sign: 1_i8,
                degree: 10_u32,
                minute: 10_u32,
                second: 10.0
            },
            DMSAngle::from_decimal(10.0 + 10.0 / 60.0 + 10.0 / 3600.0)
        );
        assert_eq!(
            DMSAngle {
                sign: 1_i8,
                degree: 180_u32,
                minute: 0_u32,
                second: 0.0
            },
            DMSAngle::from_radian(PI)
        );
    }
    #[test]
    fn test_hmsangle() {
        assert_eq!(0.0, -0.0);
        assert_eq!(
            HMSAngle {
                sign: 1_i8,
                hour: 0_u32,
                minute: 0_u32,
                second: 0.0
            },
            HMSAngle::from_decimal(0.0)
        );
        assert_eq!(
            HMSAngle {
                sign: -1_i8,
                hour: 0_u32,
                minute: 0_u32,
                second: 0.0
            },
            HMSAngle::from_decimal(-0.0)
        );
        assert_eq!(
            HMSAngle {
                sign: 1_i8,
                hour: 12_u32,
                minute: 0_u32,
                second: 0.0
            },
            HMSAngle::from_decimal(12.0)
        );
        assert_eq!(
            HMSAngle {
                sign: 1_i8,
                hour: 10_u32,
                minute: 10_u32,
                second: 10.0
            },
            HMSAngle::from_decimal(10.0 + 10.0 / 60.0 + 10.0 / 3600.0)
        );
        assert_eq!(
            HMSAngle {
                sign: 1_i8,
                hour: 12_u32,
                minute: 0_u32,
                second: 0.0
            },
            HMSAngle::from_radian(PI)
        );
    }

    #[test]
    fn test_angle_conversions() {
        assert_eq!(
            HMSAngle::from_decimal(12.0),
            DMSAngle::from_decimal(180.0).to_hmsangle()
        );
        assert_eq!(
            DMSAngle::from_decimal(90.0),
            HMSAngle::from_decimal(6.0).to_dmsangle()
        );
    }
}
