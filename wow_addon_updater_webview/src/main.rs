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
    link: ComponentLink<Model>,
    new_addon_url: String,
    config: Config,
    get_config_task: Option<FetchTask>,
    add_addon_task: Option<FetchTask>,
    save_config_task: Option<FetchTask>,
    update_all_task: Option<FetchTask>,
}

pub enum Msg {
    UpdateNewAddonUrl(String),
    UpdateWowPath(String),
    Add,
    GetConfigReady(Result<Config, Error>),
    AddonAdded,
    Ignore,
    Save,
    UpdateAll,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            console: ConsoleService::new(),
            fetch_service: FetchService::new(),
            link: link,
            new_addon_url: "".into(),
            config: Config::default(),
            get_config_task: None,
            add_addon_task: None,
            save_config_task: None,
            update_all_task: None,
        };
        model.get_config();
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateNewAddonUrl(val) => {
                self.new_addon_url = val;
            }
            Msg::UpdateWowPath(val) => {
                self.config.path = val.parse().unwrap();
            }
            Msg::Add => {
                let post_request = Request::post("/api/add-addon")
                    .body(Ok(self.new_addon_url.to_owned()))
                    .expect("Failed to build request.");

                self.add_addon_task = Some(
                    self.fetch_service.fetch(
                        post_request,
                        self.link
                            .send_back(|_resp: Response<Result<String, Error>>| Msg::AddonAdded),
                    ),
                );
            }
            Msg::AddonAdded => {
                self.get_config();
            }
            Msg::GetConfigReady(response) => {
                if let Ok(data) = response {
                    self.config = data;
                }
            }
            Msg::Save => {
                let post_request = Request::post("/api/config")
                    .header("Content-Type", "application/json")
                    .body(Json(&self.config))
                    .expect("Failed to build request.");

                self.save_config_task = Some(
                    self.fetch_service.fetch(
                        post_request,
                        self.link
                            .send_back(|_resp: Response<Json<Result<Config, Error>>>| {
                                Msg::AddonAdded
                            }),
                    ),
                );
            }
            Msg::UpdateAll => {
                let get_request = Request::get("/api/update")
                    .body(Nothing)
                    .expect("Failed to build request.");

                self.update_all_task = Some(
                    self.fetch_service.fetch(
                        get_request,
                        self.link
                            .send_back(|_resp: Response<Result<String, Error>>| Msg::AddonAdded),
                    ),
                );
            }
            Msg::Ignore => {}
        }
        true
    }
}

impl Model {
    fn get_config(&mut self) {
        let get_request = Request::get("/api/config")
            .body(Nothing)
            .expect("Failed to build request.");

        self.get_config_task = Some(
            self.fetch_service.fetch(
                get_request,
                self.link
                    .send_back(|response: Response<Json<Result<Config, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::GetConfigReady(data)
                        } else {
                            Msg::Ignore
                        }
                    }),
            ),
        );
    }

    fn view_addon(&self, addon: &Addon) -> Html<Model> {
        html! {
            <li class="list-group-item"> { &addon.file_name } </li>
        }
    }

    fn view_add_addon(&self) -> Html<Model> {
        html! {
            <>
            <div class="form-group">
                <label> { "Add wowinterface.com addon link" } </label>
                <input class="form-control" oninput=|e| Msg::UpdateNewAddonUrl(e.value) />
            </div>
            <button class="btn btn-primary" onclick=|_| Msg::Add>{"Add"}</button>
            </>
        }
    }

    fn view_path_chooser(&self) -> Html<Model> {
        let path = self.config.path.as_path().to_str().unwrap();
        html! {
            <>
            <div class="form-group">
                <label> { "Choose your wow path" } </label>
                <input value=path class="form-control" oninput=|e| Msg::UpdateWowPath(e.value) />
            </div>
            <button class="btn btn-primary" onclick=|_| Msg::Save>{"Save Path"}</button>
            </>
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container">
                { self.view_path_chooser() }
                <hr />
                { self.view_add_addon() }
                <hr />
                <button class="btn btn-success" onclick=|_| Msg::UpdateAll>{"Update All"}</button>
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
