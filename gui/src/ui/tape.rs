use egui::{
    Align, Color32, Frame, Label, Layout, Rect, Response, RichText, ScrollArea, Sense, Stroke,
    StrokeKind, Ui, Vec2,
    epaint::PathShape,
    pos2,
    scroll_area::{ScrollBarVisibility, ScrollSource},
    vec2,
};

use crate::{
    App,
    ui::theme::LIGHT_THEME,
    utils::{constant::Constant, effect::Fade},
};

pub fn show(app: &mut App, ui: &mut Ui) {
    let tapes_count = app.turing.tm.graph_ref().get_k() + 1;

    // Apply a scale correction to element for small screen
    let square_size = Constant::SQUARE_SIZE;
    let horizontal_space = Constant::HORIZONTAL_SPACING;
    let vertical_space = Constant::VERTICAL_SPACING;
    let scale = 1.0;

    // Tapes frame
    let res = Frame::new().show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = vertical_space;

        // Get the absolute center of the ribbons layout
        let center = ui.max_rect().center().x;

        // Compute how many square will be visible
        // Size + Spacing / Square + Spacing
        let mut square_count = ((ui.available_width() + horizontal_space)
            / (horizontal_space + square_size)) as usize
            + 2;

        // Ensure the count is odd because there is always a square in the center
        if square_count.is_multiple_of(2) {
            square_count += 1
        }

        // Compute the final width of the ribbons
        // #Square * Square + (#Square - 1) * Spacing
        let ribbon_width =
            square_count as f32 * (square_size + horizontal_space) - horizontal_space;

        // ScrollArea to scroll when too many tapes
        // Notes: This may be overkill but better too many than too few
        let content = ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
            .auto_shrink(true)
            .min_scrolled_height(ui.ctx().content_rect().height() / 3.0)
            .max_height(ui.ctx().content_rect().height() / 3.0)
            .show(ui, |ui| {
                // Scroll area to center and display the tapes
                ScrollArea::horizontal()
                    .scroll_source(ScrollSource::NONE)
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .horizontal_scroll_offset(
                        3.0 // 3.0 is the margin of the center square
                        + (ribbon_width - ui.available_width()) / 2.0, // center the tapes
                    ) // this offset center the symbol
                    .show(ui, |ui| {
                        // Draw each tape
                        for i in 0..tapes_count {
                            // Get the top of the current tape to draw the arrow
                            let top = ui.cursor().top();

                            // Draw the tape
                            tape(app, ui, square_count, i);

                            // Draw the arrow on top of the tapes
                            ui.painter().add(PathShape::convex_polygon(
                                vec![
                                    (center - 9.0 * scale, top).into(),
                                    (center + 9.0 * scale, top).into(),
                                    (center, top + 12.0 * scale).into(),
                                ],
                                LIGHT_THEME.border,
                                Stroke::NONE,
                            ));
                        }
                    });
            });

        (
            content.content_size.y,
            content.state.offset.y >= content.content_size.y - content.inner_rect.height(),
        )
    });

    // Add a fade effect on the side
    if !res.inner.1 && res.inner.0 >= ui.ctx().content_rect().height() / 3.0 {
        let fade_rect = Rect::from_min_max(
            pos2(res.response.rect.min.x, res.response.rect.max.y),
            res.response.rect.max,
        );
        // println!("{}", fade_rect);
        ui.put(
            fade_rect,
            Fade::new()
                .with_color(LIGHT_THEME.secondary, 0.0)
                .with_color(Color32::TRANSPARENT, 1.0)
                .with_step(50)
                .with_direction(egui::Direction::BottomUp),
        );
    }
}

/// Draw a tapes with the correct spacing and character
fn tape(app: &mut App, ui: &mut Ui, square_count: usize, tape_id: usize) -> Response {
    let horizontal_space = Constant::HORIZONTAL_SPACING;
    let square_size = Constant::SQUARE_SIZE;

    ui.allocate_ui_with_layout(
        vec2(0.0, square_size + 6.0),
        Layout::left_to_right(Align::Center)
            .with_cross_justify(false)
            .with_cross_align(Align::Center),
        |ui| {
            ui.style_mut().spacing.item_spacing = (horizontal_space, 0.0).into();

            // Compute the square count
            // let square_count: usize =
            //     ((width + horizontal_space) / (horizontal_space + square_size)) as usize + 2;

            // Get the chars and pointer from reading or writing tapes
            let tape = &app.turing.current_step.get_tapes()[tape_id];
            let (chars, pointer): (&Vec<char>, i32) =
                (tape.get_contents(), tape.get_pointer() as i32);

            // Create a vector with the character that are needed
            let tape_center = square_count as i32 / 2;
            let mut tape_vec = vec![' '; (tape_center - pointer).max(0) as usize];
            tape_vec.append(
                &mut chars[(pointer - tape_center).max(0) as usize
                    ..(pointer + tape_center + 1).min(chars.len() as i32) as usize]
                    .to_vec(),
            );

            tape_vec.append(&mut vec![
                ' ';
                (tape_center - (chars.len() as i32 - pointer - 1)).max(0)
                    as usize
            ]);

            for (i, char) in tape_vec.iter().enumerate().take(square_count) {
                square(app, ui, *char, i == tape_center as usize);
            }
        },
    )
    .response
}

/// Draw a single square with a character
fn square(_app: &mut App, ui: &mut Ui, character: char, is_current: bool) {
    // Apply a scale correction to element for small screen
    let square_size = Constant::SQUARE_SIZE;

    let size = square_size + if is_current { 6.0 } else { 0.0 };
    let (rect, _res) = ui.allocate_exact_size(Vec2::splat(size), Sense::empty());

    // Draw the square, with a border if center one
    ui.painter().rect(
        rect,
        Constant::SQUARE_CORNER,
        if character == ' ' {
            LIGHT_THEME.disabled
        } else {
            LIGHT_THEME.surface
        },
        if is_current {
            Stroke::new(3.0, LIGHT_THEME.border)
        } else {
            Stroke::new(1.0, LIGHT_THEME.border)
        },
        StrokeKind::Inside,
    );

    // Add the character into the frame
    ui.put(
        rect,
        Label::new(
            RichText::new(character)
                .size(square_size / 2.0)
                .color(LIGHT_THEME.text_primary),
        ),
    );
}