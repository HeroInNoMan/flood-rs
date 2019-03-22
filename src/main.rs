
#[derive(Clone)]
enum Color {
    RED,
    YELLOW,
    BLUE,
    BLACK,
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
            color: Color::BLACK,
        }
    }
}

struct FloodBoard {
    board: Vec<Vec<Cell>>,
}

impl FloodBoard {
    fn new(height: usize, width: usize) -> Self {
        let mut flood_board = FloodBoard {
            board: vec![vec![Cell::default(); width]; height],
        };
        flood_board.board[0][0].is_flooded = true;
        flood_board
    }
}

fn play(board: FloodBoard) {}

fn main() {
    let board = FloodBoard::new(12, 12);
    play(board);
}
