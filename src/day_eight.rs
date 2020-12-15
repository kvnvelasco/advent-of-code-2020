use crate::day_eight::Op::{Acc, Jump, Nop};
use crate::utils::split_once_at;

#[derive(Debug)]
enum Op {
    Jump(isize),
    Nop,
    Acc(isize),
}
#[derive(Debug)]
struct Program {
    trace: Vec<isize>,
    acc: isize,
    program_counter: isize, // this being negative is a good way to fck up a day
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
            "nop" => Nop,
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

    fn program_step(&mut self) {
        let instruction = &mut self.instructions[self.program_counter as usize - 1]; // expect this to panic with index out of bounds access

        if (instruction.executed) {
            println!(
                "Executing previously executed instruction, panic, dump, acc: {}",
                self.acc
            );
            println!("Instruction trace: {:?}", self.trace);
            panic!("Program halt")
        }

        match instruction.operation {
            Jump(pos) => self.program_counter += pos,
            Nop => self.program_counter += 1,
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
    }

    fn execute(&mut self) {
        loop {
            if self.program_counter as usize >= self.instructions.len() {
                // program has come to a halt
                return;
            }
            self.program_step()
        }
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
    #[should_panic]
    fn program_is_executable() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight.test.txt"));
        program.execute();
    }

    #[test]
    #[should_panic]
    fn program_a() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight.txt"));
        program.execute();
    }

    #[test]
    fn program_a_fixed() {
        let mut program = Program::parse_from_text(include_str!("inputs/day_eight_fixed.txt"));
        program.execute();

        assert_eq!(program.acc, 2060)
    }
}
