use std::env;

use tetris::{file_system, main_impl, output};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    main_impl(
        args,
        &mut file_system::FileSystem,
        &mut output::ConsoleOutput,
    );
}
