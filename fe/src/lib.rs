#![recursion_limit = "256"]

use yew::format::{Json, Nothing};
use serde::Deserialize;
use yew::services::fetch::{FetchService, Request, Response};
use wasm_logger;
use anyhow::Error;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use log;

struct Model {
    link: ComponentLink<Self>,
    title: String,
    value: i64,
    data: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Data {
   value: String
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::AddSix => self.value += 6,
            Msg::Reset => self.value = 0,
            Msg::Request => {
                {
                    let get_request = Request::get("http://0.0.0.0:3000/recent").body(Nothing).unwrap();
                    let callback = self.link.callback(|response: Response<Json<Result<Data, Error>>>| {
                        log::info!("{:#?}", &response);
                        if let (meta, Json(Ok(body))) = response.into_parts() {
                            log::info!("{:#?}", meta);
                            if meta.status.is_success() {
                                return Msg::FetchResourceComplete;
                            }
                        }
                        Msg::FetchResourceFailed
                    });
                    
                    let task = FetchService::fetch(get_request, callback);
                    log::info!("{:#?}", task);
                    
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
