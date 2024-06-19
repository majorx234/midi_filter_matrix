use std::str::FromStr;

use eframe::{self, egui::ViewportBuilder};
use midi_filter_matrix::ui::elements::DebugConsole;
mod midi_filter_matrix_gui;

fn main() {
    let midi_filter_matrix_gui = midi_filter_matrix_gui::MidiFilterMatrixGUI::default();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Midi Filter Matrix GUI",
        options,
        Box::new(|_cc| Box::new(midi_filter_matrix_gui)),
    );
}
