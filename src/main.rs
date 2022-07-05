const DATA_SIZE: usize = 1 << 15; // 32_768

struct Memory {
    data_ptr: usize,
    data: [u8; DATA_SIZE],
}

enum Instruction {
    IncData,
    DecData,
    IncDataPtr,
    DecDataPtr,
    Output,
    Input,
    WhileNonZero,
    EndWhile,
}

struct Progrm {
    instr_ptr: usize,
    instructions: Vec<Instruction>,
}

fn main() {
    println!("Hello, world!");
}
