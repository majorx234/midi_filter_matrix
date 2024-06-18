use struct_iterable::Iterable;

#[derive(Debug, Clone, Copy, Iterable)]
pub struct Vector4b {
    pub midi_out0: bool,
    pub midi_out1: bool,
    pub midi_out2: bool,
    pub midi_out3: bool,
}

impl Vector4b {
    pub fn new(midi_out0: bool, midi_out1: bool, midi_out2: bool, midi_out3: bool) -> Vector4b {
        Vector4b {
            midi_out0,
            midi_out1,
            midi_out2,
            midi_out3,
        }
    }

    pub fn zero() -> Vector4b {
        Vector4b {
            midi_out0: false,
            midi_out1: false,
            midi_out2: false,
            midi_out3: false,
        }
    }
}

impl Default for Vector4b {
    fn default() -> Vector4b {
        Vector4b {
            midi_out0: false,
            midi_out1: false,
            midi_out2: false,
            midi_out3: false,
        }
    }
}
