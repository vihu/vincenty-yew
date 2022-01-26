use crate::text_input::{SrcInput, DstInput};
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use serde::{Serialize, Deserialize};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

pub enum Msg {
    SetSrc(String),
    SetDst(String),
    GetDistance,
    SetDistanceFetchState(FetchState<f64>),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub data: InnerData
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InnerData {
    pub distance: f64,
    pub dst: Coordinate,
    pub src: Coordinate,
}


/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub struct App {
    src: String,
    dst: String,
    distance: FetchState<f64>,
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

async fn fetch_distance(url: String) -> Result<f64, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    let data: Data = json.into_serde().unwrap();
    Ok(data.data.distance)
}

impl App {
    fn get_distance(&self) -> Option<String> {
        match &self.distance {
            FetchState::NotFetching => None,
            FetchState::Fetching => None,
            FetchState::Success(dist) => Some(format!("Distance = {} Km.", dist)),
            FetchState::Failed(_) => None
        }
    }

    fn row_text(&self) -> String {
        if self.src.is_empty() {
            "Please provide SRC".to_string()
        } else if self.dst.is_empty() {
            "Please provide Dst".to_string()
        } else {
            match self.get_distance() {
                None => "Click Submit...".to_string(),
                Some(val) => val
            }
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            src: "".to_string(),
            dst: "".to_string(),
            distance: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetSrc(src) => {
                self.src = src;
                true
            }
            Msg::SetDst(dst) => {
                self.dst = dst;
                true
            }
            Msg::GetDistance => {
                let url = format!("http://localhost:5000/distance?src={}&dst={}", self.src, self.dst);
                ctx.link().send_future(async {
                    match fetch_distance(url).await {
                        Ok(dist) => Msg::SetDistanceFetchState(FetchState::Success(dist)),
                        Err(err) => Msg::SetDistanceFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetDistanceFetchState(FetchState::Fetching));
                false
            }
            Msg::SetDistanceFetchState(fetch_state) => {
                self.distance = fetch_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change1 = ctx.link().callback(Msg::SetSrc);
        let on_change2 = ctx.link().callback(Msg::SetDst);
        html! {
            <main>
                <div class="entry">
                    <div>
                        {"Enter SRC (lat, lng) or H3 index:"}
                        <div class="footnote">
                            {"Example: 42.3541165,-71.0693514 OR 631246145620214271 OR 8c2a306638701ff"}
                        </div>
                    </div>
                    <div>
                        <SrcInput {on_change1} value={self.src.clone()} />
                    </div>
                    <br/>
                    <div>
                        {"Enter DST (lat, lng) or H3 index:"}
                        <div class="footnote">
                            {"Example: 40.7791472, -73.9680804 OR 631243921460311551 OR 8c2a100894435ff"}
                        </div>
                    </div>
                    <div>
                        <DstInput {on_change2} value={self.dst.clone()} />
                    </div>
                    <br/>
                </div>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::GetDistance)}>
                        { "Submit" }
                    </button>
                </div>
                <div class="readout">
                    <div>
                        {self.row_text()}
                    </div>
                    <div class="footnote">
                        {"NOTE* Calculating using vincenty algorithm"}
                    </div>
                </div>
            </main>
        }
    }
}
