use tetris::main_handler;

#[test]
#[should_panic(expected = "Usage: ./main <filename>")]
fn should_panic_if_filepath_absent() {
    let args: Vec<String> = vec![];

    main_handler(args);
}
