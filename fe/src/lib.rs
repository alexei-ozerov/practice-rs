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

#[derive(Debug)]
struct FormData {
    title: String,
    goal: String,
    notes: String,
}

struct Model {
    link: ComponentLink<Self>,
    form: String,
    title: String,
    data: Data,
    task: Option<FetchTask>,
    form_input: FormData,
}

enum Msg {
    Form,
    Submit,
    Reset,
    GetRequest,
    PostRequest,
    FetchResourceComplete(Data),
    FetchResourceFailed,
    TitleUpdate(String),
    GoalUpdate(String),
    NotesUpdate(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            form: "No".to_string(),
            title: "Practice Journal".to_string(),
            // initialize with empty data
            data: Data {
                date: vec!["".to_string()],
                title: vec!["".to_string()],
                goal: vec!["".to_string()],
            },
            task: None,
            form_input: FormData {
                title: "".to_string(),
                goal: "".to_string(),
                notes: "".to_string(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Form => self.form = "Yes".to_string(),
            Msg::Reset => {
                self.data = Data {
                    date: vec!["".to_string()],
                    title: vec!["".to_string()],
                    goal: vec!["".to_string()],
                };
                self.form = "No".to_string();
            }
            Msg::GetRequest => {
                {
                    // Construct Request
                    let get_request = Request::builder()
                        .method("GET")
                        .uri("http://127.0.0.1:3001/recent")
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
            Msg::PostRequest => {
                log::info!("{:#?}", self.form_input);

                // Construct Payload & Request
                let payload: String = r#"{"title": "#.to_string()
                    + &self.form_input.title
                    + r#""goal:" "#
                    + &self.form_input.goal
                    + r#""notes:" "#
                    + &self.form_input.notes
                    + "}";

                let post_request = Request::builder()
                    .method("POST")
                    .uri("http://127.0.0.1:3001/recent")
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Headers", "*")
                    .body(Nothing)
                    .unwrap();

                // Send Request
                // TODO: Create a SUCCESS element and message to call if a successful response is
                // returned from the API
                log::info!("{:#?}", payload);
                log::info!("{:#?}", post_request);
            }
            Msg::Submit => {
                log::info!("{:#?}", &self.form_input);
            }
            Msg::TitleUpdate(val) => {
                self.form_input.title = val;
                log::info!("{:#?}", self.form_input.title);
            }
            Msg::GoalUpdate(val) => {
                self.form_input.goal = val;
                log::info!("{:#?}", self.form_input.title);
            }
            Msg::NotesUpdate(val) => {
                self.form_input.notes = val;
                log::info!("{:#?}", self.form_input.notes);
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
        let data_ui = build_table(&self.data);

        let mut new_entry_form = VList::new();
        if self.form == "Yes".to_string() {
            new_entry_form = build_form(&self);
        }

        // Render Application
        html! {
            <div>
                <h1>{ &self.title }</h1>
                <p class="entry">{ "Welcome. Please choose an option from below to get started. You're doing great." }</p>
                <br/>
                <table class="buttons">
                    <tr>
                        <td><button onclick=self.link.callback(|_| Msg::GetRequest)>{ "View Recent Entries" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Form)>{ "Add New Journal Entry" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Reset)>{ "Clear Journal Entries" }</button></td>
                    </tr>
                </table>
                <br/>
                {{ new_entry_form }}
                <table class="styled-table">
                    {{ data_ui }}
                </table>
            </div>

        }
    }
}

fn build_form(ctx: &Model) -> VList {
    let mut form_ui = VList::new();
    form_ui.add_child({
        html! {
            <>
            <div class="container">
            <form onsubmit=ctx.link.callback(|_| Msg::PostRequest)>
              <div class="row">
                <div class="col-25">
                  <label for="fname">{"Title"}</label>
                </div>
                <div class="col-75">
                  <input
                    type="text"
                    id="fname"
                    name="firstname"
                    oninput=ctx.link.callback(|e: InputData| Msg::TitleUpdate(e.value))
                    placeholder="Practice Session Title..."
                  />
                </div>
              </div>
              <div class="row">
                <div class="col-25">
                  <label for="lname">{"Goal"}</label>
                </div>
                <div class="col-75">
                  <input
                    type="text"
                    id="lname"
                    name="lastname"
                    oninput=ctx.link.callback(|e: InputData| Msg::GoalUpdate(e.value))
                    placeholder="Practice Session Goal..."
                  />
                </div>
              </div>
              <div class="row">
                <div class="col-25">
                  <label for="notes">{"Notes"}</label>
                </div>
                <div class="col-75">
                  <textarea
                    id="notes"
                    name="notes"
                    placeholder="Practice Session Notes..."
                    oninput=ctx.link.callback(|e: InputData| Msg::NotesUpdate(e.value))
                    style="height:200px"
                  />
                </div>
              </div>
              <br/>
              <div class="row">
                <button onclick=ctx.link.callback(|_| Msg::Submit)>{"Submit"}</button>
              </div>
            </form>
          </div>
          </>
        }
    });
    form_ui
}

// Generate Journal Entries UI
fn build_table(data: &Data) -> VList {
    let mut data_ui = VList::new();
    if data.date[0] != "".to_string() {
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

        // Iterate over all returned data and create <tr> for each, placing inside <tbody>
        let mut table_data = VList::new();
        let entry_count = &data.date.len();
        for i in 0..*entry_count as i32 {
            table_data.add_child(html! {
                <>
                <tr>
                    <td>{{ &data.date[i as usize] }}</td>
                    <td>{{ &data.title[i as usize] }}</td>
                    <td>{{ &data.goal[i as usize] }}</td>
                </tr>
                </>
            });
        }
        data_ui.add_child(html! {
            <>
            <tbody>
                {{ table_data }}
            </tbody>
            </>
        })
    }

    data_ui
}

// Mount Application To Body of index.html
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
