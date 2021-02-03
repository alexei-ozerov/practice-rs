#![recursion_limit = "256"]

use anyhow::Error;
use log;
use wasm_bindgen::prelude::*;
use wasm_logger;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use serde::{Serialize, Deserialize};

struct Model {
    link: ComponentLink<Self>,
    title: String,
    value: i64,
    data: String,
    task: Option<FetchTask>,
}

// TODO: Figure out a way to make deserialization dynamic as the response will be differenly sized based on the amount of data in DB
#[derive(Serialize, Deserialize, Debug)]
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

enum Msg {
    AddOne,
    AddSix,
    Reset,
    Request,
    FetchResourceComplete,
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
            data: "".to_string(),
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

                    // TODO: Get the processed JSON file into self.data
                    let callback =
                        self.link
                            .callback(|response: Response<Json<Result<Data, Error>>>| {
                                log::info!("{:#?}", &response);
                                if let (meta, Json(Ok(body))) = response.into_parts() {
                                    if meta.status.is_success() {
                                        self.data = serde_json::to_string(&body).unwrap().clone();
                                        return Msg::FetchResourceComplete;
                                    }
                                }
                                Msg::FetchResourceFailed
                            });

                    let task = FetchService::fetch(get_request, callback);
                    self.task = Some(task.unwrap());
                };
            }
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
                <table class="attr">
                    <tr>
                        <td><button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add One" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::AddSix)>{ "Add Six" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Reset)>{ "Reset" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Request)>{ "View Recent Entries" }</button></td>
                    </tr>
                </table>
                <p class="count">{ self.value }</p>
                <br/>
                <p>{ &self.data }</p>
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
