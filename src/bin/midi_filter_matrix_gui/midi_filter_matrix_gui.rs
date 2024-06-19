use eframe::egui::{self, ViewportCommand, Widget};
use midi_filter_matrix::ui::elements::DebugConsole;

pub struct MidiFilterMatrixGUI {
    pub console: DebugConsole,
}

impl Default for MidiFilterMatrixGUI {
    fn default() -> Self {
        Self {
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
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
