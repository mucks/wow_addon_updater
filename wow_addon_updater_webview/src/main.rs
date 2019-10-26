#![recursion_limit = "256"]
use shared::{Addon, Config};
pub use wow_addon_updater_shared as shared;
use yew::format::Json;

use failure::Error;
use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

fn main() {
    yew::start_app::<Model>();
}

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,
    value: i64,
    input_val: String,
    config: Config,
    get_config_task: Option<FetchTask>,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
    UpdateInput(String),
    Add,
    GetConfigReady(Result<Config, Error>),
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            console: ConsoleService::new(),
            fetch_service: FetchService::new(),
            value: 0,
            input_val: "".into(),
            config: Config::default(),
            get_config_task: None,
        };
        let get_request = Request::get("/api/config")
            .body(Nothing)
            .expect("Failed to build request.");

        model.get_config_task = Some(model.fetch_service.fetch(
            get_request,
            link.send_back(|response: Response<Json<Result<Config, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::GetConfigReady(data)
                } else {
                    Msg::Ignore
                }
            }),
        ));

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    self.console.log("Bulk action");
                }
            }
            Msg::UpdateInput(val) => {
                self.input_val = val;
            }
            Msg::Add => {}
            Msg::GetConfigReady(response) => {
                if let Ok(data) = response {
                    self.config = data;
                }
            }
            Msg::Ignore => {}
        }
        true
    }
}

impl Model {
    fn view_addon(&self, addon: &Addon) -> Html<Model> {
        html! {
            <li class="list-group-item"> { &addon.file_name } </li>
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container">
                <div class="form-group">
                    <label> { "Add wowinterface.com addon link" } </label>
                    <input class="form-control" oninput=|e| Msg::UpdateInput(e.value) />
                </div>
                <button class="btn btn-primary" onclick=|_| Msg::Add>{"Add"}</button>
                <hr />
                <ul class="list-group">
                    {
                        for self.config.added.iter().map(|addon| self.view_addon(addon))
                    }
                </ul>
            </div>
        }
    }
}
