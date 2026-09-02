use egui::{
    Align, Atom, AtomLayout, Button, Color32, FontId, Frame, Image, Label, Layout, Margin, Pos2,
    Rect, Response, RichText, ScrollArea, Separator, Spinner, Stroke, TextWrapMode, Ui, Vec2,
    include_image, vec2,
};
use egui_alignments::Alignable;

use crate::{
    App,
    api::{ApiHandle, TuringMachineListing, TuringMachineResponse, TuringMachinesListingResponse},
    ui::theme::LIGHT_THEME,
    utils::{effect::Fade, font::Font},
};

#[derive(Default, PartialEq)]
enum Space {
    #[default]
    TM,
    Saved,
    My,
}

// #[derive(Default)]
pub struct TSpace {
    current_space: Space,
    _search: String,
    machines_available: usize,
    current_page: usize,
    element_per_page: usize,
    listing: Vec<TuringMachineListing>,
    refresh: bool,
}

impl Default for TSpace {
    fn default() -> Self {
        Self {
            current_page: 0,
            _search: String::new(),
            machines_available: 0,
            current_space: Space::TM,
            element_per_page: 10,
            listing: vec![],
            refresh: true,
        }
    }
}

/// Show the TSPACE
/// The 3 section - TSPACE, Bookmark and My - Only change which machine are visible
pub fn show(ui: &mut Ui, app: &mut App) {
    // The Ids used to identify the requests
    let listing_handle: ApiHandle<(), TuringMachinesListingResponse> = ApiHandle::new("listing");

    // If we just opened the popup try to fetch the turing machine according to the current tab
    if app.ui.popup.just_opened() || app.ui.tspace.refresh {
        listing_handle.get(
            app,
            format!(
                "api/turingmachine?page={}&page_size={}",
                app.ui.tspace.current_page + 1,
                app.ui.tspace.element_per_page
            ),
        );
        app.ui.tspace.refresh = false;
    }

    listing_handle.on_result(ui, app, |_, app, res| {
        app.ui.tspace.machines_available = res.pages.total_count;
        app.ui.tspace.listing = res.items;
    });

    // This vertical layout is necessary to go from bottom-up to top-down
    ui.vertical(|ui| {
        // The tabs section to chose what list we want
        // When a tab is clicked a new request for the corresponding category is made
        Frame::new()
            .stroke(Stroke::new(1.0, LIGHT_THEME.border))
            .show(ui, |ui| {
                // No sapacing
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.columns_const(|[tspace, bookmarked, mymachine]| {
                    // The TSPACE button, for all the turing machines made by the community
                    if tab(
                        tspace,
                        app,
                        "TSpace",
                        app.ui.tspace.current_space == Space::TM,
                    )
                    .clicked()
                    {
                        app.ui.tspace.current_space = Space::TM;
                        app.ui.tspace.listing.clear();
                        listing_handle.get(
                            app,
                            format!(
                                "api/turingmachine?page={}&page_size={}",
                                app.ui.tspace.current_page + 1,
                                app.ui.tspace.element_per_page
                            ),
                        );
                    };

                    // The Bookmarked button, for all the turing machines that the user bookmarked
                    if tab(
                        bookmarked,
                        app,
                        "Bookmarked",
                        app.ui.tspace.current_space == Space::Saved,
                    )
                    .clicked()
                    {
                        app.ui.tspace.current_space = Space::Saved;
                        app.ui.tspace.listing.clear();
                        listing_handle.get(
                            app,
                            format!(
                                "api/turingmachine/bookmarked?page={}&page_size={}",
                                app.ui.tspace.current_page + 1,
                                app.ui.tspace.element_per_page
                            ),
                        );
                    };

                    // The My button, for all the turing machines made by the user
                    if tab(
                        mymachine,
                        app,
                        "My Machines",
                        app.ui.tspace.current_space == Space::My,
                    )
                    .clicked()
                    {
                        app.ui.tspace.current_space = Space::My;
                        app.ui.tspace.listing.clear();
                        listing_handle.get(
                            app,
                            format!(
                                "api/turingmachine/me?page={}&page_size={}",
                                app.ui.tspace.current_page + 1,
                                app.ui.tspace.element_per_page
                            ),
                        );
                    };
                });
            });

        // Search bar and other options
        // TODO

        // The list of machines avalaible
        // If a request is pending, display a spinner, and if no machine is found display a message
        ui.allocate_ui_with_layout(
            ui.available_size(),
            Layout::bottom_up(Align::Center),
            |ui| {
                pages(ui, app);

                // If there is no machines availables we warn the user that none where found
                if !listing_handle.on_wait(ui, app, |ui, _| {
                    // This is executed only if we are waiting
                    Spinner::new().size(30.0).center(ui);
                }) && app.ui.tspace.listing.is_empty()
                {
                    AtomLayout::new((
                        Image::new(include_image!("../../../assets/icon/not_found.svg"))
                            .fit_to_exact_size(Vec2::splat(30.0))
                            .tint(LIGHT_THEME.text_primary),
                        RichText::new("No machines found"),
                    ))
                    .gap(10.0)
                    .wrap_mode(TextWrapMode::Extend)
                    .center(ui);
                } else {
                    ScrollArea::vertical().show(ui, |ui| {
                        Frame::new().inner_margin(5).show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.set_width(ui.available_width());
                                ui.spacing_mut().item_spacing.y = 5.0;

                                if !app.ui.tspace.listing.is_empty() {
                                    machine(ui, app, 0);
                                }

                                ui.visuals_mut().widgets.inactive.bg_stroke.color =
                                    LIGHT_THEME.border;
                                for i in 1..app.ui.tspace.listing.len() {
                                    ui.add(Separator::default().horizontal().spacing(1.0));
                                    machine(ui, app, i);
                                }
                            });
                        });
                    });
                }
            },
        );
    });
}

fn pages(ui: &mut Ui, app: &mut App) {
    let pages_needed = app
        .ui
        .tspace
        .machines_available
        .div_ceil(app.ui.tspace.element_per_page)
        .max(1);

    let current_page = app.ui.tspace.current_page;

    let size = Font::_text_size(ui, FontId::proportional(20.0), "...");

    ui.horizontal(|ui| {
        if pages_needed <= 7 {
            ui.add_space(
                (ui.available_width() - ((size.x + 5.0) * pages_needed as f32 - 5.0)) / 2.0,
            );
            for i in 0..pages_needed {
                // The content
                if ui
                    .add(Button::new(format!("{}", i + 1)).min_size(size))
                    .clicked()
                {
                    app.ui.tspace.current_page = i
                }
            }
        } else {
            ui.add_space((ui.available_width() - ((size.x + 5.0) * 7.0 - 5.0)) / 2.0);
            // Always the first page
            if ui
                .add(Button::new(RichText::new("1")).min_size(size))
                .clicked()
            {
                app.ui.tspace.current_page = 0
            }

            // 3+ mean ..., less we fill
            if current_page >= 3 {
                ui.add(Button::new("..."));
            } else {
                if ui.add(Button::new("2")).clicked() {
                    app.ui.tspace.current_page = 1
                }
                if ui.add(Button::new("3")).clicked() {
                    app.ui.tspace.current_page = 2
                }
            }

            // The central button does nothing
            ui.add(Button::new(format!("{}", current_page + 1)));

            // 3+ mean ..., less we fill
            if pages_needed - current_page - 1 <= 3 {
                ui.add(Button::new("..."));
            } else {
                if ui
                    .add(Button::new(format!("{}", pages_needed - 2)))
                    .clicked()
                {
                    app.ui.tspace.current_page = pages_needed - 3
                }
                if ui
                    .add(Button::new(format!("{}", pages_needed - 3)))
                    .clicked()
                {
                    app.ui.tspace.current_page = pages_needed - 2
                }
            }

            // Always the last page
            if ui.add(Button::new(format!("{pages_needed}"))).clicked() {
                app.ui.tspace.current_page = pages_needed - 1
            }
        }
    });
}

fn tab(ui: &mut Ui, _app: &mut App, text: impl Into<String>, active: bool) -> Response {
    ui.spacing_mut().button_padding = vec2(10.0, 5.0);
    ui.add(
        Button::new((Atom::grow(), RichText::new(text), Atom::grow()))
            .min_size(vec2(ui.available_width(), 0.0))
            .corner_radius(0)
            .fill(if active {
                Color32::TRANSPARENT
            } else {
                Color32::from_black_alpha(25)
            })
            .stroke(Stroke::NONE),
    )
}

fn machine(ui: &mut Ui, app: &mut App, i: usize) {
    let mut frame = Frame::new()
        .inner_margin(Margin::symmetric(10, 0))
        .begin(ui);
    {
        let ui = &mut frame.content_ui;

        ui.spacing_mut().item_spacing.x = 5.0;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 80.0),
            Layout::right_to_left(Align::Center).with_cross_justify(false),
            |ui| {
                ui.set_min_height(80.0);
                ui.set_max_height(80.0);

                // Download button
                download(ui, app, i);

                ui.spacing_mut().item_spacing.y = 5.0;
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(ui.available_width(), 30.0),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = 10.0;
                                let machine = &app.ui.tspace.listing[i];
                                ui.label(
                                    RichText::new(&machine.name)
                                        .font(FontId::proportional(Font::MEDIUM)),
                                );
                                ui.label(
                                    RichText::new(format!("by {}", machine.author_name))
                                        .font(FontId::proportional(Font::SMALL))
                                        .color(LIGHT_THEME.text_secondary),
                                );

                                bookmark(ui, app, i);

                                // if ui.add(Link::new(RichText::new("See more ->").font(FontId::proportional(Font::SMALL)))).clicked() {
                                //     // Open more details
                                // }
                            },
                        );

                        let machine = &app.ui.tspace.listing[i];
                        // println!("{}", ui.available_height());
                        let res = ScrollArea::vertical()
                            .min_scrolled_height(80.0 - ui.min_rect().height())
                            .max_height(80.0 - ui.min_rect().height())
                            .id_salt(&machine.name)
                            .show(ui, |ui| {
                                ui.add(
                                    Label::new(
                                        RichText::new(&machine.description)
                                            .font(FontId::proportional(Font::SMALL)),
                                    )
                                    .wrap(),
                                )
                            });

                        ui.put(
                            Rect::from_points(&[
                                Pos2::new(res.inner_rect.min.x, res.inner_rect.max.y - 15.0),
                                res.inner_rect.max + vec2(0.0, 5.0),
                            ]),
                            Fade::new()
                                .with_step(20)
                                .with_direction(egui::Direction::BottomUp)
                                .with_color(LIGHT_THEME.surface, 0.0)
                                .with_color(Color32::TRANSPARENT, 1.0),
                        );
                    },
                );
            },
        );
    }
    let res = frame.allocate_space(ui);
    if res.hovered() {
        frame.frame.fill = Color32::from_black_alpha(5);
    }
    frame.paint(ui);
}

fn bookmark(ui: &mut Ui, app: &mut App, i: usize) {
    ui.spacing_mut().button_padding = vec2(0.0, 0.0);
    ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    ui.visuals_mut().widgets.active.weak_bg_fill = Color32::TRANSPARENT;
    ui.visuals_mut().widgets.hovered.weak_bg_fill = Color32::from_black_alpha(10);
    let bookmark_handle: ApiHandle<(), ()> =
        ApiHandle::new(format!("bookmark-{}", app.ui.tspace.listing[i].id));

    if bookmark_handle.is_waiting(app) {
        ui.add(Spinner::new().size(30.0));
    };

    if bookmark_handle.is_idle(app)
        && app
            .api
            .info
            .as_ref()
            .is_none_or(|f| f.id != app.ui.tspace.listing[i].author_id)
        && ui
            .add(
                Button::new((
                    Image::new(if app.ui.tspace.listing[i].bookmarked {
                        include_image!("../../../assets/icon/bookmarked.svg")
                    } else {
                        include_image!("../../../assets/icon/bookmark.svg")
                    })
                    .fit_to_exact_size(Vec2::splat(Font::ICON)),
                    RichText::new(format!("{}", app.ui.tspace.listing[i].bookmark_count)),
                ))
                .frame(false),
            )
            .clicked()
    {
        bookmark_handle.post(
            app,
            format!("api/turingmachine/{}/bookmark", app.ui.tspace.listing[i].id),
            (),
        );
    }

    if bookmark_handle.has_succeed(app).is_some() {
        let machine = &mut app.ui.tspace.listing[i];
        machine.bookmarked ^= true;
        if machine.bookmarked {
            machine.bookmark_count += 1
        } else {
            machine.bookmark_count -= 1
        }
    }

    if let Some(err) = bookmark_handle.has_failed(app) {
        println!("Could not update bookmark : {}", err);
    };
}

fn download(ui: &mut Ui, app: &mut App, i: usize) {
    let download_handle: ApiHandle<(), TuringMachineResponse> = ApiHandle::new(format!("download-{i}"));
    Frame::new()
        .inner_margin(Margin::symmetric(5, 10))
        .corner_radius(10.0)
        .fill(LIGHT_THEME.secondary)
        .show(ui, |ui| {
            download_handle.on_idle(ui, app, |ui, app| {
                if ui
                    .add(
                        Button::image(
                            Image::new(include_image!("../../../assets/icon/download.svg"))
                                .fit_to_exact_size(Vec2::splat(Font::ICON))
                                .tint(LIGHT_THEME.surface),
                        )
                        .small()
                        .frame(false),
                    )
                    .clicked()
                {
                    download_handle.get(
                        app,
                        format!("api/turingmachine/{}", app.ui.tspace.listing[i].id),
                    );
                }
            });

            download_handle.on_wait(ui, app, |ui, _| {
                ui.spinner();
            });

            download_handle.on_result(ui, app, |ui, app, res| {
                app.ui.code.new_tab(res.name, res.code);
                app.ui.popup.close();
            });
        });
}
