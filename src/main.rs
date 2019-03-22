use colored::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
enum Color {
    BLUE,
    GREEN,
    PURPLE,
    RED,
    WHITE,
    YELLOW,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0, 6) {
            0 => Color::BLUE,
            1 => Color::GREEN,
            2 => Color::PURPLE,
            3 => Color::RED,
            4 => Color::WHITE,
            _ => Color::YELLOW,
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cell = match self {
            Color::BLUE => " ".on_blue(),
            Color::GREEN => " ".on_green(),
            Color::PURPLE => " ".on_purple(),
            Color::RED => " ".on_red(),
            Color::WHITE => " ".on_white(),
            Color::YELLOW => " ".on_yellow(),
        };
        write!(f, "{}", cell)
    }
}

#[derive(Clone)]
struct Cell {
    is_flooded: bool,
    color: Color,
}

impl Cell {
    fn default() -> Self {
        Cell {
            is_flooded: false,
            color: Color::BLUE,
        }
    }
}

struct FloodBoard {
    board: Vec<Cell>,
    width: usize,
    height: usize,
}

impl FloodBoard {
    fn new(width: usize, height: usize) -> Self {
        let mut flood_board = FloodBoard {
            board: vec![Cell::default(); width * height],
            width,
            height,
        };
        for cell in &mut flood_board.board {
            cell.color = rand::random::<Color>();
        }

        flood_board.board[0].is_flooded = true;
        flood_board.flood_neighbors(flood_board.board[0].color);
        flood_board
    }

    fn change_color(&mut self, color: Color) {
        // ① change color of flooded colors
        for cell in &mut self.board {
            if cell.is_flooded {
                cell.color = color;
            }
        }

        // ② try to flood neighbors
        self.flood_neighbors(color);

        println!("Changing to {}!", color)
    }

    fn flood_neighbors(&mut self, color: Color) {
        // create queue with flooded elements
        let mut candidates: std::collections::VecDeque<_> = self
            .board
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_flooded)
            .map(|(cellidx, _)| cellidx)
            .collect();
        while let Some(candidate) = candidates.pop_front() {
            let (x, y) = self.idx_to_pos(candidate);
            let x = x as isize;
            let y = y as isize;

            let mut to_be_flooded = |x, y| {
                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                    let cellidx = self.pos_to_idx(x as usize, y as usize);
                    let mut cell = &mut self.board[cellidx];
                    if !cell.is_flooded && cell.color == color {
                        cell.is_flooded = true;
                        candidates.push_back(cellidx);
                    }
                }
            };

            // left
            to_be_flooded(x - 1, y);
            to_be_flooded(x, y - 1);
            to_be_flooded(x + 1, y);
            to_be_flooded(x, y + 1);
        }
    }

    fn display(&self) {
        for (cellidx, cell) in self.board.iter().enumerate() {
            if cellidx % self.width == 0 {
                println!("");
            }
            print!("{}", cell.color);
        }
        println!("");
    }

    fn pos_to_idx(&self, col: usize, row: usize) -> usize {
        col + (row * self.width)
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width % self.height)
    }
}

fn read_input() -> Option<Color> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.as_ref() {
        "b\n" => Some(Color::BLUE),
        "g\n" => Some(Color::GREEN),
        "p\n" => Some(Color::PURPLE),
        "r\n" => Some(Color::RED),
        "w\n" => Some(Color::WHITE),
        "y\n" => Some(Color::YELLOW),
        _ => None,
    }
}

fn play(mut board: FloodBoard) {
    loop {
        let color = read_input();
        if let Some(color) = color {
            board.change_color(color);
            board.display();
        }
    }
}

fn main() {
    let board = FloodBoard::new(12, 12);
    board.display();
    play(board);
}
