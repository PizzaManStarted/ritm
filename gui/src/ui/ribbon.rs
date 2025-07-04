use egui::{
    Color32, Frame, Label, Margin, RichText, ScrollArea, Sense, Stroke, Ui, epaint::PathShape,
    scroll_area::ScrollBarVisibility,
};

use crate::App;

pub fn show(app: &mut App, ui: &mut Ui) {
    // TODO replace by TuringMachine value
    let ribbon_count = 4;

    // Ribbons frame
    Frame::new()
        .inner_margin(Margin::same(0))
        .outer_margin(Margin::same(0))
        .fill(Color32::LIGHT_GRAY)
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
                    for _ in 0..ribbon_count {
                        // Get the top of the current ribbon to draw the arrow
                        let top = ui.available_rect_before_wrap().top() - 2.0;

                        // Draw the ribbon
                        ribbon(app, ui);

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

fn ribbon(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing = (5.0, 0.0).into();

        let square_count = ((ui.available_width() + 5.0) / (5.0 + 50.0)) as usize + 2;

        for _ in 0..square_count {
            square(app, ui);
        }
    });
}

fn square(app: &mut App, ui: &mut Ui) {
    let character = '0';

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
