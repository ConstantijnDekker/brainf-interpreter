use std::io::Read;

const DATA_SIZE: usize = 1 << 15; // 32_768

struct ProgramState {
    data_ptr: usize,
    data: [u8; DATA_SIZE],
    instr_ptr: usize,
    loop_stack: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState {
            data_ptr: 0,
            data: [0; DATA_SIZE],
            instr_ptr: 0,
            loop_stack: Vec::new(),
        }
    }
}

fn to_instruction(ch: char) -> Option<Instruction> {
    match ch {
        '+' => Some(Instruction::IncData),
        '-' => Some(Instruction::DecData),
        '>' => Some(Instruction::IncDataPtr),
        '<' => Some(Instruction::DecDataPtr),
        '.' => Some(Instruction::Output),
        ',' => Some(Instruction::Input),
        '[' => Some(Instruction::WhileNonZero),
        ']' => Some(Instruction::EndWhile),
        _ => None,
    }
}

fn skip_from(remaining: &[Instruction]) -> usize {
    let mut nest_level = 1;
    let mut advance = 1;

    // Index out of bounds if no matching EndWhile can be found.
    while nest_level > 0 {
        if remaining[advance] == Instruction::EndWhile {
            nest_level -= 1;
        } else if remaining[advance] == Instruction::WhileNonZero {
            nest_level += 1;
        }
        advance += 1;
    }

    advance
}

impl Program {
    pub fn from_str(s: &str) -> Program {
        Program {
            instructions: s.chars().filter_map(to_instruction).collect()
        }
    }

    pub fn execute(&self) {
        let mut state = ProgramState::new();

        while state.instr_ptr < self.instructions.len() {
            let instruction = self.instructions[state.instr_ptr];
            match instruction {
                Instruction::IncData => {
                    state.data[state.data_ptr] = state.data[state.data_ptr].wrapping_add(1);
                    state.instr_ptr += 1;
                }
                Instruction::DecData => {
                    state.data[state.data_ptr] = state.data[state.data_ptr].wrapping_sub(1);
                    state.instr_ptr += 1;
                }
                Instruction::IncDataPtr => {
                    state.data_ptr += 1;
                    state.instr_ptr += 1;
                }
                Instruction::DecDataPtr => {
                    state.data_ptr -= 1;
                    state.instr_ptr += 1;
                }
                Instruction::Output => {
                    print!("{}", state.data[state.data_ptr] as char);
                    state.instr_ptr += 1;
                }
                Instruction::Input => {
                    state.data[state.data_ptr] = std::io::stdin()
                        .bytes()
                        .next()
                        .expect("No more bytes in input.")
                        .expect("Error reading byte from input.");
                },
                Instruction::WhileNonZero => {
                    if state.data[state.data_ptr] == 0 {
                        state.instr_ptr += skip_from(&self.instructions[state.instr_ptr..]);
                    } else {
                        state.loop_stack.push(state.instr_ptr);
                        state.instr_ptr += 1;
                    }
                }
                Instruction::EndWhile => {
                    // Index out of bounds if no matching "WhileNonZero" exists.
                    let loop_entry = state.loop_stack.pop().unwrap();
                    state.instr_ptr = loop_entry;
                }
            };
        }
        // perhaps assert the loop stack is empty
    }
}
