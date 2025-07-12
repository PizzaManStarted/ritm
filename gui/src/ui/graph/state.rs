use egui::{vec2, Color32, Label, Rect, RichText, Sense, Stroke, Ui};

use crate::{
    ui::{constant::Constant, font::Font, utils::{self, constrast_color}}, App
};

pub fn show(app: &mut App, ui: &mut Ui) {
    // This line copy every keys of the hasmap to avoid borrowing the struct App that we need in each call.
    let keys: Vec<u8> = app.states.keys().map(|u| *u).collect::<Vec<u8>>();
    for i in keys {
        draw_node(app, ui, i);
    }
}

fn draw_node(app: &mut App, ui: &mut Ui, state_id: u8) {
    // Get the state information
    let state = app.states.get(&state_id).unwrap();

    // Define the boundaries of the node
    let rect = Rect::from_center_size(
        state.position,
        vec2(Constant::STATE_RADIUS, Constant::STATE_RADIUS) * 2.0,
    );

    // Listen for click and drag event on the node
    let response = ui.allocate_rect(rect, Sense::click_and_drag());

    // Draw the node circle
    ui.painter().circle(
        state.position,
        Constant::STATE_RADIUS,
        state.color,
        if app.selected_state.is_some_and(|id| id == state_id ) {
            Stroke::new(3.0, Color32::BLUE)
        } else {
            Stroke::new(1.0, Color32::BLACK)
        },
    );

    let name = RichText::new(&state.name)
        .font(Font::default())
        .color(constrast_color(state.color));

    let label = Label::new(name).truncate();

    // Draw the label inside the node, without overflow
    ui.put(
        Rect::from_center_size(
            rect.center(),
            utils::text_size(ui, Font::default(), &state.name).min(vec2(rect.width(), rect.height()))
        ),
        label
    );


    if response.clicked() {

        if app.event.is_adding_transition {
            app.add_transition(state_id);
        } else {
            app.selected_transition = None;
            app.selected_state = Some(state_id)
        }
    }

    // If dragged, make the node follow the pointer
    if response.dragged() {
        app.states.get_mut(&state_id).unwrap().position = response.interact_pointer_pos().unwrap();
    }

}
