mod interpreter;
use std::{env, fs};

fn main() {
    let program_file_name = env::args().nth(1).expect("Please enter the path to the brainf source.");
    let program_string = fs::read_to_string(program_file_name).expect("Error reading source");
    let prog = interpreter::Program::from_str(&program_string);
    dbg!(&prog);
    prog.execute();
}
