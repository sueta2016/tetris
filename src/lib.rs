use field::Field;
use file_system::FileSystemOperations;
use output::Output;

pub mod field;
pub mod file_system;
pub mod output;

pub fn main_impl(
    args: Vec<String>,
    file_system: &mut dyn FileSystemOperations,
    output: &mut dyn Output,
) {
    if args.len() < 1 {
        output.write("Usage: ./main <filename>");
        return;
    }
    let file_path = &args[0];

    // read file
    let input = match file_system.read_file(file_path) {
        Ok(value) => value,
        Err(e) => {
            output.write(&format!("Error: {}", e));
            return;
        }
    };

    let mut field = match Field::try_from(input.as_str()) {
        Ok(value) => value,
        Err(err_text) => {
            output.write(err_text);
            return;
        }
    };

    field.play();

    // write to a file
    let final_state = field.to_string();

    match file_system.write_file("out.txt", final_state.as_str()) {
        Ok(_) => output.write("File created"),
        Err(e) => output.write(&format!("Error: {}", e)),
    }
}
