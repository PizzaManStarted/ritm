

/// Constant used in the application
pub struct Constant {}

impl Constant {

    // Graph
    pub const CREP: f32 = 30000.0;
    pub const CSPRING: f32 = 50.0;
    pub const L: f32 = 250.0;
    pub const MAX_FORCE: f32 = 10.0;
    pub const STATE_RADIUS: f32 = 40.0;
    pub const TRANSITION_THICKNESS: f32 = 1.0;
    pub const STABILITY_TRESHOLD: f32 = 0.001;
    pub const ARROW_SIZE: f32 = 10.0;
    pub const TRANSITION_CURVATURE: f32 = 20.0;

    // Ribbon
    pub const SQUARE_SIZE: f32 = 50.0;
    pub const VERTICAL_SPACE: f32 = 5.0;
    pub const HORIZONTAL_SPACE: f32 = 5.0;
}