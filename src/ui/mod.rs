

fn render_board(cells: &[Option<char>; 9]) -> String {
    let rows = [(0, 1, 2), (3, 4, 5), (6, 7, 8)];

    let mut out = String::new();

    for (row, (a, b, c)) in rows.iter().enumerate() {
        out.push_str(&format!(
            //refactored from single indexed access to using cell_text function
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

pub fn display_board(cells: &[Option<char>; 9], mut write: impl std::io::Write) {
    write!(write, "{}", render_board(cells)).unwrap();
}

pub fn prompt_player(player: char, mut write: impl std::io::Write) {
    let _ = writeln!(write, "Player {}, enter your move (0-8): ", player);
}

pub fn input_out_of_bounds(mut write: impl std::io::Write) {
    let _ = writeln!(write, "Invalid input. Please enter a number between 0 and 8.");
}

pub fn position_taken(mut write: impl std::io::Write) {
    let _ = writeln!(write, "Position already taken. Please choose another.");
}



#[cfg(test)]
mod ui_spec;
