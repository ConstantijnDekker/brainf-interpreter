use std::io::Read;

const DATA_SIZE: usize = 1 << 15; // 32_768

struct ProgramState {
    data_ptr: usize,
    data: [u8; DATA_SIZE],
    instr_ptr: usize,
    loop_stack: Vec<usize>,
    on: bool,
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
            on: true,
        }
    }

    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::IncData => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_add(1);
            }
            Instruction::DecData => {
                self.data[self.data_ptr] = self.data[self.data_ptr].wrapping_sub(1);
            }
            Instruction::IncDataPtr => {
                self.data_ptr = self.data_ptr.wrapping_add(1);
            }
            Instruction::DecDataPtr => {
                self.data_ptr = self.data_ptr.wrapping_sub(1);
            }
            Instruction::Output => print!("{}", self.data[self.data_ptr] as char),
            Instruction::Input => {
                self.data[self.data_ptr] = std::io::stdin()
                    .bytes()
                    .next()
                    .expect("No more bytes in input.")
                    .expect("Error reading byte from input.");
            },
            Instruction::WhileNonZero => {
                if self.data[self.data_ptr] == 0 {
                    self.on = false;
                } else {
                    self.loop_stack.push(self.instr_ptr);
                }
            }
            Instruction::EndWhile => {
                let loop_entry = self.loop_stack.pop().unwrap();
                self.instr_ptr = loop_entry;
            }
        };
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

fn find_match(instructions: Vec<Instruction>, mut instr_ptr: usize) -> usize {
    let x = 1;
    while x > 0 {
        if instructions[instr_ptr] == Instruction::WhileNonZero {
            x += 1;
        } else if instructions[instr_ptr] == Instruction::EndWhile {
            x -= 1;
        }
        instr_ptr += 1;
    }
}

impl Program {
    pub fn from_str(s: &str) -> Program {
        Program {
            instructions: s.chars().filter_map(to_instruction).collect()
        }
    }

    pub fn execute(&self) {
        let mut state = ProgramState::new();

        loop {
            let instr = self.instructions[state.instr_ptr];
            if state.on {
                state.execute(instr);
            } else {
                state.instr_ptr = find_match(self.instructions, state.instr_ptr);
                state.on = true;
            }
            state.instr_ptr += 1;
        }
    }
}
