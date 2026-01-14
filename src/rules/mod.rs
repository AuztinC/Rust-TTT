pub fn winner(cells: &[Option<char>; 9]) -> Option<char> {
    let winning_combinations = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    for &(a, b, c) in &winning_combinations {
        if let (Some(ch1), Some(ch2), Some(ch3)) = (cells[a], cells[b], cells[c]) {
            if ch1 == ch2 && ch2 == ch3 {
                return Some(ch1);
            }
        }
    }
    None
}

pub fn is_tie(cells: &[Option<char>; 9]) -> bool {
    cells.iter().all(|cell| cell.is_some())
}

pub fn open_cells(cells: &[Option<char>; 9]) -> Vec<usize> {
    cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| if cell.is_none() { Some(idx) } else { None })
        .collect()
}

#[cfg(test)]
mod rules_spec;