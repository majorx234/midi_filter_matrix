#[derive(Debug, Clone, Copy)]
pub struct Vector4b {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

impl Vector4b {
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Vector4b {
        Vector4b { x, y, z, w }
    }

    pub fn zero() -> Vector4b {
        Vector4b {
            x: false,
            y: false,
            z: false,
            w: false,
        }
    }
}

impl Default for Vector4b {
    fn default() -> Vector4b {
        Vector4b {
            x: false,
            y: false,
            z: false,
            w: false,
        }
    }
}
