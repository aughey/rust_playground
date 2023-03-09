use ndarray::{self, arr2, Array1, Array2};
use std::io;

fn get_col_row<T: io::BufRead>(mut io: T) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut input = String::new();

    println!("Column: ");
    io.read_line(&mut input)?;
    let x_num = input.trim().parse::<usize>()?;
    // Clear input
    input.clear();
    // Get y
    println!("Row: ");
    io.read_line(&mut input)?;
    let y_num = input.trim().parse::<usize>()?;
    Ok((x_num, y_num))
}

struct Board {
    grid: Array2<char>,
}
impl Board {
    fn new() -> Board {
        let cell_empty: char = '-';
        Board {
            grid: arr2(&[
                [cell_empty, cell_empty, cell_empty],
                [cell_empty, cell_empty, cell_empty],
                [cell_empty, cell_empty, cell_empty],
            ]),
        }
    }
    fn print(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", self.grid[(i, j)]);
            }
            print!("\n");
        }
    }

    fn try_set(&mut self, cell: (usize, usize), player: &PlayerTurn) -> Result<(), String> {
        let value = self.grid.get_mut(cell).ok_or("Invalid index")?;
        if *value != '-' {
            return Err("Cell is already occupied".to_string());
        } else {
            *value = player.to_char();
            Ok(())
        }
    }

    fn win(&self) -> bool {
        let grid = &self.grid;
        // Check columns
        for col in grid.columns() {
            if col.iter().all(|n| n == &col[0]) && col[0] != '-' {
                return true;
            }
        }

        // Check rows
        for row in grid.rows() {
            if row.iter().all(|n| n == &row[0]) && row[0] != '-' {
                return true;
            }
        }

        // Check diagonal
        if grid.diag().iter().all(|n| n == &grid.diag()[0]) && grid.diag()[0] != '-' {
            return true;
        }

        // Check other diagonal
        if diag_2(grid).iter().all(|n| n == &diag_2(grid)[0]) && diag_2(grid)[0] != '-' {
            return true;
        }

        // Return false if no one has won
        false
    }
}

fn playgame() -> Result<(), Box<dyn std::error::Error>> {
    let cell_empty: char = '-';
    let stdin = io::stdin();

    let mut board = Board::new();
    let mut player = PlayerTurn::X;

    loop {
        player.print_turn();

        board.print();

        loop {
            let cell = get_col_row(stdin.lock())?;
            if let Err(why) = board.try_set(cell, &player) {
                println!("{}", why);
            } else {
                break;
            }
        }

        if board.win() {
            println!("Someone has won!");
            break;
        }

        player = player.other();
    }

    Ok(())
}

fn main() {
    let res = playgame();
    if let Err(why) = res {
        println!("Error: {}", why);
    }
}

fn oldmain() {
    let cell_empty: char = '-';
    let stdin = io::stdin();
    let mut grid = arr2(&[
        [cell_empty, cell_empty, cell_empty],
        [cell_empty, cell_empty, cell_empty],
        [cell_empty, cell_empty, cell_empty],
    ]);

    let mut playing = true;
    let mut player = PlayerTurn::X;

    // Game loop
    while playing {
        // Prompt the player
        player.print_turn();

        // Initialize position and input variables
        let mut input = String::new();
        let x_num: usize;
        let y_num: usize;

        // Get position
        // Get x
        println!("Column: ");
        stdin.read_line(&mut input).expect("Read failed: at x_str");
        x_num = input
            .trim()
            .parse::<usize>()
            .expect("Parse failed: at x_num")
            - 1;
        // Clear input
        input.clear();
        // Get y
        println!("Row: ");
        stdin.read_line(&mut input).expect("Read failed: at y_str");
        y_num = input
            .trim()
            .parse::<usize>()
            .expect("Parse failed: at y_num")
            - 1;

        // Check for out of bounds error
        // `function`

        // If the cell is empty
        if grid[(y_num, x_num)] == cell_empty {
            // Place an X or O
            match player {
                PlayerTurn::X => grid[(y_num, x_num)] = 'X',
                PlayerTurn::O => grid[(y_num, x_num)] = 'O',
            }
        } else {
            // If the cell is invalid, the grid will not be printed and the turn will not change
            println!("Invalid location, please try again.");
            continue;
        }

        // Print the grid as a grid
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", grid[(i, j)]);
            }
            print!("\n");
        }

        // Check for a winner
        match check_win(&grid) {
            true => {
                playing = false;
                println!("Someone has won!");
                continue;
            }
            false => (),
        };

        // Switch turn
        player = player.switch();
    }
}

//
fn check_win(grid: &Array2<char>) -> bool {
    // Check columns
    for col in grid.columns() {
        if col.iter().all(|n| n == &col[0]) && col[0] != '-' {
            return true;
        }
    }

    // Check rows
    for row in grid.rows() {
        if row.iter().all(|n| n == &row[0]) && row[0] != '-' {
            return true;
        }
    }

    // Check diagonal
    if grid.diag().iter().all(|n| n == &grid.diag()[0]) && grid.diag()[0] != '-' {
        return true;
    }

    // Check other diagonal
    if diag_2(grid).iter().all(|n| n == &diag_2(grid)[0]) && diag_2(grid)[0] != '-' {
        return true;
    }

    // Return false if no one has won
    false
}

// Returns an Array1 containing the characters on the 'other diagonal' of the given Array2
fn diag_2(grid: &Array2<char>) -> Array1<char> {
    // Copy the given array
    let mut invert = grid.clone();
    // Invert the array side-to-side
    invert.invert_axis(ndarray::Axis(1));
    // Return the diagonal of the inverted array
    invert.diag().to_owned()
}

fn check_out_of_bounds(grid: &Array2<char>, y: usize, x: usize) -> Result<(), String> {
    match grid.get((y, x)) {
        Some(_) => Ok(()),
        None => return Err(String::from("Invalid index")),
    }
}

enum PlayerTurn {
    X,
    O,
}

impl PlayerTurn {
    fn switch(self) -> PlayerTurn {
        match self {
            PlayerTurn::X => PlayerTurn::O,
            PlayerTurn::O => PlayerTurn::X,
        }
    }

    fn print_turn(&self) {
        match self {
            PlayerTurn::X => println!("X's turn"),
            PlayerTurn::O => println!("O's turn"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            PlayerTurn::X => 'X',
            PlayerTurn::O => 'O',
        }
    }

    fn other(&self) -> PlayerTurn {
        match self {
            PlayerTurn::X => PlayerTurn::O,
            PlayerTurn::O => PlayerTurn::X,
        }
    }
}
