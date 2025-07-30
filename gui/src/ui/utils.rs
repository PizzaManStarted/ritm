use eframe::egui::{Pos2, Vec2};

use crate::ui::constant::Constant;

/// Compute the distance between 2 points
pub fn distance(p1: Pos2, p2: Pos2) -> f32 {
    f32::sqrt((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2))
}

/// Compute the repulsion force of the node
pub fn rep_force(p1: Pos2, p2: Pos2) -> f32 {
    let force = Constant::CREP / distance(p1, p2).powi(2);
    if force > Constant::MAX_FORCE {
        Constant::MAX_FORCE * force.signum()
    } else {
        force
    }
}

/// Compute the attraction force of the node
pub fn attract_force(p1: Pos2, p2: Pos2) -> f32 {
    let force = Constant::CSPRING * (distance(p1, p2) / (Constant::L)).log(10.0);
    if force > Constant::MAX_FORCE {
        Constant::MAX_FORCE * force.signum()
    } else {
        force
    }
}

/// Compute the direction between 2 points
pub fn direction(p1: Pos2, p2: Pos2) -> Vec2 {
    Vec2::new(p2.x - p1.x, p2.y - p1.y).normalized()
}
