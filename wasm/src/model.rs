use yew::html::Html;
use yew::html;
use std::fmt;

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

impl Row {
    // returns a table with one row and all the cells
    fn to_html(&self) -> Html {
        if self.highlight {
            html! {<>
                <table class={"highlight"}><tr>
                    { for self.cells.iter().map(|cell| html! {
                        <td>{ format!("{}", cell) }</td>
                    })}
                </tr></table>
            </>}
        }
        else {
            html! {<>
                <table><tr>
                    { for self.cells.iter().map(|cell| html! {
                        <td>{ format!("{}", cell) }</td>
                    })}
                </tr></table>
            </>}
        }
    }
}

// formats Row as the numbers in each of its cells, delimited by spaces 
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        self.cells.iter().for_each(|cell| {
            result.push_str(&format!("{}", cell.number));
        });
        write!(f, "{}", result)
    }
}

pub struct Model {
    rows: Vec<Row>,
    highlighted_row: Option<usize>,
}

impl Model {
    pub fn new() -> Self {
        let rows = vec!(Row {width: 1, cells: vec!(Cell {number: 1}), highlight: true},);
        Self { 
            rows: rows,
            highlighted_row: Some(0),
        }
    }
    pub fn generate_rows(&mut self, count: u32) {
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
    pub fn to_html(&self) -> Html {
    html! { 
        { for self.rows.iter().map(|row| row.to_html() )}
    }
    }
}

