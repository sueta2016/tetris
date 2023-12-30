pub trait Output {
    fn write(&self, message: &str);
}

pub struct StdoutOutput;

impl Output for StdoutOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct MockOutput {
    pub expected_output: &'static str,
}

impl Output for MockOutput {
    fn write(&self, string: &str) {
        assert_eq!(self.expected_output, string);
    }
}
