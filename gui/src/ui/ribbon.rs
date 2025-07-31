use std::i32;

use egui::{
    epaint::PathShape, scroll_area::{ScrollBarVisibility, ScrollSource}, vec2, Align, Frame, Label, Layout, Margin, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui
};
use ritm_core::turing_ribbon::TuringRibbon;

use crate::{
    App,
    ui::constant::Constant,
};

pub fn show(app: &mut App, ui: &mut Ui) {
    let ribbon_count = app.turing.graph_ref().get_k() + 1;

    let square_size = Constant::scale(ui, Constant::SQUARE_SIZE);

    // Ribbons frame
    Frame::new()
        .inner_margin(Margin::same(3))
        .outer_margin(Margin::same(0))
        .fill(app.theme.ribbon)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = (0.0, Constant::VERTICAL_SPACE).into();

            // Get the center of the ribbons layout
            let center = ui.available_rect_before_wrap().left() + ui.available_width() / 2.0;
            let mut square_count = ((ui.available_width() + Constant::HORIZONTAL_SPACE)
                / (Constant::HORIZONTAL_SPACE + square_size))
                as usize;
            if square_count % 2 == 0 {
                square_count += 1
            }
            let ribbon_size = square_count as f32
                * (square_size + Constant::HORIZONTAL_SPACE)
                - Constant::HORIZONTAL_SPACE;

            // Scroll area to center and display the ribbon
            ScrollArea::horizontal()
                .scroll_source(ScrollSource::NONE)
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .horizontal_scroll_offset(3.0 + // 3.0 is the margin of the center square
                    square_size
                        + Constant::HORIZONTAL_SPACE
                        + (ribbon_size - ui.available_width()) / 2.0,
                ) // this offset center the symbol
                .show(ui, |ui| {
                    let width = ui.available_width();
                    // Draw each ribbon
                    for i in 0..ribbon_count {
                        // Get the top of the current ribbon to draw the arrow
                        let top = ui.available_rect_before_wrap().top();

                        // Draw the ribbon
                        ribbon(app, ui, width, i);

                        // Draw the arrow on top of the ribbon
                        ui.painter().add(PathShape::convex_polygon(
                            vec![
                                (center - 9.0, top).into(),
                                (center + 9.0, top).into(),
                                (center, top + 12.0).into(),
                            ],
                            app.theme.gray,
                            Stroke::NONE,
                        ));
                    }
                });
        });
}

/// Draw a ribbon with the correct spacing and character
fn ribbon(app: &mut App, ui: &mut Ui, width: f32, ribbon_id: usize) {
    let square_size = Constant::scale(ui, Constant::SQUARE_SIZE);
    ui.allocate_ui_with_layout(
        vec2(0.0, square_size + 6.0),
        Layout::left_to_right(Align::Center)
            .with_cross_justify(false)
            .with_cross_align(Align::Center),
        |ui| {
            ui.style_mut().spacing.item_spacing = (Constant::HORIZONTAL_SPACE, 0.0).into();

            let square_count: i32 = ((width + Constant::HORIZONTAL_SPACE)
                / (Constant::HORIZONTAL_SPACE + square_size))
                as i32
                + 2;

            // Get the chars and pointer from reading or writing ribbon
            let (chars, pointer): (&Vec<char>, i32) = if ribbon_id == 0 {
                (
                    &app.step.get_reading_ribbon().get_contents(),
                    app.step.get_reading_ribbon().get_pointer() as i32,
                )
            } else {
                let write_ribbon = &app.step.get_writting_ribbons()[ribbon_id - 1 as usize];
                (&write_ribbon.get_contents(), write_ribbon.get_pointer() as i32)
            };

            // Create a vector with the character that are needed
            let ribbon_center = square_count / 2;
            let mut ribbon_vec = vec![' '; (ribbon_center - pointer).max(0) as usize];
            ribbon_vec.append(
                &mut chars[(pointer - ribbon_center).max(0) as usize
                    ..(pointer + ribbon_center + 1).min(chars.len() as i32) as usize]
                    .to_vec(),
            );
            ribbon_vec.append(&mut vec![
                ' ';
                (ribbon_center - (chars.len() as i32 - pointer - 1)).max(0)
                    as usize
            ]);

            for i in 0..square_count {
                square(
                    app,
                    ui,
                    ribbon_vec[i as usize],
                    i == ribbon_center,
                );
            }
        },
    );
}

// Draw a single square with the character wanted
fn square(app: &mut App, ui: &mut Ui, character: char, is_current: bool) {
    let square_size = Constant::scale(ui, Constant::SQUARE_SIZE);
    Frame::new().show(ui, |ui| {
        let size = square_size + if is_current {6.0} else {0.0};
        let (rect, _res) = ui.allocate_exact_size((size, size).into(), Sense::empty());

        ui.painter().rect(
            rect,
            Constant::SQUARE_CORNER,
            app.theme.white,
            if is_current {
                Stroke::new(3.0, app.theme.gray)
            } else {
                Stroke::NONE
            },
            StrokeKind::Inside
        );
        ui.put(
            rect,
            Label::new(
                RichText::new(character)
                    .size(square_size / 2.0)
                    .color(app.theme.gray),
            ),
        );
    });
}
