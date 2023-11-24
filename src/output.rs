use colored::Colorize;

pub trait Output {
    fn write(&self, message: &str);
    fn clear(&self);
}

pub struct ConsoleOutput;

pub struct ColorFulOutput;

impl Output for ConsoleOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
    fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl Output for ColorFulOutput {
    fn write(&self, message: &str) {
        let lines: Vec<&str> = message.split('\n').collect();

        for line in lines {
            for character in line.chars() {
                match character {
                    'p' => print!("{}", character.to_string().bright_green()),
                    '#' => print!("{}", character.to_string().bright_purple()),
                    _ => print!("{}", character.to_string().bright_blue()),
                }
            }
            print!("\n")
        }
    }
    fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}

pub struct MockOutput {
    pub expected_output: &'static str,
}

impl Output for MockOutput {
    fn write(&self, string: &str) {
        assert_eq!(self.expected_output, string);
    }
    fn clear(&self) {}
}

impl Default for MockOutput {
    fn default() -> Self {
        MockOutput {
            expected_output: "",
        }
    }
}
