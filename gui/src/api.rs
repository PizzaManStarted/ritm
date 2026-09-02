use std::{
    any::Any,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use egui::{Id, Ui, ahash::HashMap};
use poll_promise::Promise;
use reqwest::{
    Client, Error,
    header::{COOKIE, HeaderValue},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    App,
    ui::popup::RitmPopupEnum,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Api {
    #[serde(skip)]
    pub queries: HashMap<Id, Box<dyn Any + 'static>>,
    pub info: Option<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub access_token: String,
    pub id: String,
    pub username: String,
    pub email: String,
}

pub struct ApiHandle<Req: Serialize + Send + 'static, Res: DeserializeOwned + Send + 'static> {
    id: Id,
    req: PhantomData<Req>,
    res: PhantomData<Res>,
}

impl<Req: Serialize + Send + 'static, Res: DeserializeOwned + Send + Debug + 'static>
    ApiHandle<Req, Res>
{
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: Id::new(id.into()),
            req: PhantomData,
            res: PhantomData,
        }
    }

    pub fn on_idle(
        &self,
        ui: &mut Ui,
        app: &mut App,
        action: impl FnOnce(&mut Ui, &mut App),
    ) -> bool {
        if !app.api.queries.contains_key(&self.id) {
            action(ui, app);
            return true;
        }
        false
    }

    pub fn on_wait(
        &self,
        ui: &mut Ui,
        app: &mut App,
        action: impl FnOnce(&mut Ui, &mut App),
    ) -> bool {
        if let Some(query) = app.api.queries.get(&self.id) {
            let promise = query
                .downcast_ref::<Promise<Result<Res, Error>>>()
                .expect("Invalid response struct");

            if promise.ready().is_none() {
                action(ui, app);
                return true;
            }
        }
        false
    }

    pub fn on_result(
        &self,
        ui: &mut Ui,
        app: &mut App,
        action: impl FnOnce(&mut Ui, &mut App, Res),
    ) -> bool {
        if let Some(query) = app.api.queries.get(&self.id)
            && query
                .downcast_ref::<Promise<Result<Res, Error>>>()
                .expect("Invalid response struct")
                .ready()
                .is_some_and(|r| r.is_ok())
        {
            let value = app
                .api
                .queries
                .remove(&self.id)
                .expect("Query exist")
                .downcast::<Promise<Result<Res, Error>>>()
                .expect("Invalid response struct")
                .block_and_take();

            match value {
                Ok(val) => action(ui, app, val),
                Err(err) => app.ui.popup.open(RitmPopupEnum::Error(err.to_string())),
            }

            return true;
        }
        false
    }

    pub fn post(&self, app: &mut App, endpoint: String, body: Req) {
        if let Err(err) = app.api.post::<Req, Res>(&self.id, endpoint, body) {
            println!("{err:?}")
        }
    }

    pub fn get(&self, app: &mut App, endpoint: String) {
        if let Err(err) = app.api.get::<Res>(&self.id, endpoint) {
            println!("{err:?}")
        }
    }

    pub fn is_idle(&self, app: &App) -> bool {
        if !app.api.queries.contains_key(&self.id) {
            return true;
        }
        false
    }

    pub fn is_waiting(&self, app: &App) -> bool {
        app.api.queries.get(&self.id).is_some_and(|query| {
            query
                .downcast_ref::<Promise<Result<Res, Error>>>()
                .expect("Invalid response struct")
                .ready()
                .is_none()
        })
    }

    pub fn has_succeed(&self, app: &mut App) -> Option<Res> {
        if let Some(query) = app.api.queries.get(&self.id)
            && query
                .downcast_ref::<Promise<Result<Res, Error>>>()
                .expect("Invalid response struct")
                .ready()
                .is_some_and(|r| r.is_ok())
        {
            Some(
                app.api
                    .queries
                    .remove(&self.id)
                    .expect("Query exist")
                    .downcast::<Promise<Result<Res, Error>>>()
                    .expect("Invalid response struct")
                    .block_and_take()
                    .expect("Should be ok"),
            )
        } else {
            None
        }
    }

    pub fn has_failed(&self, app: &mut App) -> Option<Error> {
        if let Some(query) = app.api.queries.get(&self.id)
            && query
                .downcast_ref::<Promise<Result<Res, Error>>>()
                .expect("Invalid response struct")
                .ready()
                .is_some_and(|r| r.is_ok())
        {
            Some(
                app.api
                    .queries
                    .remove(&self.id)
                    .expect("Query exist")
                    .downcast::<Promise<Result<Res, Error>>>()
                    .expect("Invalid response struct")
                    .block_and_take()
                    .expect_err("Should have failed"),
            )
        } else {
            None
        }
    }
}

impl Api {
    const URL: &'static str = "http://localhost:5212";

    pub fn signout(&mut self) {
        self.info = None
    }

    fn post<Req, Res>(
        &mut self,
        id: &Id,
        endpoint: impl ToString,
        body: Req,
    ) -> Result<(), reqwest::Error>
    where
        Req: serde::Serialize + Send + 'static,
        Res: serde::de::DeserializeOwned + Send + 'static,
    {
        let endpoint = endpoint.to_string();
        let endpoint_clone = endpoint.clone();
        let token = self
            .info
            .as_ref()
            .map(|x| x.access_token.clone())
            .unwrap_or_default();

        let prom: Promise<Result<Res, Error>> = Promise::spawn_async(async move {
            let res = Client::new()
                .post(format!("{}/{}", Self::URL, endpoint_clone))
                .header(
                    COOKIE,
                    HeaderValue::from_str(format!("access_token={token}").as_str()).unwrap(),
                )
                .json(&body)
                .send()
                .await?
                .json::<Res>()
                .await?;

            Ok::<Res, Error>(res)
        });

        self.queries.insert(*id, Box::new(prom));
        Ok(())
    }

    fn get<Res>(&mut self, id: &Id, endpoint: impl ToString) -> Result<(), reqwest::Error>
    where
        Res: serde::de::DeserializeOwned + 'static + Send,
    {
        let endpoint = endpoint.to_string();
        let endpoint_clone = endpoint.clone();
        let token = self
            .info
            .as_ref()
            .map(|x| x.access_token.clone())
            .unwrap_or_default();

        let prom = Promise::spawn_async(async move {
            let res = Client::new()
                .get(format!("{}/{}", Self::URL, endpoint_clone))
                .header(
                    COOKIE,
                    HeaderValue::from_str(format!("access_token={token}").as_str()).unwrap(),
                )
                .send()
                .await?
                .json::<Res>()
                .await?;

            Ok::<Res, Error>(res)
        });

        self.queries.insert(*id, Box::new(prom));
        Ok(())
    }
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SigninRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SignResponse {
    pub access_token: String,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct TuringMachinesListingResponse {
    #[serde(flatten)]
    pub pages: Pagination,
    #[serde(default)]
    pub items: Vec<TuringMachineListing>,
}

#[derive(Debug, Deserialize)]
pub struct TuringMachineListing {
    pub id: String,
    pub name: String,
    pub description: String,
    pub mode: TuringMachineMode,
    pub author_id: String,
    pub author_name: String,
    pub bookmark_count: usize,
    pub bookmarked: bool,
}

#[derive(Debug, Deserialize)]
pub struct TuringMachineResponse {
    pub name: String,
    pub description: String,
    pub code: String,
    pub author_id: usize,
    pub author_name: String,
    pub bookmark_count: usize,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub enum TuringMachineMode {
    #[default]
    Deterministic,
    NonDeterministic,
}

impl Display for TuringMachineMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
    pub total_count: usize,
}
