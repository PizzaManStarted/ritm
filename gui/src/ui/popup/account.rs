use std::mem::take;

use egui::{
    Align, Button, CornerRadius, Frame, Grid, Layout, Margin, Response, RichText, Spinner, Stroke,
    TextEdit, TextWrapMode, Ui, vec2,
};

use crate::{
    App,
    api::{ApiHandle, SignResponse, SigninRequest, SignupRequest, UserData},
    ui::{popup::RitmPopupEnum, theme::LIGHT_THEME},
};

#[derive(Default)]
pub struct Account {
    pub signin: SigninRequest,
    pub signup: SignupRequest,
}

impl Account {
    pub fn clear_sign_in(&mut self) {
        self.signin = SigninRequest::default()
    }

    pub fn clear_sign_up(&mut self) {
        self.signup = SignupRequest::default()
    }
}

pub fn logged_in(ui: &mut Ui, app: &mut App, info: UserData) {
    ui.vertical_centered(|ui| {
        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            Frame {
                stroke: Stroke::new(1.0, LIGHT_THEME.border),
                inner_margin: Margin::same(10),
                corner_radius: CornerRadius::same(10),
                ..Default::default()
            }
            .show(ui, |ui| {
                Grid::new("informations").num_columns(2).show(ui, |ui| {
                    ui.label(RichText::new("Username"));
                    ui.label(RichText::new(info.username.to_string()));
                    ui.end_row();

                    ui.label(RichText::new("Email"));
                    ui.label(RichText::new(info.email.to_string()));
                    ui.end_row();
                });
            });
            ui.label(RichText::new("Informations"));
        });

        ui.label(RichText::new("Actions"));
        Frame::new()
            .inner_margin(Margin::symmetric(50, 0))
            .show(ui, |ui| {
                if popup_button(ui, "Sign out".to_string()).clicked() {
                    app.api.signout();
                    // app.ui.popup.switch_to(RitmPopupEnum:: Account);
                }
                if popup_button(ui, "Delete account".to_string()).clicked() {
                    app.ui.popup.open(RitmPopupEnum::Confirmation);
                }
            });
    });
}

pub fn logged_of(ui: &mut Ui, app: &mut App) {
    ui.set_min_width(ui.content_rect().width() / 3.0);
    Frame::new().inner_margin(Margin::same(50)).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            if popup_button(ui, "Sign in".to_string()).clicked() {
                app.ui.popup.open(RitmPopupEnum::SignIn);
            }

            ui.label("or");

            if popup_button(ui, "Sign up".to_string()).clicked() {
                app.ui.popup.open(RitmPopupEnum::SignUp);
            }
        });
    });
}

pub fn signin(ui: &mut Ui, app: &mut App) {
    ui.vertical_centered(|ui| {
        field(
            ui,
            &mut app.ui.account.signin.username_or_email,
            "Email or Username".to_string(),
            false,
        );
        field(
            ui,
            &mut app.ui.account.signin.password,
            "Password".to_string(),
            true,
        );

        let signin_handle: ApiHandle<SigninRequest, SignResponse> = ApiHandle::new("signin");

        signin_handle.on_idle(ui, app, |ui, app| {
            if popup_button(ui, "Sign in".to_string()).clicked() {
                let req = take(&mut app.ui.account.signin);
                signin_handle.post(app, "signin".to_string(), req);
            }
        });

        signin_handle.on_wait(ui, app, |ui, app| {
            ui.add(Spinner::new().size(30.0));
        });

        signin_handle.on_result(ui, app, |_ui, app, res| {
            app.api.info = Some(UserData {
                id: res.user.id,
                access_token: res.access_token,
                username: res.user.username,
                email: res.user.email,
            });
            app.ui.popup.close()
        });
    });
}

pub fn signup(ui: &mut Ui, app: &mut App) {
    ui.vertical_centered(|ui| {
        field(
            ui,
            &mut app.ui.account.signup.username,
            "Username".to_string(),
            false,
        );
        field(
            ui,
            &mut app.ui.account.signup.email,
            "Email".to_string(),
            false,
        );
        field(
            ui,
            &mut app.ui.account.signup.password,
            "Password".to_string(),
            true,
        );

        let signup_handle: ApiHandle<SignupRequest, SignResponse> = ApiHandle::new("signup");

        signup_handle.on_idle(ui, app, |ui, app| {
            if popup_button(ui, "Sign up".to_string()).clicked() {
                let req = take(&mut app.ui.account.signup);
                signup_handle.post(app, "signup".to_string(), req);
            }
        });

        signup_handle.on_wait(ui, app, |ui, _app| {
            ui.add(Spinner::new().size(30.0));
        });

        signup_handle.on_result(ui, app, |_ui, app, res| {
            app.api.info = Some(UserData {
                id: res.user.id,
                access_token: res.access_token,
                username: res.user.username,
                email: res.user.email,
            });
            app.ui.popup.close()
        });
    });
}

fn popup_button(ui: &mut Ui, text: String) -> Response {
    ui.spacing_mut().button_padding = vec2(10.0, 5.0);
    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(1.0, LIGHT_THEME.border);
    ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, LIGHT_THEME.border);
    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.0, LIGHT_THEME.border);
    ui.add(
        Button::new(text)
            .corner_radius(10)
            .wrap_mode(TextWrapMode::Extend)
            .min_size(vec2(ui.available_width(), 0.0)),
    )
}

fn field(ui: &mut Ui, text: &mut String, placeholder: String, password: bool) {
    TextEdit::singleline(text)
        .hint_text(placeholder)
        .password(password)
        .show(ui);
}
