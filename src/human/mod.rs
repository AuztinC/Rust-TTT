
pub fn parse_input<F>(input: &str, mut on_error: F) -> Option<usize>
where
    F: FnMut(),
{
    match input.trim().parse::<usize>() {
        Ok(num) if num < 9 => Some(num),
        _ => {
            on_error();
            None
        }
    }
}

pub fn input_out_of_bounds() {
    println!("Invalid input. Please enter a number between 0 and 8.");
}

#[cfg(test)]
mod human_spec;
