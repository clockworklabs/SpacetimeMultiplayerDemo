use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(tuple)]
pub struct StdbVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[spacetimedb(tuple)]
pub struct StdbQuarternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
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
    }
}