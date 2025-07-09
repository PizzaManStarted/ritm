use std::collections::HashMap;

use egui::{
    Align, Color32, Direction, Frame, Image, Label, Layout, Margin, Pos2, Rect, Scene, Shadow,
    Stroke, TextWrapMode, Ui, UiBuilder, Vec2, include_image, vec2,
};

use crate::{
    ui::{constant::Constant, edit, theme::Theme, utils}, App
};

pub mod state;
pub mod transition;

/// Show the graph display of the turing machine
///
/// User can edit the graph and update the code and turing machine
pub fn show(app: &mut App, ui: &mut Ui) {
    // current rect of the element inside the scene
    let mut inner_rect = Rect::ZERO;
    let mut scene_rect = app.graph_rect;

    let ui_rect = ui.available_rect_before_wrap();

    // Compute the force applied on every node
    // apply_force(app);

    let scene_response = Frame::new()
        .fill(app.theme.color5)
        .corner_radius(5)
        .show(ui, |ui| {
            Scene::new()
                .show(ui, &mut scene_rect, |ui| {
                    // Draw the transitions of the turing machine
                    transition::show(app, ui);

                    // Draw the states of the turing machine
                    state::show(app, ui);

                    // This Rect can be used to "Reset" the view of the graph
                    inner_rect = ui.min_rect();
                    
                })
                .response
        })
        .inner;

    // Save scene border
    app.graph_rect = scene_rect;

    // If the graph scene is clicked
    if scene_response.clicked() {

        if app.event.is_adding_state {
            
            app.add_state(scene_response.interact_pointer_pos().unwrap());
        }


        // CLick on the scene reset selection and editing
        app.event.is_adding_state = false;
        app.event.is_adding_transition = false;
        app.selected_state = None;
        app.selected_transition = None;
    }


    edit::show(app, ui, ui_rect);

    // Repaint the canvas
    ui.ctx().request_repaint();
}


/// Apply natural force on the node
///
/// If 2 nodes are too close, they repulse each other to reach a distance L
/// If 2 nodes are linked by a transition, they attract each other to reach a distance L
pub fn apply_force(app: &mut App) -> bool {
    let mut forces: HashMap<u8, Vec2> = HashMap::new();

    // register the max force applied on a state to check if the system is stable
    let mut max_force_applied: f32 = 0.0;

    for (i, state_1) in app.states.iter() {
        let mut force: f32 = 0.0;
        let mut final_force: Vec2 = Vec2::ZERO;

        for (j, state_2) in app.states.iter() {
            // continue if it's the same state
            if j == i {
                continue;
            }

            // true if there is a transition between the two states
            let are_adjacent = app
                .turing
                .get_transition_index(*i, *j)
                .is_ok_and(|v| !v.is_empty())
                || app
                    .turing
                    .get_transition_index(*j, *i)
                    .is_ok_and(|v| !v.is_empty());

            let distance = utils::distance(state_1.position, state_2.position);
            let direction = utils::direction(state_1.position, state_2.position);

            // different equations are use based on the adjacency of the states
            if are_adjacent {
                force = utils::attract_force(state_1.position, state_2.position);
            } else if distance < Constant::L {
                force = -utils::rep_force(state_1.position, state_2.position);
            };

            // apply the force on the final force vector
            final_force += direction * force;
        }

        // save the highest force applied
        if force > max_force_applied {
            max_force_applied = force;
        }

        // store the compute force to not alter the current physical state
        forces.insert(*i, final_force);
    }

    for (i, state) in app.states.iter_mut() {
        // translate the state by the amount of force
        state.position += *forces.get(&i).unwrap();
    }

    max_force_applied < Constant::STABILITY_TRESHOLD
}
