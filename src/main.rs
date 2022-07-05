mod interpreter;

fn main() {
    let program_file_name = std::env::args().nth(1).expect("Please enter the path to the brainf source.");
    let program_string = std::fs::read_to_string(program_file_name).expect("Error reading source");
    let prog = interpreter::Program::from_str(&program_string);
    prog.execute();
}
