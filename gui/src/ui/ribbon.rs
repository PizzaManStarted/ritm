use std::i32;

use egui::{
    Color32, Frame, Label, Margin, RichText, ScrollArea, Sense, Stroke, Ui, epaint::PathShape,
    scroll_area::ScrollBarVisibility,
};
use ritm_core::turing_ribbon::TuringRibbon;

use crate::App;

pub fn show(app: &mut App, ui: &mut Ui) {
    // TODO replace by TuringMachine value
    let ribbon_count = app.turing.get_k();

    // Ribbons frame
    Frame::new()
        .inner_margin(Margin::same(3))
        .outer_margin(Margin::same(0))
        .fill(app.theme.color3)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = (0.0, 5.0).into();

            // Get the center of the ribbons layout
            let center = ui.available_rect_before_wrap().left() + ui.available_width() / 2.0;
            let mut square_count = ((ui.available_width() + 5.0) / (5.0 + 50.0)) as usize;
            if square_count % 2 == 0 {
                square_count += 1
            }
            let ribbon_size = square_count as f32 * (50.0 + 5.0) - 5.0;

            // Scroll area to center and display the ribbon
            ScrollArea::horizontal()
                .enable_scrolling(false)
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .horizontal_scroll_offset(55.0 + (ribbon_size - ui.available_width()) / 2.0)// this offset center the symbol
                .show(ui, |ui| {
                    // Draw each ribbon
                    for i in 0..ribbon_count {
                        // Get the top of the current ribbon to draw the arrow
                        let top = ui.available_rect_before_wrap().top() - 2.0;

                        // Draw the ribbon
                        ribbon(app, ui, i);

                        // Draw the arrow on top of the ribbon
                        ui.painter().add(PathShape::convex_polygon(
                            vec![
                                (center - 9.0, top).into(),
                                (center + 9.0, top).into(),
                                (center, top + 12.0).into(),
                            ],
                            Color32::from_gray(100),
                            Stroke::NONE,
                        ));
                    }
                });
            
        });
}

fn ribbon(app: &mut App, ui: &mut Ui, ribbon_id: u8) {
    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing = (5.0, 0.0).into();

        let square_count: i32 = ((ui.available_width() + 5.0) / (5.0 + 50.0)) as i32 + 2;

        let (chars, pointer): (&Vec<char>, i32) = if ribbon_id == 0 {
            (&app.step.read_ribbon.chars_vec, app.step.read_ribbon.pointer as i32)
        } else {
            let write_ribbon = &app.step.write_ribbons[ribbon_id as usize];
            (&write_ribbon.chars_vec, write_ribbon.pointer as i32)
        };

        let ribbon_center = square_count/2;
        let mut ribbon_vec = vec![' '; (ribbon_center - pointer).max(0) as usize];
        ribbon_vec.append(&mut chars[(pointer-ribbon_center).max(0) as usize..(pointer+ribbon_center).min(chars.len() as i32) as usize].to_vec());
        ribbon_vec.append(&mut vec![' '; (ribbon_center-(chars.len() as i32-pointer-1)).max(0)as usize]);

        for i in 0..square_count {
            square(app, ui, ribbon_vec[i as usize]);
        }
    });
}

fn square(app: &mut App, ui: &mut Ui, character: char) {

    Frame::new()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(0.0, Color32::from_gray(191)))
        .corner_radius(2.0)
        .show(ui, |ui| {
            let (rect, _res) = ui.allocate_exact_size((50.0, 50.0).into(), Sense::empty());

            ui.put(
                rect,
                Label::new(RichText::new(character).size(25.0).color(Color32::BLACK)),
            );
        });
}
