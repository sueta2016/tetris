pub trait Output {
    fn write(&self, message: &str);
    fn clear(&self);
}

pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
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
    fn clear(&self) {
        assert_eq!(self.expected_output, "");
    }
}

impl Default for MockOutput {
    fn default() -> Self {
        MockOutput {
            expected_output: "",
        }
    }
}
