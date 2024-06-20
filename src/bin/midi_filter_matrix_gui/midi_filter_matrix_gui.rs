use crossbeam_channel::Sender;
use eframe::egui::{self, ViewportCommand, Widget};
use midi_filter_matrix::{midi_matrix::MidiMatrix, ui::elements::DebugConsole};

pub struct MidiFilterMatrixGUI {
    pub console: DebugConsole,
    pub tx_midi_matrix_opt: Option<Sender<MidiMatrix>>,
}

impl MidiFilterMatrixGUI {
    pub fn new(tx_midi_matrix: Sender<MidiMatrix>) -> Self {
        MidiFilterMatrixGUI {
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            tx_midi_matrix_opt: Some(tx_midi_matrix),
        }
    }
}

impl Default for MidiFilterMatrixGUI {
    fn default() -> Self {
        Self {
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            tx_midi_matrix_opt: None,
        }
    }
}

impl eframe::App for MidiFilterMatrixGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MidiMatrixGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.console.debug_console_ui(ui);
        });
    }
}
