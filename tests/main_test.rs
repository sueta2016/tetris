use tetris::{file_system::FileSystemOperations, main_handler};

struct MockFileSystem {
    is_exist: bool,
    file_content_result: Result<String, std::io::Error>,
    expected_output: &'static str,
}

impl FileSystemOperations for MockFileSystem {
    fn exists(&self, _file_path: &str) -> bool {
        self.is_exist
    }
    fn read_file(&self, _file_path: &str) -> Result<String, std::io::Error> {
        match self.file_content_result {
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            )),
            Ok(ref value) => Ok(value.clone()),
        }
    }
    fn write_file(&mut self, _file_path: &str, content: &str) -> Result<(), std::io::Error> {
        assert_eq!(content, self.expected_output);
        Ok(())
    }
}

impl Default for MockFileSystem {
    fn default() -> Self {
        MockFileSystem {
            is_exist: false,
            file_content_result: Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            )),
            expected_output: "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Usage: ./main <filename>")]
    fn should_panic_if_filepath_absent() {
        let args: Vec<String> = vec![];

        main_handler(args, &mut MockFileSystem::default());
    }

    #[test]
    #[should_panic(expected = "File not exists")]
    fn should_panic_if_file_not_exists() {
        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut mock_file_system = MockFileSystem {
            is_exist: false,
            ..Default::default()
        };

        main_handler(args, &mut mock_file_system)
    }

    #[test]
    #[should_panic(expected = "Couldn't read file")]
    fn should_panic_if_could_not_read_file() {
        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut mock_file_system = MockFileSystem {
            file_content_result: Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            )),
            is_exist: true,
            ..Default::default()
        };

        main_handler(args, &mut mock_file_system)
    }

    #[test]
    fn should_correctly_play_game() {
        let input = r"3 4
        .p.
        pp.
        ...
        ###"
        .to_string();

        let output_str = "...
.p.
pp.
###
";

        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut mock_file_system = MockFileSystem {
            file_content_result: Ok(input),
            is_exist: true,
            expected_output: output_str,
        };

        main_handler(args, &mut mock_file_system);
    }
}
