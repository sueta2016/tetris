use std::env;

use tetris::main_handler;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    main_handler(args);
}
