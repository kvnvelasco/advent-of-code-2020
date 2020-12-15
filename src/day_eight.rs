use crate::day_eight::Op::{Acc, Jump, Nop};
use crate::utils::split_once_at;
use std::error::Error;

#[derive(Debug)]
enum Op {
    Jump(isize),
    Nop(isize),
    Acc(isize),
}
#[derive(Debug)]
struct Program {
    trace: Vec<isize>,
    acc: isize,
    program_counter: isize,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    operation: Op,
    executed: bool,
}

impl Instruction {
    fn parse_from_text(source: &'static str) -> Self {
        let (operation, parameters) = split_once_at(source, " ");
        let operation = match operation {
            "jmp" => Jump(parameters.parse().unwrap()),
            "acc" => Acc(parameters.parse().unwrap()),
            "nop" => Nop(parameters.parse().unwrap()),
            _ => panic!("Unable to parse instruction"),
        };

        Self {
            operation,
            executed: false,
        }
    }
}

impl Program {
    fn parse_from_text(source: &'static str) -> Self {
        let instructions = source.lines().map(Instruction::parse_from_text).collect();
        Self {
            acc: 0,
            program_counter: 1,
            instructions: instructions,
            trace: vec![1],
        }
    }

    fn reset_program(&mut self) {
        self.trace.clear();
        self.program_counter = 1;
        self.acc = 0;
        for instruction in self.instructions.iter_mut() {
            instruction.executed = false;
        }
    }

    fn program_step(&mut self) -> Result<(), Box<dyn Error>> {
        let instruction = &mut self.instructions[self.program_counter as usize - 1]; // expect this to panic with index out of bounds access

        if (instruction.executed) {
            return Err("Previously Executed Instruction".into());
        }

        match instruction.operation {
            Jump(pos) => self.program_counter += pos,
            Nop(_) => self.program_counter += 1,
            Acc(amt) => {
                self.acc += amt;
                self.program_counter += 1
            }
        }

        instruction.executed = true;
        self.trace.push(self.program_counter);

        if self.program_counter < 0 {
            panic!("Program counter has gone to negatives, the world is a lie")
        }

        Ok(())
    }

    fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.program_step()?;
            if self.program_counter as usize >= self.instructions.len() {
                // program has come to a halt
                return Ok(());
            }
        }
    }

    // will attempt to backtrace itself after an error and attempt to correct a flipped instruction
    // consumes self and returns a new uninited program
    fn self_debug(mut self) -> Self {
        // walk over all of the instructions going backwards and attempt to rerun the program
        let mut trace = self.trace.clone();

        loop {
            let active_modification = trace.pop();

            if let Some(active) = active_modification {
                self.reset_program();

                {
                    let instruction = &mut self.instructions[active as usize - 1];
                    match instruction.operation {
                        Jump(to) => instruction.operation = Nop(to),
                        Nop(to) => instruction.operation = Jump(to),
                        _ => {}
                    }
                }
                // attempt to run the program
                let attempt = self.execute();
                if attempt.is_err() {
                    let instruction = &mut self.instructions[active as usize - 1];
                    // rollback and try again
                    match instruction.operation {
                        Jump(to) => instruction.operation = Nop(to),
                        Nop(to) => instruction.operation = Jump(to),
                        _ => {}
                    }
                } else {
                    println!("Found error at {}", active);
                    self.reset_program();

                    return self;
                }
            } else {
                return self;
            };
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::day_eight::Program;

    #[test]
    fn program_is_parsable() {
        let program = Program::parse_from_text(include_str!("inputs/day_eight.test.txt"));

        assert_eq!(program.instructions.len(), 9);
    }

    #[test]
    fn program_is_executable() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight.test.txt"));
        assert!(program.execute().is_err());
    }

    #[test]
    fn program_a() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight.txt"));
        assert!(program.execute().is_err());
    }

    #[test]
    fn program_a_is_self_debuggable() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight.txt"));
        assert!(program.execute().is_err());

        program = program.self_debug();

        assert!(program.execute().is_ok());
        assert_eq!(program.acc, 2060);
    }

    #[test]
    fn program_a_fixed() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight_fixed.txt"));
        program.execute();

        assert_eq!(program.acc, 2060)
    }
}
