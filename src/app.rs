use crate::text_input::{C1Input, C2Input};
use crate::vincenty;
use yew::prelude::*;

pub enum Msg {
    SetC1(String),
    SetC2(String),
}

#[derive(Debug, Default)]
pub struct App {
    c1: String,
    c2: String,
}

impl App {
    fn get_distance(&self) -> String {
        match vincenty::calc_distance(self.c1.clone(), self.c2.clone()) {
            Ok(Some(dist)) => format!("Distance = {:?} Km", dist),
            Ok(None) => "None!".to_string(),
            Err(_) => "Boom!".to_string(),
        }
    }

    fn row_text(&self) -> String {
        if self.c1.is_empty() {
            "Please provide C1".to_string()
        } else if self.c2.is_empty() {
            "Please provide C2".to_string()
        } else {
            self.get_distance()
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetC1(c1) => self.c1 = c1,
            Msg::SetC2(c2) => self.c2 = c2,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change1 = ctx.link().callback(Msg::SetC1);
        let on_change2 = ctx.link().callback(Msg::SetC2);
        html! {
            <main>
                <div class="entry">
                    <div>
                        {"Enter C1 below:"}
                        <div class="footnote">
                            {"Example: 42.3541165,-71.0693514"}
                        </div>
                    </div>
                    <div>
                        <C1Input {on_change1} value={self.c1.clone()} />
                    </div>
                    <br/>
                    <div>
                        {"Enter C2 below:"}
                        <div class="footnote">
                            {"Example: 40.7791472, -73.9680804"}
                        </div>
                    </div>
                    <div>
                        <C2Input {on_change2} value={self.c2.clone()} />
                    </div>
                </div>
                <div class="readout">
                    <div>
                        {self.row_text()}
                    </div>
                </div>
            </main>
        }
    }
}
