fn main() {
    println!("privet oleh");
}

#[cfg(test)]
mod tests {
    use crate::{parse_into_field, Pixel};

    #[test]
    fn should_parse_input_into_field() {
        // Arrange
        let input = r"3 4
        .p.
        .p.
        ...
        ###";
        let expected_width = 3;
        let expected_height = 4;
        let expected_landscape: Vec<Pixel> = vec![
            Pixel { x: 0, y: 3 },
            Pixel { x: 1, y: 3 },
            Pixel { x: 2, y: 3 },
        ];
        let expected_figure: Vec<Pixel> = vec![Pixel { x: 1, y: 0 }, Pixel { x: 1, y: 1 }];

        // Act
        let field = parse_into_field(input);

        // Assert
        assert_eq!(field.width, expected_width);
        assert_eq!(field.height, expected_height);
        assert_eq!(field.landscape, expected_landscape);
        assert_eq!(field.figure, expected_figure);
    }

    #[test]
    #[should_panic(expected = "Unexpected char in field")]
    fn should_panic_on_invalid_char_in_field() {
        // Arrange
        let input = r"3 4
        .M.
        .E.
        .S.
        #I#";

        // Act
        let _ = parse_into_field(input);
    }

    #[test]
    #[should_panic(expected = "Specified width don't match with the actual one")]
    fn should_panic_on_unmatching_width() {
        // Arrange
        let input = r"2 4
        .p.
        ...
        ...
        ###";

        // Act
        let _ = parse_into_field(input);
    }

    #[test]
    #[should_panic(expected = "Specified height don't match with the actual one")]
    fn should_panic_on_unmatching_height() {
        // Arrange
        let input = r"3 5
        .p.
        ....
        ...
        ###";

        // Act
        let _ = parse_into_field(input);
    }

    #[test]
    #[should_panic]
    fn should_panic_on_invalid_field_dimensions() {
        // Arrange
        let input = r"mesi -
        .p.
        ...
        ...
        ###";

        // Act
        let _ = parse_into_field(input);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn should_panic_on_invalid_input() {
        // Arrange
        let input = r"Ronaldo better than Messi";

        // Act
        let _ = parse_into_field(input);
    }
}
