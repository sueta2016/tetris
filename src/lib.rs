pub mod field;

pub fn main_handler(args: Vec<String>) {
    if args.len() < 1 {
        panic!("Usage: ./main <filename>")
    }
    let filename = &args[0];
    println!("{}", filename)
    // read file

    // play game

    // write in file
}
