use egui::{
    Color32,
    Direction::{self, LeftToRight},
    Mesh, Rect, Ui, Widget, pos2,
};
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct ColorKey {
    color: Color32,
    key: f32,
}

impl Eq for ColorKey {}

impl PartialEq for ColorKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl PartialOrd for ColorKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColorKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.key.total_cmp(&self.key)
    }
}

pub struct Fade {
    color: BinaryHeap<ColorKey>,
    direction: Direction,
    step: usize,
    min: f32,
    max: f32,
}

impl Fade {
    pub fn new() -> Self {
        Self {
            color: BinaryHeap::new(),
            direction: LeftToRight,
            step: 100,
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub fn with_color(mut self, color: Color32, key: f32) -> Self {
        self.color.push(ColorKey { color, key });
        if key < self.min {
            self.min = key
        }
        if key > self.max {
            self.max = key
        }
        self
    }

    pub fn with_step(mut self, step: usize) -> Self {
        self.step = step;
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn fade(&self, ui: &mut Ui, rect: Rect) {
        let mut mesh = Mesh::default();

        let step_size = match self.direction {
            Direction::LeftToRight | Direction::RightToLeft => rect.width() / self.step as f32,
            Direction::TopDown | Direction::BottomUp => rect.height() / self.step as f32,
        };
        let mut old_max = match self.direction {
            Direction::LeftToRight => rect.min.x,
            Direction::RightToLeft => rect.max.x,
            Direction::TopDown => rect.min.y,
            Direction::BottomUp => rect.max.y,
        };
        let sorted = self.color.clone().into_sorted_vec();
        let mut colors = sorted
            .iter()
            .zip(sorted.iter().skip(1))
            .map(|(c1, c2)| (c2, c1))
            .collect::<Vec<(&ColorKey, &ColorKey)>>();
        let mut color = colors.pop().expect("exist");

        let mut i = 0;
        while i < self.step {
            let r = match self.direction {
                Direction::LeftToRight => {
                    let r = Rect::from_min_max(
                        pos2(old_max, rect.min.y),
                        pos2(old_max + step_size, rect.max.y),
                    );
                    old_max += step_size;
                    r
                }
                Direction::RightToLeft => {
                    let r = Rect::from_min_max(
                        pos2(old_max - step_size, rect.min.y),
                        pos2(old_max, rect.max.y),
                    );
                    old_max -= step_size;
                    r
                }
                Direction::TopDown => {
                    let r = Rect::from_min_max(
                        pos2(rect.min.x, old_max),
                        pos2(rect.max.x, old_max + step_size),
                    );
                    old_max += step_size;
                    r
                }
                Direction::BottomUp => {
                    let r = Rect::from_min_max(
                        pos2(rect.min.x, old_max - step_size),
                        pos2(rect.max.x, old_max),
                    );
                    old_max -= step_size;
                    r
                }
            };
            let x = self.min + ((self.max - self.min) * i as f32 / self.step as f32);
            let t = (x - color.0.key) / (color.1.key - color.0.key);
            mesh.add_colored_rect(r, color.0.color.lerp_to_gamma(color.1.color, t));

            if x >= color.1.key {
                color = colors.pop().expect("color should exist");
            }
            i += 1;
        }
        ui.painter().add(mesh);
    }
}

impl Widget for Fade {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        self.fade(ui, ui.available_rect_before_wrap());
        ui.response()
    }
}