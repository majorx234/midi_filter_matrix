use crate::vector4b::Vector4b;
use struct_iterable::Iterable;

#[derive(Debug, Clone, Copy, Iterable)]
pub struct MidiMatrix {
    /// The first column of the matrix.
    pub midi_in0: Vector4b,
    /// The second column of the matrix.
    pub midi_in1: Vector4b,
    /// The third column of the matrix.
    pub midi_in2: Vector4b,
    /// The fourth column of the matrix.
    pub midi_in3: Vector4b,
}

impl MidiMatrix {
    pub fn new() -> Self {
        MidiMatrix {
            midi_in0: Vector4b::zero(),
            midi_in1: Vector4b::zero(),
            midi_in2: Vector4b::zero(),
            midi_in3: Vector4b::zero(),
        }
    }
}

impl Default for MidiMatrix {
    fn default() -> Self {
        MidiMatrix {
            midi_in0: Vector4b::zero(),
            midi_in1: Vector4b::zero(),
            midi_in2: Vector4b::zero(),
            midi_in3: Vector4b::zero(),
        }
    }
}
