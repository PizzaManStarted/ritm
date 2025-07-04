

use turingrs::turing_machine::{TuringMachine};

use crate::ui;

pub struct App {
    pub turing: TuringMachine,
    pub target: f32,
}


impl Default for App {
    fn default() -> Self {
        Self {
            turing: TuringMachine::new(2),
            target: 0.0,
        }
    }
}


impl App {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // cc.egui_ctx.set_debug_on_hover(true);
        Default::default()
    }    
}



/// Update loop
impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // install_image_loaders(ctx);
        ui::show(self, ctx);
    }
}