#[cfg(test)]
mod tests {

    use crate::human::*;
    use crate::ui::input_out_of_bounds;

    #[test]
    fn parses_valid_input() {
        let input = "4\n";
        assert_eq!(parse_input(input, || {}), Some(4));
    }

    #[test]
    fn invokes_out_of_bounds_handler_on_invalid_input_and_captures_output() {
        let mut buf: Vec<u8> = Vec::new();
        let result = parse_input("9", || {
            input_out_of_bounds(&mut buf);
        });
        let out = String::from_utf8(buf).unwrap();
        
        assert_eq!(out,"Invalid input. Please enter a number between 0 and 8.\n");
        assert!(result.is_none());
    }
}
