#![recursion_limit = "1024"]

use anyhow::Error;
use log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_logger;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::*;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Data {
    date: Vec<String>,
    title: Vec<String>,
    goal: Vec<String>,
}

struct Model {
    link: ComponentLink<Self>,
    title: String,
    data: Data,
    task: Option<FetchTask>,
}

enum Msg {
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
            title: "Practice Journal".to_string(),
            // initialize with empty data
            data: Data {
                date: vec!["".to_string()],
                title: vec!["".to_string()],
                goal: vec!["".to_string()],
            },
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reset => self.data = Data {
                date: vec!["".to_string()],
                title: vec!["".to_string()],
                goal: vec!["".to_string()],
            },
            Msg::Request => {
                {
                    // Construct Request
                    let get_request = Request::builder()
                        .method("GET")
                        .uri("http://127.0.0.1:3000/recent")
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Access-Control-Allow-Headers", "*")
                        .body(Nothing)
                        .unwrap();

                    // Return Failure or Success via Msg
                    let callback =
                        self.link
                            .callback(|response: Response<Json<Result<Data, Error>>>| {
                                if let (meta, Json(Ok(body))) = response.into_parts() {
                                    if meta.status.is_success() {
                                        return Msg::FetchResourceComplete(body);
                                    }
                                }
                                Msg::FetchResourceFailed
                            });

                    // Execute Task & Store
                    let task = FetchService::fetch(get_request, callback);
                    self.task = Some(task.unwrap());
                };
            }
            Msg::FetchResourceComplete(body) => {
                self.data = body;
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
        // Generate Journal Entries UI
        let mut data_ui = VList::new();
        if self.data.date[0] != "".to_string() {
            let header = html! {
                <>
                <thead>
                <tr>
                    <th>{"Title"}</th>
                    <th>{"Date"}</th>
                    <th>{"Goal"}</th>
                </tr>
                </thead>
                </>
            };
            data_ui.add_child(header);
    
            let mut table_data = VList::new();
            let entry_count = &self.data.date.len();
            for i in 0..*entry_count as i32 {
                table_data.add_child(
                    html! {
                        <>
                        <tr>
                            <td>{{ &self.data.date[i as usize] }}</td>
                            <td>{{ &self.data.title[i as usize] }}</td>
                            <td>{{ &self.data.goal[i as usize] }}</td>
                        </tr>
                        </>
                    }
                );
            }
            data_ui.add_child(
                html!{
                    <>
                    <tbody>
                        {{ table_data }}
                    </tbody>
                    </>
                }
            )
        }
        
        html! {
            <div>
                <h1>{ &self.title }</h1>
                <p class="entry">{ "Welcome. Please choose an option from below to get started. You're doing great." }</p>
                <br/>
                <table class="buttons">
                    <tr>
                        <td><button onclick=self.link.callback(|_| Msg::Request)>{ "View Recent Entries" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Reset)>{ "Clear Recent Entries" }</button></td>
                    </tr>
                </table>
                <br/>
                <table class="styled-table">
                    {{ data_ui }}
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
