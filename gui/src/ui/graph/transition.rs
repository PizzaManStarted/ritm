use std::{collections::{btree_map::Entry, BTreeMap, HashMap, HashSet}, f32::{self, consts::PI}};

use egui::{emath::Rot2, epaint::{CubicBezierShape, PathShape, QuadraticBezierShape, TextShape}, text::LayoutJob, vec2, Align2, Color32, Pos2, Rect, Sense, Stroke, TextFormat, Ui, Vec2};
use ritm_core::turing_machine::Ribbon;
use crate::{
    turing::{State, Transition, TransitionId}, ui::{constant::Constant, font::Font, utils::{self}}, App
};

/// Draw every transition of the turing machine
pub fn show(app: &mut App, ui: &mut Ui) {

    // We use this binary tree mapping structure to group every transition by source and target state
    let mut transitions_hashmap: BTreeMap<(usize, usize), Vec<(TransitionId, bool)>> = BTreeMap::new();

    // Used to compute the center of every state position
    let mut graph_center = Vec2::ZERO;

    let mut neighbors: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Extract each keys to avoid borrowing the whole App struct when iterating every state
    let keys: Vec<usize> = app.states.keys().map(|u| *u).collect::<Vec<usize>>();

    for i in keys {

        let state = State::get(app, i);
        let transitions_count = state.transitions.len();


        for j in 0..transitions_count {

            let transition = Transition::get(app, (i,j));
            let target_state_index = transition.target_id;

            if target_state_index != i {
                neighbors.entry(i).or_insert(HashSet::from([target_state_index])).insert(target_state_index);
                neighbors.entry(target_state_index).or_insert(HashSet::from([i])).insert(i); 
            }

            // check if the current transition has been used to get to the current state
            let is_previous = app.step.0.get_transition_taken().is_some_and(|f| f
                == app
                    .turing
                    .graph().get_state(i as usize)
                    .unwrap()
                    .transitions
                    .get(transition.id as usize)
                    .unwrap());

            match transitions_hashmap.entry((i, target_state_index)) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(((i, j), is_previous));
                }
                Entry::Vacant(e) => {
                    e.insert(vec![((i, j), is_previous)]);
                }
            }
        }

        // add every state position for later computation
        graph_center += state.position.to_vec2();
    }


    // compute the center of the graph by computing the mean of every state position
    graph_center /= app.states.len() as f32;


    for ((from, to), transitions) in transitions_hashmap.iter() {

        if from == to {
            draw_self_transition(
                app,
                ui, 
                *from,
                neighbors.get(from).unwrap().iter()
                    .fold(Vec2::ZERO, |acc, e| acc + (State::get(app, *e).position - State::get(app, *from).position).normalized()),
                transitions
            );
        } else {
            draw_transition(
                app,
                ui,
                *from,
                *to,
                graph_center, 
                app.turing.graph().get_transitions_by_index(*to, *from).is_ok_and(|x| !x.is_empty() && from > to),
                transitions,
            );
        }
    }
}

/// Draw transition between 2 different state
fn draw_transition(
    app: &mut App,
    ui: &mut Ui,
    source: usize,
    target: usize,
    graph_center: Vec2,
    reverse: bool,
    transitions: &Vec<(TransitionId, bool)>,
) {

    let source_position = State::get(app, source).position;
    let target_position = State::get(app, target).position;
    // compute the center between the 2 states
    let center = Pos2::new((source_position.x + target_position.x) / 2.0, (source_position.y + target_position.y) / 2.0);

    // compute the direction of the curve
    let mut delta = (source_position - target_position).rot90().normalized();
    let need_to_flip = utils::distance(center + delta, graph_center.to_pos2()) < utils::distance(center - delta, graph_center.to_pos2());
    // trust me bro, it's a xor operation
    delta = if reverse != need_to_flip { -delta } else { delta };

    // points of the curve
    let points = [
        source_position,
        (center + delta * Constant::TRANSITION_CURVATURE * 2.0),
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


    draw_labels(app, ui, transitions, (center , delta * Constant::TRANSITION_CURVATURE));
    
}

/// Draw self-transition, aka with the target being the source
fn draw_self_transition(
    app: &mut App,
    ui: &mut Ui,
    state_id: usize,
    transition_vec: Vec2,
    transitions: &Vec<(TransitionId, bool)>
) {

    let state_position = State::get(app, state_id).position;
    // normalize the delta
    let delta = - transition_vec.normalized();

    let size = Constant::SELF_TRANSITION_SIZE * 1.33;
    let points = [
        state_position,
        state_position + (delta * size) + (delta.rot90() * size * 0.69),
        state_position + (delta * size) - (delta.rot90() * size * 0.69),
        state_position,
    ];

    ui.painter().add(CubicBezierShape::from_points_stroke(
        points, 
        false, 
        Color32::TRANSPARENT, 
        Stroke::new(Constant::TRANSITION_THICKNESS, Color32::BLACK)
    ));

    // we get the arrow position on the curve
    let n = 100;
    let curve_lenght = get_cubic_len(points, n);
    let arrow_position = (
        cubicbeziercurve(points, map(&curve_lenght, n, 1.0 - (Constant::STATE_RADIUS - 5.0)/curve_lenght.last().unwrap()) ),
        cubicbeziercurve(points, map(&curve_lenght, n, 1.0 - (Constant::STATE_RADIUS + Constant::ARROW_SIZE/2.0 - 5.0)/curve_lenght.last().unwrap()) ),
    );
    let arrow_direction = (arrow_position.1 - arrow_position.0).normalized();

    // points of the triangle
    let triangles = vec![
        arrow_position.0.to_pos2(),
        (arrow_position.1
            + arrow_direction * Constant::ARROW_SIZE
            + arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0)
            .to_pos2(),
        (arrow_position.1 + arrow_direction * Constant::ARROW_SIZE
            - arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0)
            .to_pos2(),
    ];

    // draw the triangle
    ui.painter().add(PathShape::convex_polygon(
        triangles,
        Color32::BLACK,
        Stroke::NONE,
    ));

    // draw the label
    draw_labels(app, ui, transitions, (state_position, delta * Constant::SELF_TRANSITION_SIZE));
}


/// draw the transitions rules as superposed label
fn draw_labels(
    app: &mut App,
    ui: &mut Ui,
    transitions: &Vec<((usize, usize), bool)>,
    placement: (Pos2, Vec2),
) {
    
    // compute the position of the apsis of the curved transition
    let position = placement.0 + placement.1;

    // debug
    // ui.painter().circle(position, 2.0, Color32::RED, Stroke::NONE);

    let sample_transition = Transition::get(app, transitions[0].0);

    let vector = if sample_transition.parent_id != sample_transition.target_id {
        State::get(app, sample_transition.parent_id).position - State::get(app, sample_transition.target_id).position
    } else {
        (State::get(app, sample_transition.parent_id).position - position).rot90()
    };

    let angle = ((vector.angle() + PI/2.0 - 0.00001).rem_euclid(PI)) - PI/2.0;

    let reverse = placement.1.y.is_sign_positive();
    let selected = app.selected_transition.is_some_and(|transitions| transitions == (sample_transition.parent_id, sample_transition.target_id));

    let mut clicked = false;

    // place each rules
    for (i, (identifier, is_previous)) in transitions.iter().enumerate() {

        let transition = Transition::get(app, *identifier);
        let text = format!("{}", &transition.text);

        let job = LayoutJob::single_section(text, TextFormat {
            font_id: Font::default(),
            color: if selected {app.theme.selected} else {app.theme.ribbon},
            underline: if *is_previous {Stroke::new(1.0, app.theme.ribbon)} else {Stroke::NONE},
            ..Default::default()
        });

        let galley = ui.painter().layout_job(job);
        let text_height = galley.size().y;

        let bounding_rect = Rect::from_center_size(Pos2::ZERO, galley.size()).rotate_bb(Rot2::from_angle(angle));

        let row_i = if reverse {i} else {transitions.len() - 1 - i};
        let position_rect = Rect::from_center_size(position + vec2(
            (-text_height * row_i as f32 - text_height/2.0) * angle.sin(),
            (text_height * row_i as f32 + text_height/2.0) * angle.cos()
        ) * if reverse {1.0} else {-1.0}, bounding_rect.size());

        let anchor: Pos2 = 
        if angle.sin().is_sign_positive() && angle.cos().is_sign_positive() {
            Pos2::new(angle.sin() * text_height, 0.0)
        } else if angle.sin().is_sign_positive() && angle.cos().is_sign_negative() {
            Pos2::new(position_rect.width(), -angle.cos() * text_height)
        } else if angle.sin().is_sign_negative() && angle.cos().is_sign_negative() {
            Pos2::new(position_rect.width() - angle.sin() * text_height, position_rect.height())
        } else {
            Pos2::new(0.0,position_rect.height() - angle.cos() * text_height)
        };

        ui.painter_at(position_rect)
            .add(TextShape::new(position_rect.left_top() + anchor.to_vec2(), galley, Color32::BLACK)
                    .with_angle_and_anchor(angle, Align2::LEFT_TOP));

        ui.allocate_rect(position_rect, Sense::click()).clicked().then(|| clicked = true);
    }
    
    if clicked {
        app.selected_transition = Some((sample_transition.parent_id, sample_transition.target_id));
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

    if len[i] > target && i != 0 {
        i -= 1;
    }

    let before = len[i];

    if before == target {
        i as f32 / n as f32
    } else {
        (i as f32 + (target - before) as f32 / (len[i + 1] - before)) / n as f32
    }
}