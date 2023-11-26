#![allow(dead_code)]
use yew::prelude::*;

pub enum Msg {
    AddRow,
    HighlightDown,
    HighlightUp,
}

mod model;
use model::Model;

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        let new_self = Self::new();
        new_self
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddRow => {
                self.generate_rows(1);
                true
            },
            Msg::HighlightDown => {
                true
            },
            Msg::HighlightUp => {
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let add_row_onclick = ctx.link().callback(|_| Msg::AddRow);

        // let hl_up_btn_onclick = ctx.link().callback(|_| Msg::HighlightUp);

        // let highlight_up_btn: Html = html! {};

        html!{
            <div>
                { self.to_html() }
                <button onclick={ add_row_onclick } >{ "Add row" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
