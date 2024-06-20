use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::{self, egui::ViewportBuilder};
use midi_filter_matrix::{midi_matrix::MidiMatrix, ui::elements::DebugConsole};
use std::str::FromStr;
mod midi_filter_matrix_gui;

fn main() {
    let (tx_midi_matrix, rx_midi_matrix): (Sender<MidiMatrix>, Receiver<MidiMatrix>) = unbounded();
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
