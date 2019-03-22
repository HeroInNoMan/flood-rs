use colored::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
enum Color {
    RED,
    YELLOW,
    BLUE,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0, 3) {
            0 => Color::RED,
            1 => Color::YELLOW,
            _ => Color::BLUE,
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cell = match self {
            Color::RED => " ".on_red(),
            Color::YELLOW => " ".on_yellow(),
            Color::BLUE => " ".on_blue(),
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
        flood_board
    }

    fn change_color(&mut self, color: Color) {
        // ① change color of flooded colors
        for cell in &mut self.board {
            if cell.is_flooded {
                cell.color = color.clone();
            }
        }

        // ② try to flood neighbors
        self.flood_neighbors();

        println!("Changing to {}!", color)
    }

    fn flood_neighbors(&mut self) {
        // // create queue with flooded elements
        // let mut candidates = self
        //     .board
        //     .iter()
        //     .flat_map(|row| row.iter())
        //     .filter(|cell| cell.is_flooded);

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
}

fn read_input() -> Option<Color> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.as_ref() {
        "b\n" => Some(Color::BLUE),
        "y\n" => Some(Color::YELLOW),
        "r\n" => Some(Color::RED),
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
    let board = FloodBoard::new(3, 4);
    board.display();
    play(board);
}
