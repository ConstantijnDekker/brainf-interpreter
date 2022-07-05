const DATA_SIZE: usize = 1 << 15; // 32_768

struct ProgramState {
    data_ptr: usize,
    data: [u8; DATA_SIZE],
    instr_ptr: usize,
    loop_stack: Vec<usize>
}

#[derive(Clone, Copy)]
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

struct Program {
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

    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::IncData => self.data[self.data_ptr] += 1,
            Instruction::DecData => self.data[self.data_ptr] -= 1,
            Instruction::IncDataPtr => self.data_ptr += 1,
            Instruction::DecDataPtr => self.data_ptr -= 1,
            Instruction::Output => { dbg!("Not implemented"); },
            Instruction::Input => { dbg!("Not implemented"); },
            Instruction::WhileNonZero => {
                if self.data[self.data_ptr] == 0 {
                    dbg!("Go to next bracket, not implemented");
                } else {
                    self.loop_stack.push(self.instr_ptr);
                }
            }
            Instruction::EndWhile => {
                let loop_entry = self.loop_stack.pop().unwrap();
                self.instr_ptr = loop_entry;
            }
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

impl Program {
    fn from_str(s: &str) -> Program {
        Program {
            instructions: s.chars().filter_map(to_instruction).collect()
        }
    }

    fn execute(&self) {
        let mut state = ProgramState::new();
    
        for &instr in &self.instructions {
            state.execute(instr);
        }
    }
}
