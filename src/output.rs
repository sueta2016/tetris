pub trait Output {
    fn write(&mut self, message: &str);
}

pub struct StdoutOutput;

impl Output for StdoutOutput {
    fn write(&mut self, message: &str) {
        println!("{}", message);
    }
}

pub struct FakeOutput {
    pub messages: Vec<String>,
}

impl Default for FakeOutput {
    fn default() -> Self {
        FakeOutput {
            messages: Vec::new(),
        }
    }
}

impl Output for FakeOutput {
    fn write(&mut self, message: &str) {
        self.messages.push(message.to_string())
    }
}
