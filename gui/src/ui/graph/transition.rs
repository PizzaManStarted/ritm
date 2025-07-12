use std::collections::{btree_map::Entry, BTreeMap};

use egui::{epaint::{PathShape, QuadraticBezierShape}, vec2, Align, Color32, Label, Pos2, Rect, RichText, Sense, Stroke, TextEdit, Ui, Vec2};
use crate::{
    turing::{State, Transition}, ui::{constant::Constant, font::Font, utils::{self}}, App
};


pub fn show(app: &mut App, ui: &mut Ui) {

    let mut transitions_hashmap: BTreeMap<(u8, u8), Vec<((u8, u8), bool)>> = BTreeMap::new();
    let mut graph_center = Vec2::ZERO;
    let states_count = app.states.len();

    let keys: Vec<u8> = app.states.keys().map(|u| *u).collect::<Vec<u8>>();


    for i in keys {

        let state = State::get(app, i);
        let transitions_count = state.transitions.len();

        for j in 0..transitions_count {

            let transition = Transition::get(app, (i,j as u8));
            let target_state_index = transition.target_id;

            
            let is_previous = app.step.transition_taken.as_ref().is_some_and(|f| f
                == app
                    .turing
                    .get_state(i)
                    .unwrap()
                    .transitions
                    .get(transition.id as usize)
                    .unwrap());

            match transitions_hashmap.entry((i, target_state_index)) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(((i, j as u8), is_previous));
                }
                Entry::Vacant(e) => {
                    e.insert(vec![((i, j as u8), is_previous)]);
                }
            }
        }

        // compute graph center based on node position
        graph_center += state.position.to_vec2();
    }

    graph_center /= states_count as f32;


    for ((from, to), transitions) in transitions_hashmap.iter() {

        draw_transition(
            app,
            ui,
            *from,
            *to,
            graph_center, 
            app.turing.get_transitions_to(*to, *from).is_ok_and(|_| to > from),
            transitions,
        );
    }
}

fn draw_transition(
    app: &mut App,
    ui: &mut Ui,
    source: u8,
    target: u8,
    graph_center: Vec2,
    reverse: bool,
    transitions: &Vec<((u8, u8), bool)>,
) {

    let source_position = State::get(app, source).position;
    let target_position = State::get(app, target).position;
    // compute the center between the 2 states
    let center = vec2((source_position.x + target_position.x) / 2.0, (source_position.y + target_position.y) / 2.0);

    // compute the direction of the curve
    let mut delta = (source_position - target_position).rot90().normalized();
    let need_to_flip = utils::distance((center + delta).to_pos2(), graph_center.to_pos2()) < utils::distance((center - delta).to_pos2(), graph_center.to_pos2());
    // trust me bro, it's a xor operation
    delta = if reverse != need_to_flip { -delta } else { delta };

    // points of the curve
    let points = [
        source_position,
        (center + delta * Constant::TRANSITION_CURVATURE * 2.0).to_pos2(),
        target_position
    ];

    // draw the bezier
    ui.painter().add(QuadraticBezierShape::from_points_stroke(
        points, 
        false,
        Color32::TRANSPARENT,
    Stroke::new(Constant::TRANSITION_THICKNESS, Color32::BLACK)));

    // compute the curve lenght to find where to draw the triangle
    let curve_lenght = get_quadratic_len(points, 100);
    let arrow_position = quadraticbeziercurve(points, map(&curve_lenght, 100, 1.0 - Constant::STATE_RADIUS/curve_lenght.last().unwrap()));
    let arrow_direction = (arrow_position - target_position.to_vec2()).normalized();

    // compute vertices of triangle
    let triangles = vec![
        arrow_position.to_pos2(),
        (arrow_position
            + arrow_direction * Constant::ARROW_SIZE
            + arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0).to_pos2(),
        (arrow_position
            + arrow_direction * Constant::ARROW_SIZE
            - arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0).to_pos2(),
    ];

    // draw triangle
    ui.painter().add(PathShape::convex_polygon(
        triangles, 
        Color32::BLACK, 
        Stroke::NONE
    ));

    let state = app.states.get(&source).unwrap();
    let rules_len = state.transitions[0].text.len();
    let offset = vec2(
        Constant::TRANSITION_CURVATURE + rules_len as f32 * utils::get_width(ui, &Font::default()) / 2.0,
        Constant::TRANSITION_CURVATURE + transitions.len() as f32 * (utils::get_heigth(ui, &Font::default()) - 10.0)
    );

    draw_labels(app, ui, center.to_pos2(), transitions, (center + delta * offset).to_pos2());
    
}

fn draw_self_transition(
    ui: &mut Ui,
    source: Pos2,
    graph_center: Vec2,
    reverse: bool,
) {

}


/// draw the transitions rules as superposed label
fn draw_labels(
    app: &mut App,
    ui: &mut Ui,
    source: Pos2,
    transitions: &Vec<((u8, u8), bool)>,
    position: Pos2,
) {
    let font_height = utils::get_heigth(ui, &Font::default());
    let height_used = transitions.len() as f32 * font_height;

    // enumerate the transition
    let mut i: usize = 0;

    // debug
    // ui.painter().circle(position, 2.0, Color32::CYAN, Stroke::NONE);

    for (identifier, is_previous) in transitions {


        // initialise the rectangle where the text will be
        let max_rect = Rect::from_center_size(
            position + vec2(0.0, -(height_used/2.0 - (i as f32 + 0.5) * font_height)),
            vec2((transitions.len() + 5) as f32 * utils::get_width(ui, &Font::default()), utils::get_heigth(ui, &Font::default())),
        );

        // draw the text or a single line text edit if selected, then return the rect used
        let selected = app.selected_transition.is_some_and(|selected_transition| selected_transition == *identifier);
        let transition = Transition::get_mut(app, *identifier);
        let rect = if selected {

            
            // scope to not affect style of other parts
            let response = ui.scope(|ui| {
                ui.visuals_mut().extreme_bg_color = Color32::TRANSPARENT;
                ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, utils::constrast_color(Color32::WHITE));
                ui.visuals_mut().selection.stroke = Stroke::NONE;

                ui.put(max_rect, TextEdit::singleline(&mut transition.text)
                    .font(Font::default())
                    .text_color(utils::constrast_color(Color32::WHITE))
                    .horizontal_align(Align::Center)
                    .vertical_align(Align::Center)
                )
            }).inner;

            response.request_focus();

            response

        } else {
            let text = RichText::new(&transition.text)
                .font(Font::default())
                .color(Color32::BLACK);

            // draw the text
            ui.put(max_rect, Label::new(text).extend())
        }.rect;

        // add a click listener to the rectangle of the label/textedit
        let response = ui.allocate_rect(rect, Sense::click());

        // if a transition rule is clicked, then we set it as selected
        if response.clicked() {
            app.selected_transition = Some(*identifier);
            app.selected_state = None;
        }

        i += 1;
    }
}


/// return a point on the curve of a quadratic bezier
fn quadraticbeziercurve(points: [Pos2; 3], t: f32) -> Vec2 {
    let x = (1.0 - t).powi(2) * points[0].x
        + 2.0 * (1.0 - t) * t * points[1].x
        + t.powi(2) * points[2].x;
    let y = (1.0 - t).powi(2) * points[0].y
        + 2.0 * (1.0 - t) * t * points[1].y
        + t.powi(2) * points[2].y;
    Vec2::new(x, y)
}



/// return a point on the curve of a cubic bezier
fn cubicbeziercurve(points: [Pos2; 4], t: f32) -> Vec2 {
    let x = (1.0 - t).powi(3) * points[0].x
        + 3.0 * (1.0 - t).powi(2) * t * points[1].x
        + 3.0 * (1.0 - t) * t.powi(2) * points[2].x
        + t.powi(3) * points[3].x;
    let y = (1.0 - t).powi(3) * points[0].y
        + 3.0 * (1.0 - t).powi(2) * t * points[1].y
        + 3.0 * (1.0 - t) * t.powi(2) * points[2].y
        + t.powi(3) * points[3].y;
    Vec2::new(x, y)
}

fn get_cubic_len(points: [Pos2; 4], n: usize) -> Vec<f32> {
    let mut arc_length: Vec<f32> = vec![0.0; n+1];

    let mut origin = cubicbeziercurve(points, 0.0);
    let mut clen = 0.0;
    for i in 1..n+1 {
        let pos = cubicbeziercurve(points, i as f32 * (1.0/n as f32));
        let delta = origin - pos;
        clen += (delta.x.powi(2) + delta.y.powi(2)).sqrt();
        arc_length[i as usize] = clen;
        origin = pos;
    }

    arc_length
}

fn get_quadratic_len(points: [Pos2; 3], n: usize) -> Vec<f32> {
    let mut arc_length: Vec<f32> = vec![0.0; n+1];

    let mut origin = quadraticbeziercurve(points, 0.0);
    let mut clen = 0.0;
    for i in 1..n+1 {
        let pos = quadraticbeziercurve(points, i as f32 / (n+1) as f32);
        let delta = origin - pos;
        clen += (delta.x.powi(2) + delta.y.powi(2)).sqrt();
        arc_length[i as usize] = clen;
        origin = pos;
    }

    arc_length
}


fn map(len: &Vec<f32>, n: usize, t: f32) -> f32 {
    let target = t * len[n-1];
    let mut low = 0;
    let mut high = n;
    let mut i = 0;

    while low < high {
        i = low + ((high - low) / 2 | 0);

        if len[i] < target {
            low = i + 1;
        } else {
            high = i;
        }
    }

    if len[i] > target {
        i -= 1;
    }

    let before = len[i];

    if before == target {
        i as f32 / n as f32
    } else {
        (i as f32 + (target - before) as f32 / (len[i + 1] - before)) / n as f32
    }
}