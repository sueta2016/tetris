use field::parse_into_field;
use file_system::FileSystemOperations;

pub mod field;
pub mod file_system;

pub fn main_handler(args: Vec<String>, file_system: &mut dyn FileSystemOperations) {
    if args.len() < 1 {
        panic!("Usage: ./main <filename>")
    }
    let file_path = &args[0];
    // read file

    if !file_system.exists(&file_path) {
        panic!("File not exists")
    }

    let input = match file_system.read_file(file_path) {
        Ok(value) => value,
        Err(_) => panic!("Couldn't read file"),
    };

    let mut field = parse_into_field(input.as_str());
    // play game

    while field.can_move() {
        field.move_figure();
    }
    // write in file
    let final_state = field.to_string();

    match file_system.write_file("output.txt", final_state.as_str()) {
        Ok(_) => println!("File saved"),
        Err(_) => panic!("Couldn't save file"),
    }
}