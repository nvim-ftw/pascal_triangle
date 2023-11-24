#![allow(dead_code)]
use yew::prelude::*;
use std::fmt;

enum Msg {
    AddRow,
}

struct Cell {
    number: u128,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

struct Row {
    width: u32,
    cells: Vec<Cell>,
    highlight: bool,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        self.cells.iter().for_each(|cell| {
            result.push_str(&format!("{}", cell.number));
        });
        write!(f, "{}", result)
    }
}

struct Model {
    rows: Vec<Row>,
}

impl Model {
    fn new() -> Self {
        Self {
            rows: vec!(Row {width: 1, cells: vec!(Cell {number: 1}), highlight: false},),
        }
    }
    fn generate_rows(&mut self, count: u32) {
        for _ in 0..count {
            let current_row_number = self.rows.len() - 1;
            let current_row = &self.rows[current_row_number];

            let new_width = self.rows[current_row_number].width + 1;
            let mut new_cells: Vec<Cell> = Vec::new();

            for i in 0..new_width {
                if i == 0 || i == new_width - 1 {
                    new_cells.push(Cell {number: 1});
                    continue;
                }
                let value = current_row
                                .cells[i as usize - 1].number +
                            current_row
                                .cells[i as usize].number;
                new_cells.push(Cell { number: value });
            }
            let new_row: Row;
            new_row = Row {
                width: new_width,
                cells: new_cells,
                highlight: false,
            };
            self.rows.push(new_row);
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        let mut new_self = Self::new();
        new_self.rows[0].highlight = true;
        new_self
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddRow => {
                self.generate_rows(1);
            }
        }

        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_click = ctx.link().callback(|_| Msg::AddRow);
        html!{
            <div>
                { for self.rows.iter().map(|item| {
                    if item.highlight {
                        return html!{
                            <table class={"highlight"}><tr>{ for item.cells.iter()
                                .map(|item2| html!{
                                    <td>{format!("{}", item2)}</td>
                                })
                            }</tr></table>
                        };
                    }
                    else {
                        return html!{
                            <table><tr>{ for item.cells.iter()
                                .map(|item2| html!{
                                    <td>{format!("{}", item2)}</td>
                                })
                            }</tr></table>
                        };
                    }
                })}
                <button onclick={ on_click } >{ "Add row" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
