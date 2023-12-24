use super::Expr;
use prettytable::{format, Cell, Row, Table};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Sheet {
    pub cells: Vec<Expr>,
}

impl Sheet {
    pub fn new() -> Self {
        Self {
            cells: vec![Expr::Numeric(0)],
        }
    }
    
    pub fn new_from_vec(cells: Vec<Expr>) -> Self {
        cells.into()
    }
    
    pub fn len(&self) -> usize {
        self.cells.len()
    }
    
    pub fn get(&self, var: char) -> Option<&Expr> {
        let idx = char_to_index(var);
        self.cells.get(idx)
    }

    pub fn set(&mut self, var: char, expr: Expr) {
        let idx = char_to_index(var);
        if idx >= self.cells.len() {
            self.cells.resize(idx + 1, Expr::Numeric(0));
        }

        self.cells[idx] = expr;
    }
}

impl Into<Sheet> for Vec<Expr> {
    fn into(self) -> Sheet {
        Sheet {
            cells: self,
        }
    }
}

fn char_to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn index_to_char(idx: usize) -> char {
    (idx as u8 + 'a' as u8) as char
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        // Add header row
        let mut header = Row::empty();
        for i in 0..self.cells.len() {
            header.add_cell(Cell::new(&format!("{}", index_to_char(i))));
        }
        table.add_row(header);

        // Add value row
        let mut value_row = Row::empty();
        for expr in &self.cells {
            value_row.add_cell(Cell::new(&expr.to_string()));
        }
        table.add_row(value_row);

        write!(f, "{}", table)
    }
}
