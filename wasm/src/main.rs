#![allow(dead_code)]
use yew::prelude::*;

mod text_input;
use crate::text_input::TextInput;

pub enum Msg {
    AddRow,
    UpdateFieldValue(String),
    GenerateToRow,
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
            Msg::UpdateFieldValue(input) => {
                self.held_field_value = input;
                false
            },
            Msg::GenerateToRow => {
                match self.held_field_value.parse::<usize>() {
                    Ok(i) => {
                        self.generate_to_row(i + 1);
                        self.set_highlighted_row(i);
                        true
                    }
                    Err(_) => false,
                }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let add_row_onclick = link.callback(|_| Msg::AddRow);
        let generate_onclick = link.callback(|_| Msg::GenerateToRow);
        let on_n_change = link.callback(Msg::UpdateFieldValue);

        // let hl_up_btn_onclick = ctx.link().callback(|_| Msg::HighlightUp);

        // let highlight_up_btn: Html = html! {};

        html!{
            <div>
                <button onclick={ add_row_onclick } >{ "Add row" }</button><br/>
                <label>{ "n: " }</label>
                <TextInput on_change={on_n_change} value={self.held_field_value.clone()}/>
                <button onclick={ generate_onclick } >
                    { "See nth row" }
                </button>
                { self.to_html() }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
