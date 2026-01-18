use std::io::{self, Write};

pub const WELCOME_MESSAGE: &str = "\nWelcome to Tic Tac Toe! \n Player X goes first. Choose a square by entering a number from 0 to 8.\n";
pub const OUT_OF_BOUNDS: &str = "Invalid input. Please enter a number between 0 and 8.";
pub const POSITION_TAKEN: &str = "Position already taken. Please choose another.";
pub const INVALID_CHAR_INPUT: &str = "Please enter a number";

fn render_board(cells: &[Option<char>; 9]) -> String {
    let rows = [(0, 1, 2), (3, 4, 5), (6, 7, 8)];

    let mut out = String::new();

    for (row, (a, b, c)) in rows.iter().enumerate() {
        out.push_str(&format!(
            "  {} | {} | {}\n",
            cell_text(cells, *a),
            cell_text(cells, *b),
            cell_text(cells, *c),
        ));

        if row < 2 {
            out.push_str(" -----------\n");
        }
    }
    out.push_str("-------------\n");
    out
}

fn cell_text(cells: &[Option<char>; 9], idx: usize) -> String {
    match cells[idx] {
        Some(ch) => format!("{}", ch),
        None => format!("{}", idx),
    }
}

pub fn display_board(cells: &[Option<char>; 9], mut write: impl Write) -> io::Result<()> {
    write!(write, "{}", render_board(cells))
}

pub fn prompt_player(player: char, mut write: impl Write) -> io::Result<()> {
    writeln!(write, "Player {}, enter your move: ", player)
}

pub fn write_error(mut write: impl Write, error: &str) -> io::Result<()> {
    writeln!(write, "{}", error)
}

pub fn announce_winner(winner: char, mut write: impl Write) -> io::Result<()> {
    writeln!(write, "Player {} wins!", winner)
}

pub fn announce_tie(mut write: impl Write) -> io::Result<()> {
    writeln!(write, "The game is a tie!")
}

pub fn welcome(mut write: impl Write) -> io::Result<()> {
    writeln!(write, "{}", WELCOME_MESSAGE)
}

pub fn unknown_argument(arg: &str, mut write: impl Write) -> io::Result<()> {
    writeln!(
        write,
        "\nUnknown argument: '{}' Defaulting to Human vs AI mode. \n Use '-- -HvH' to play Human vs Human or '-- -AIvAI' to watch AI vs AI.\n",
        arg
    )
}

pub fn starting_game(arg: &str, mut write: impl Write) -> io::Result<()> {
    writeln!(write, "Starting game: {}\n", arg)
}

#[cfg(test)]
mod spec;
