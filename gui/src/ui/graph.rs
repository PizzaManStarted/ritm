use std::{collections::HashMap, io::Cursor, vec};

use egui::{
    Button, Color32, Event, Id, Image, LayerId, Pos2, Rect, Scene, Stroke, Ui, UiBuilder, UserData,
    Vec2, ViewportCommand, include_image, pos2, vec2,
};
use image::{ImageBuffer, Rgba};

use crate::{
    App, error::RitmError, turing::{StateWrapper, TransitionId, Turing}, ui::{
        edit,
        graph::transition::{draw_arrow, draw_self_arrow},
        popup::RitmPopupEnum,
        theme::LIGHT_THEME,
    }, utils::{constant::Constant, physic},
};

pub mod state;
pub mod transition;

pub struct Graph {
    // The id of a state if selected
    selected_state: Option<usize>,
    // The id of a transition if selected
    selected_transitions: Option<TransitionId>,
    // The graph rect
    graph_rect: Rect,
    recenter: bool,
    is_stable: bool,
    is_dragging: bool,
    grid_enabled: bool,
    // The start state id and the end state id if hovered
    drag_transition: Option<(usize, Option<usize>)>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            selected_state: Default::default(),
            selected_transitions: Default::default(),
            graph_rect: Rect::ZERO,
            recenter: false,
            is_stable: false,
            is_dragging: false,
            drag_transition: None,
            grid_enabled: true,
        }
    }
}

impl Graph {
    /// Return None if no state are selected
    pub fn selected_state(&self) -> Option<usize> {
        self.selected_state
    }

    /// Return None if no transitions are selected
    pub fn selected_transitions(&self) -> Option<TransitionId> {
        self.selected_transitions
    }

    pub fn select_state(&mut self, state_id: usize) {
        self.selected_state = Some(state_id);
        self.selected_transitions = None;
    }

    pub fn select_transitions(&mut self, transition_id: TransitionId) {
        self.selected_transitions = Some(transition_id);
        self.selected_state = None;
    }

    /// Unselect state or transition selected
    pub fn unselect(&mut self) {
        self.selected_state = None;
        self.selected_transitions = None;
    }

    /// Request to recenter the graph
    pub fn recenter(&mut self) {
        self.recenter = true;
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    pub fn reset(&mut self) {
        self.selected_state = None;
        self.selected_transitions = None;
        self.drag_transition = None;
    }
}

/// Show the graph display of the turing machine
///
/// User can edit the graph and update the code and turing machine
pub fn show(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    // Current rect of the element inside the scene
    let mut inner_rect = Rect::ZERO;

    // Previous frame scene rect
    let mut scene_rect = app.ui.graph.graph_rect;

    let graph_rect = ui.available_rect_before_wrap();

    // Compute the force applied on every node when no node are dragged
    if !app.ui.graph.is_dragging && !app.ui.graph.grid_enabled {
        apply_force(app);
    }

    let scene_response = Scene::new()
        .zoom_range(0.1..=1.0)
        .show(ui, &mut scene_rect, |ui| {
            // Draw the grid
            if app.ui.graph.grid_enabled {
                let bound = ui.clip_rect();

                let increment = app.settings.grid_size as i32;
                // println!("{:?} {}", bound.min, );
                let offset = bound.min / (increment as f32).round();
                for i in 0..=(bound.width() / increment as f32) as i32 {
                    ui.painter().line(
                        vec![
                            pos2(((offset.x as i32 + i) * increment) as f32, bound.min.y),
                            pos2(((offset.x as i32 + i) * increment) as f32, bound.max.y),
                        ],
                        Stroke::new(1.0, Color32::GRAY),
                    );
                }
                for i in 0..=(bound.height() / increment as f32) as i32 {
                    ui.painter().line(
                        vec![
                            pos2(bound.min.x, ((offset.y as i32 + i) * increment) as f32),
                            pos2(bound.max.x, ((offset.y as i32 + i) * increment) as f32),
                        ],
                        Stroke::new(1.0, Color32::GRAY),
                    );
                }

                ui.painter().line(
                    vec![pos2(0.0, 0.0), pos2(0.0, 10.0)],
                    Stroke::new(1.0, Color32::BLUE),
                );
                ui.painter().line(
                    vec![pos2(0.0, 0.0), pos2(10.0, 0.0)],
                    Stroke::new(1.0, Color32::RED),
                );
            }

            // Draw the transitions of the turing machine
            transition::show(app, ui)?;

            // Draw the states of the turing machine on top of the transition
            state::show(app, ui)?;

            // Handle the drag of a transition from a state
            if transition_dragging(ui, app, graph_rect).is_err() {
                // app.error.push_back(x);
            }

            // This Rect can be used to "Reset" the view of the graph
            inner_rect = ui.min_rect();

            Ok::<(), RitmError>(())
        })
        .response;

    // Take a screenshot if a screenshot event is received
    // TODO replace it by svg ?
    ui.input(|i| {
        for e in &i.raw.events {
            if let Event::Screenshot { image, .. } = e {
                let ppp = i.pixels_per_point();
                let region = graph_rect;
                let image = image.region(&region, Some(ppp));
                let img2: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
                    image.width() as u32,
                    image.height() as u32,
                    image.as_raw(),
                )
                .unwrap();
                let mut bytes: Vec<u8> = Vec::new();
                img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
                    .unwrap();
                app.transient.temp_screenshot = Some(bytes);
                app.transient.taking_screenshot = false
            }
        }
    });

    // Add a state by long press on empty space
    // TODO make popup safer to avoid infinite popup
    if scene_response.is_pointer_button_down_on()
        && !scene_response.dragged()
        && app.ui.popup.current().is_none()
    {
        let time = ui.input(|r| r.time);
        let time_down = time - ui.input(|r| r.pointer.press_start_time()).unwrap_or(time);
        if time_down
            > ui.ctx()
                .options(|r| r.input_options.max_click_duration - 0.4)
        {
            let pointer_pos = scene_response
                .interact_pointer_pos()
                .expect("Pointer should exist");
            app.new_state_at_pos(pointer_pos);
        }
        ui.ctx().request_repaint();
    }

    // TODO maybe enable the button when small windows but change the behavior to save code as text file directly
    let layer = LayerId::new(egui::Order::Middle, Id::new("graph-buttons"));

    // The different button on top of the graph
    ui.scope_builder(
        UiBuilder::new()
            .layer_id(layer)
            .max_rect(ui.max_rect().shrink(10.0)), // Shrink the size of the overlay to avoid problem
        |ui| {
            // Add a button to hide all ?
            toggle_grid(ui, app);
        },
    );

    // Save scene border and recenter if asked
    // TODO find a better way to recenter, to avoid sticking to top
    app.ui.graph.graph_rect = if app.ui.graph.recenter {
        app.ui.graph.recenter = false;
        inner_rect
    } else {
        scene_rect
    };

    // Reset the graph (after recenter because need to redraw the states)
    if !app.transient.taking_screenshot {
        reset_button(ui, app, layer);
    }

    // If the graph scene is clicked...
    if scene_response.clicked() {
        //... and the user want a new state
        if app.ui.edit.is_adding_state {
            let click_pos = scene_response
                .interact_pointer_pos()
                .expect("no click position found");
            app.new_state_at_pos(click_pos);
        }

        // CLick on the scene reset selection and editing
        app.ui.edit.is_adding_state &= !app.settings.reset_after_action;
        app.ui.edit.is_adding_transition = false;
        app.ui.graph.unselect();
    }

    if !app.transient.taking_screenshot {
        edit::show(app, ui)?;
    }

    // // Take a screenshot of the machine
    // if !app.transient.taking_screenshot {
    //     take_screenshot_button(ui, app, layer);
    // }

    // Force the repaint of the canvas if the graph is instable
    if !app.ui.graph.is_stable {
        ui.ctx().request_repaint();
    }
    Ok(())
}

/// Apply natural force on the node
///
/// If 2 nodes are too close, they repulse each other to reach a distance L
/// If 2 nodes are linked by a transition, they attract each other to reach a distance L
fn apply_force(app: &mut App) {
    let mut forces: HashMap<usize, Vec2> = HashMap::new();

    // register the max force applied on a state to check if the system is stable
    let mut max_force_applied: f32 = 0.0;

    let states = app.turing.get_states();

    for i in 0..states.len() {
        let mut force: f32 = 0.0;
        let mut final_force: Vec2 = Vec2::ZERO;

        for j in 0..states.len() {
            // continue if it's the same state
            if j == i {
                continue;
            }

            let are_adjacent = app.turing.adjacent(i, j);

            let distance = physic::distance(
                states[i].inner_state.position,
                states[j].inner_state.position,
            );
            let direction = physic::direction(
                states[i].inner_state.position,
                states[j].inner_state.position,
            );
            let size = Constant::L + (100 * app.turing.writing_tape_count()) as f32;

            // different equations are use based on the adjacency of the states
            if are_adjacent {
                force = physic::attract_force(
                    states[i].inner_state.position,
                    states[j].inner_state.position,
                    size,
                );
            } else if distance < size {
                force = -physic::rep_force(
                    states[i].inner_state.position,
                    states[j].inner_state.position,
                );
            };

            // apply the force on the final force vector
            final_force += direction * force;
        }

        // save the highest force applied
        if force.abs() > max_force_applied {
            max_force_applied = force.abs();
        }

        // store the compute force to not alter the current physical state
        forces.insert(i, final_force);
    }

    let mut states_mut: Vec<&mut StateWrapper> = app.turing.get_states_mut();

    for state_mut in states_mut.iter_mut().filter(|s| !s.inner_state.is_pinned) {
        // translate the state by the amount of force
        state_mut.inner_state.position += *forces.get(&state_mut.get_id()).expect("Should exist")
    }

    app.ui.graph.is_stable = max_force_applied < Constant::STABILITY_TRESHOLD;
}

/// Button to reset the graph to the initial and accepting state
fn reset_button(ui: &mut Ui, app: &mut App, layer: LayerId) {
    let icon_size = Vec2::splat(app.settings.edit_button_size + 10.0);
    ui.scope_builder(
        UiBuilder::new()
            .layer_id(layer)
            .max_rect(Rect::from_min_size(
                ui.max_rect().right_top() - vec2(icon_size.x, 0.0),
                icon_size,
            )),
        |ui| {
            let button = ui.put(
                Rect::from_min_size(
                    ui.max_rect().right_top() - vec2(icon_size.x + 10.0, 0.0),
                    icon_size,
                ),
                Button::image(
                    Image::new(include_image!("../../assets/icon/erase.svg"))
                        .fit_to_exact_size(icon_size)
                        .tint(LIGHT_THEME.surface),
                )
                .frame(false),
            );
            if button.clicked() {
                app.turing = Turing::default()
            }
        },
    );
}

/// Button to reset the graph to the initial and accepting state
fn toggle_grid(ui: &mut Ui, app: &mut App) {
    let icon_size = Vec2::splat(app.settings.edit_button_size + 10.0);
    let icon_color = if app.ui.graph.grid_enabled {
        LIGHT_THEME.primary
    } else {
        LIGHT_THEME.icon
    };

    let button = ui.put(
        Rect::from_min_size(ui.max_rect().left_top(), icon_size),
        Button::image(
            Image::new(include_image!("../../assets/icon/grid.svg"))
                .fit_to_exact_size(icon_size)
                .tint(icon_color),
        )
        .frame(false),
    );
    if button.clicked() {
        app.ui.graph.grid_enabled ^= true;

        if app.ui.graph.grid_enabled {
            let increment = app.settings.grid_size as f32;
            for state in app.turing.get_states_mut() {
                state.inner_state.position =
                    (state.inner_state.position / increment).round() * increment
            }
        }
    }
}

/// Button to reset the graph to the initial and accepting state
fn take_screenshot_button(ui: &mut Ui, app: &mut App, layer: LayerId) {
    let icon_size = Vec2::splat(app.settings.edit_button_size + 10.0);
    ui.scope_builder(
        UiBuilder::new()
            .layer_id(layer)
            .max_rect(Rect::from_min_size(
                ui.max_rect().left_bottom() - vec2(0.0, icon_size.y),
                icon_size,
            )),
        |ui| {
            let button = ui.put(
                Rect::from_min_size(
                    ui.max_rect().left_bottom() - vec2(0.0, icon_size.y),
                    icon_size,
                ),
                Button::image(
                    Image::new(include_image!("../../assets/icon/screenshot.svg"))
                        .fit_to_exact_size(icon_size)
                        .tint(LIGHT_THEME.surface),
                )
                .frame(false),
            );
            if button.clicked() {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Screenshot(UserData::default()));
                app.transient.taking_screenshot = true;
            }
        },
    );
}

fn transition_dragging(ui: &mut Ui, app: &mut App, graph_rect: Rect) -> Result<(), RitmError> {
    if let Some((source_id, target_id)) = app.ui.graph.drag_transition {
        // If the mouse/pen is released then we check if a transition can be added
        if !ui.input(|r| r.pointer.any_down()) {
            if let Some(target_id) = target_id {
                if app.turing.get_transitions(source_id, target_id).is_err() {
                    app.turing.add_default_transition(source_id, target_id)?;
                }
                app.turing.prepare_transition_edit(source_id, target_id)?;
                app.ui
                    .popup
                    .open(RitmPopupEnum::TransitionEdit(TransitionId {
                        source_id,
                        target_id,
                        id: 0,
                    }));
            }

            app.ui.graph.drag_transition = None;
        }
        // We draw the arrow if still down
        else if let Ok(source) = app.turing.get_state(source_id)
            && let Some(absolute_position) = ui.input(|r| r.pointer.latest_pos())
        {
            let target = if graph_rect.contains(absolute_position) {
                absolute_to_relative(ui.clip_rect(), graph_rect, absolute_position)
            } else {
                absolute_to_relative(
                    ui.clip_rect(),
                    graph_rect,
                    absolute_position.clamp(graph_rect.min, graph_rect.max),
                )
            };

            if Rect::from_center_size(
                source.get_inner().position,
                Vec2::splat(Constant::STATE_RADIUS * 2.0),
            )
            .contains(target)
            {
                let transition_vec = app.turing.best_vector(source_id)?;
                let _ = draw_self_arrow(app, ui, source.get_inner().position, transition_vec);
            } else {
                let _ = draw_arrow(app, ui, source.get_inner().position, target, None);
            }
        }
        state::draw_node(app, ui, source_id)?;
    }

    if let Some((s, _)) = app.ui.graph.drag_transition {
        app.ui.graph.drag_transition = Some((s, None));
    }
    Ok(())
}

fn absolute_to_relative(relative_rect: Rect, absolute_rect: Rect, absolute_position: Pos2) -> Pos2 {
    Pos2::new(
        relative_rect.left()
            + (relative_rect.width() * (absolute_position.x - absolute_rect.left())
                / absolute_rect.width()),
        relative_rect.top()
            + (relative_rect.height() * (absolute_position.y - absolute_rect.top())
                / absolute_rect.height()),
    )
}

fn relative_to_absolute(absolute_rect: Rect, relative_rect: Rect, relative_pos: Pos2) -> Pos2 {
    Pos2::new(
        absolute_rect.left()
            + (absolute_rect.width() * (relative_pos.x - relative_rect.left())
                / relative_rect.width()),
        absolute_rect.top()
            + (absolute_rect.height() * (relative_pos.y - relative_rect.top())
                / relative_rect.height()),
    )
}
