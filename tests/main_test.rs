use tetris::{file_system::FileSystemOperations, main_impl};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tetris::{file_system::FakeFileSystem, output::FakeOutput};

    use super::*;

    #[test]
    fn should_out_usage_info_if_filepath_absent() {
        let args: Vec<String> = vec![];

        let mut fake_output = FakeOutput::default();

        let mut fake_file_system = FakeFileSystem::default();

        main_impl(args, &mut fake_file_system, &mut fake_output);

        assert_eq!(
            fake_output.messages[0],
            String::from("Usage: ./main <filename>")
        );
    }

    #[test]
    fn should_output_error_if_file_not_exists() {
        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut fake_file_system = FakeFileSystem::default();

        let mut fake_output = FakeOutput::default();

        main_impl(args, &mut fake_file_system, &mut fake_output);

        assert_eq!(fake_output.messages[0], String::from("File not exists"));
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

        let mut file_contents = HashMap::new();

        file_contents.insert("messi.txt".to_string(), input);

        let mut fake_file_system = FakeFileSystem { file_contents };

        let mut fake_output = FakeOutput::default();

        main_impl(args, &mut fake_file_system, &mut fake_output);

        assert_eq!(fake_output.messages[0], "File created");

        assert_eq!(fake_file_system.read_file("out.txt").unwrap(), output_str);
    }
}
