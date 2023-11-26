#![allow(dead_code)]
use yew::prelude::*;

mod num_input;
use crate::num_input::NumInput;

pub enum TrackedValueType {
    I64(i64),
    Str(String),
}

pub enum Msg {
    AddRow,
    UpdateHighlightedRow(String),
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
            Msg::UpdateHighlightedRow(input) => {
                match input.parse::<usize>() {
                    Ok(i) if i > 0 => {
                        self.set_highlighted_row(i-1);
                        true
                    },
                    Ok(i) if i == 0 => {
                        match self.highlighted_row() {
                            Some(_) => self.remove_highlight(),
                            None => return false,
                        }
                        true
                    }
                    _ => true,
                }
            },
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let add_row_onclick = link.callback(|_| Msg::AddRow);
        let on_change = link.callback(Msg::UpdateHighlightedRow);

        // let hl_up_btn_onclick = ctx.link().callback(|_| Msg::HighlightUp);

        // let highlight_up_btn: Html = html! {};

        html!{
            <div>
                <button onclick={ add_row_onclick } >{ "Add row" }</button>
                <NumInput {on_change} value={
                    match self.highlighted_row() {
                        Some(n) => format!("{}", n + 1),
                        None => "0".to_string(),
                    }
                } />
                { self.to_html() }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
