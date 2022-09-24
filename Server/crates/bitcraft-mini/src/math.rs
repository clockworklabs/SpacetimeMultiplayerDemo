use spacetimedb::spacetimedb;
use std::ops;

#[derive(Clone, Copy, Debug)]
#[spacetimedb(tuple)]
pub struct StdbVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl StdbVector3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn up() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }

    // Normalize the vector in place
    #[inline]
    pub fn normalize(&mut self) {
        let len = self.length();

        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    // Return a new, normalized version of this vector
    #[inline]
    pub fn normalized(self) -> StdbVector3 {
        let len = self.length();
        let ret = StdbVector3::new(self.x / len, self.y / len, self.z / len);

        return ret;
    }

    #[inline]
    pub fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    #[inline]
    pub fn length_sq(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    #[inline]
    pub fn sq_distance(&self, other: &StdbVector3) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }

    #[inline]
    pub fn cross(&self, rhs: StdbVector3) -> StdbVector3 {
        StdbVector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    pub fn dot(&self, rhs: StdbVector3) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
}

// StdbVector3 Operators
// + will memberwise add two Vectors
impl ops::Add<StdbVector3> for StdbVector3 {
    type Output = StdbVector3;

    #[inline]
    fn add(self, rhs: StdbVector3) -> StdbVector3 {
        StdbVector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// - will memberwise subtract two Vectors
impl ops::Sub<StdbVector3> for StdbVector3 {
    type Output = StdbVector3;

    #[inline]
    fn sub(self, rhs: StdbVector3) -> StdbVector3 {
        StdbVector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// * w/ a float will scale the Vector and is commutative
impl ops::Mul<f32> for StdbVector3 {
    type Output = StdbVector3;

    #[inline]
    fn mul(self, rhs: f32) -> StdbVector3 {
        StdbVector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// * w/ a float will scale the Vector and is commutative
impl ops::Mul<StdbVector3> for f32 {
    type Output = StdbVector3;

    #[inline]
    fn mul(self, rhs: StdbVector3) -> StdbVector3 {
        StdbVector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[derive(Clone, Copy, Debug)]
#[spacetimedb(tuple)]
pub struct StdbQuaternion {
    pub y: f32,
    pub x: f32,
    pub z: f32,
    pub w: f32,
}

impl StdbQuaternion {
    pub fn identity() -> Self {
        Self {
            y: 0.0,
            x: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn new(yaw: f32, pitch: f32, roll: f32) -> StdbQuaternion {
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();

        StdbQuaternion {
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
            w: cr * cp * cy + sr * sp * sy,
        }
    }

    pub fn look_rotation(forward: StdbVector3, up: StdbVector3) -> StdbQuaternion {
        let vector = forward.normalized();
        let vector2 = up.cross(vector).normalized();
        let vector3 = vector.cross(vector2);
        let m00 = vector2.x;
        let m01 = vector2.y;
        let m02 = vector2.z;
        let m10 = vector3.x;
        let m11 = vector3.y;
        let m12 = vector3.z;
        let m20 = vector.x;
        let m21 = vector.y;
        let m22 = vector.z;

        let num8 = (m00 + m11) + m22;
        if num8 > 0.0 {
            let num = (num8 + 1.0).sqrt();
            let w = num * 0.5;
            let num = 0.5 / num;
            return StdbQuaternion {
                x: (m12 - m21) * num,
                y: (m20 - m02) * num,
                z: (m01 - m10) * num,
                w,
            };
        }
        if m00 >= m11 && m00 >= m22 {
            let num7 = (((1.0 + m00) - m11) - m22).sqrt();
            let num4 = 0.5 / num7;
            return StdbQuaternion {
                x: 0.5 * num7,
                y: (m01 + m10) * num4,
                z: (m02 + m20) * num4,
                w: (m12 - m21) * num4,
            };
        }
        if m11 > m22 {
            let num6 = (((1.0 + m11) - m00) - m22).sqrt();
            let num3 = 0.5 / num6;
            return StdbQuaternion {
                x: (m10 + m01) * num3,
                y: 0.5 * num6,
                z: (m21 + m12) * num3,
                w: (m20 - m02) * num3,
            };
        }
        let num5 = (((1.0 + m22) - m00) - m11).sqrt();
        let num2 = 0.5 / num5;
        StdbQuaternion {
            x: (m20 + m02) * num2,
            y: (m21 + m12) * num2,
            z: 0.5 * num5,
            w: (m01 - m10) * num2,
        }
    }
}

#[inline]
pub(crate) fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    assert!(max >= min);
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

#[inline]
pub(crate) fn remap(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    if value < old_min {
        return new_min;
    }
    if value > old_max {
        return new_max;
    }

    let mut value = (value - old_min) / (old_max - old_min);
    value = (value * (new_max - new_min)) + new_min;
    return clamp(value, new_min, new_max);
}

#[inline]
pub(crate) fn map_to_u8(value: f32, min_value: f32, max_value: f32) -> u8 {
    let mut value = value - min_value;
    value = value / max_value;
    return if value < 0.0 {
        0
    } else if value > u8::MAX as f32 {
        u8::MAX
    } else {
        ((value * u8::MAX as f32).round() + 0.1) as u8
    };
}
