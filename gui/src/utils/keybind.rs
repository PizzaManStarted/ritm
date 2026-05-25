use egui::{Key, Ui};

use crate::App;

pub fn keybind(ui: &mut Ui, app: &mut App) {
    if app.transient.listen_to_keybind {
        let popup_displayed = app.popup.current().is_some();

        ui.input(|r| {
            if r.key_pressed(Key::Escape) {
                if app.popup.current().is_some() {
                    // Request graceful exit of popup
                    app.popup.close();
                } else {
                    // Unselect what is selected
                    app.graph.unselect()
                }
            }

            if r.key_pressed(Key::Enter) {
                if !app.error.is_empty() {
                    app.error.pop_front();
                } else if app.popup.current().is_some() {
                    app.popup.confirm();
                } else if !popup_displayed && app.tutorial.in_tutorial() {
                    app.tutorial.next();
                }
            }

            if !popup_displayed && !app.tutorial.in_tutorial() {
                // Press S to create a state
                if r.key_pressed(Key::S) {
                    app.edit.is_adding_state ^= true;
                }

                // Press T to create a transition
                if app.graph.selected_state().is_some() && r.key_pressed(Key::T) {
                    app.edit.is_adding_transition ^= true;
                }

                // Press U to unpin all state
                if r.key_pressed(Key::U) {
                    app.turing.unpin_all();
                }

                // Press C to open and close code section
                if r.key_pressed(Key::C) {
                    app.code.toggle();
                }

                // Press R to recenter
                if r.key_pressed(Key::R) {
                    app.graph.recenter();
                }

                // Press Space to make 1 iteration
                if app.turing.accepted.is_none() && r.key_pressed(Key::Space) {
                    app.turing.next_step();
                }

                // Press P to autoplay the machine
                if r.key_pressed(Key::P) {
                    if app.control.is_running() {
                        app.control.run();
                    } else {
                        app.control.pause();
                    }
                }

                if r.key_pressed(Key::Plus) {
                    app.control.speed_down();
                }

                if r.key_pressed(Key::Minus) {
                    app.control.speed_up();
                }

                // Press Backspace to reset the machine
                if r.key_pressed(Key::Backspace) {
                    app.reset();
                }

                // Press Backspace to reset the machine
                if r.key_pressed(Key::Delete) {
                    if let Some(state_id) = app.graph.selected_state()
                        && state_id > 1
                    {
                        let _ = app.turing.remove_state(state_id);
                    }

                    if let Some(transition_id) = app.graph.selected_transitions() {
                        let _ = app
                            .turing
                            .remove_transitions(transition_id.source_id, transition_id.target_id);
                    }
                }
            }
        });
    } else {
        app.transient.listen_to_keybind = true;
    }
}
