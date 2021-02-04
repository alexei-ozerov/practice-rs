#![recursion_limit = "512"]

use anyhow::Error;
use log;
use wasm_bindgen::prelude::*;
use wasm_logger;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use serde::{Serialize, Deserialize};

// TODO: Figure out a way to make deserialization dynamic as the response will be differenly sized based on the amount of data in DB
#[derive(Serialize, Deserialize, Debug, Default)]
struct Data {
    Date_1: String,
    Date_2: String,
    Date_3: String,
    Goal_1: String,
    Goal_2: String,
    Goal_3: String,
    Title_1: String,
    Title_2: String,
    Title_3: String,
}

struct Model {
    link: ComponentLink<Self>,
    title: String,
    value: i64,
    data: Data,
    task: Option<FetchTask>,
}

enum Msg {
    AddOne,
    AddSix,
    Reset,
    Request,
    FetchResourceComplete(Data),
    FetchResourceFailed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            title: "Practice Journal".to_string(),
            data: Data::default(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::AddSix => self.value += 6,
            Msg::Reset => self.value = 0,
            Msg::Request => {
                {
                    let get_request = Request::builder()
                        .method("GET")
                        .uri("http://127.0.0.1:3000/recent")
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Access-Control-Allow-Headers", "*")
                        .body(Nothing)
                        .unwrap();

                    let callback =
                        self.link
                            .callback(|response: Response<Json<Result<Data, Error>>>| {
                                log::info!("{:#?}", &response);
                                if let (meta, Json(Ok(body))) = response.into_parts() {
                                    if meta.status.is_success() {
                                        // self.data = serde_json::to_string(&body).unwrap().clone();
                                        return Msg::FetchResourceComplete(body);
                                    }
                                }
                                Msg::FetchResourceFailed
                            });

                    let task = FetchService::fetch(get_request, callback);
                    self.task = Some(task.unwrap());
                };
            }
            Msg::FetchResourceComplete(body) => self.data = body,
            _ => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ &self.title }</h1>
                <p class="entry">{ "Welcome. Please choose an option from below to get started. You're doing great." }</p>
                <table class="attr">
                    <tr>
                        <td><button onclick=self.link.callback(|_| Msg::Request)>{ "View Recent Entries" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Reset)>{ "Reset Counter" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add One" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::AddSix)>{ "Add Six" }</button></td>
                    </tr>
                </table>
                <p class="count">{ self.value }</p>
                <br/>
                <table class="attr">
                    <tr>
                        <td>{"Title"}</td>
                        <td>{"Date"}</td>
                        <td>{"Goal"}</td>
                    </tr>
                    <br/>
                    <tr>
                        <td>{{ &self.data.Title_1 }}</td>
                        <td>{{ &self.data.Date_1 }}</td>
                        <td>{{ &self.data.Goal_1 }}</td>
                    </tr>
                    <br/>
                    <tr>
                        <td>{{ &self.data.Title_2 }}</td>
                        <td>{{ &self.data.Date_2 }}</td>
                        <td>{{ &self.data.Goal_2 }}</td>
                    </tr>
                    <br/>
                    <tr>
                        <td>{{ &self.data.Title_3 }}</td>
                        <td>{{ &self.data.Date_3 }}</td>
                        <td>{{ &self.data.Goal_3 }}</td>
                    </tr>
                </table>
            </div>
        }
    }
}

// Mount Application To Body of index.html
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
