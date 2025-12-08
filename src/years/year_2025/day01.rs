use crate::years::{Input, Part};
use anyhow::{Error, anyhow};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Instruction {
    Left(usize),
    Right(usize),
}

fn read_file(file_path: &PathBuf) -> anyhow::Result<Input> {
    std::fs::read_to_string(file_path)
        .map(Input::from)
        .map_err(|err| err.into())
}

fn parse(input: &Input) -> Vec<Instruction> {
    input
        .text
        .split("\r\n")
        .map(String::from)
        .map(parse_line)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn parse_line(line: String) -> anyhow::Result<Instruction> {
    let (side, number) = line.split_at(1);
    match (side.starts_with("L"), side.starts_with("R")) {
        (true, _) => Ok(Instruction::Left(number.parse::<usize>()?)),
        (_, true) => Ok(Instruction::Right(number.parse::<usize>()?)),
        _ => Err(Error::msg("Invalid side"))?,
    }
}

pub type VmStateHandler = fn(rotation: &Rotation, state: usize) -> usize;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct VmState {
    dial: Dial,

    handlers: BTreeMap<Part, VmStateHandler>,
    results: BTreeMap<Part, usize>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Vm {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    state: VmState,
}

impl Vm {
    fn execute(&mut self) {
        if let Some(instruction) = self.instructions.get(self.instruction_pointer) {
            let rotation = match *instruction {
                Instruction::Left(amount) => self.state.dial.left(amount),
                Instruction::Right(amount) => self.state.dial.right(amount),
            };
            self.state.dial = Dial::new(rotation.position, self.state.dial.max);

            for (part, handler) in &self.state.handlers {
                self.state.results.entry(*part).or_default();
                self.state
                    .results
                    .insert(*part, handler(&rotation, self.state.results[part]));
            }

            self.instruction_pointer += 1;
        }
    }

    fn run(&mut self) {
        loop {
            if self.instruction_pointer >= self.instructions.len() {
                break;
            }
            self.execute();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Dial {
    position: usize,
    max: usize,
}

impl Dial {
    pub fn new(position: usize, max: usize) -> Dial {
        Dial { position, max }
    }

    pub fn left(&self, amount: usize) -> Rotation {
        Rotation::new(
            self.rotation_position_left(amount),
            self.rotation_amount_left(amount),
        )
    }

    pub fn right(&self, amount: usize) -> Rotation {
        Rotation::new(
            self.rotation_position_right(amount),
            self.rotation_amount_right(amount),
        )
    }

    fn rotation_position_left(&self, amount: usize) -> usize {
        let tmp = amount % self.max();

        if tmp > self.position {
            self.max() - (tmp - self.position)
        } else {
            self.position - tmp
        }
    }

    fn rotation_amount_left(&self, amount: usize) -> usize {
        Self::rotation_amount_left_inner(self.position, amount, self.max())
    }

    fn rotation_amount_left_inner(position: usize, amount: usize, max: usize) -> usize {
        amount / max
            + if (amount % max) >= position && position != 0 {
                1
            } else {
                0
            }
    }

    fn rotation_position_right(&self, amount: usize) -> usize {
        let tmp = amount % self.max();
        (self.position + tmp) % self.max()
    }

    fn rotation_amount_right(&self, amount: usize) -> usize {
        Self::rotation_amount_right_inner(self.position, amount, self.max())
    }

    fn rotation_amount_right_inner(position: usize, amount: usize, max: usize) -> usize {
        (amount / max) + (amount % max + position) / max
    }

    fn max(&self) -> usize {
        self.max + 1
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            max: 99,
            position: 50,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Rotation {
    position: usize,
    amount: usize,
}

impl Rotation {
    pub fn new(position: usize, amount: usize) -> Rotation {
        Rotation { position, amount }
    }
}

fn setup(instructions: Vec<Instruction>) -> Vm {
    let dial = Dial::default();

    let part_one_handler = |rotation: &Rotation, state: usize| -> usize {
        if rotation.position == 0 {
            state + 1
        } else {
            state
        }
    };

    let part_two_handler = |rotation: &Rotation, state: usize| -> usize { state + rotation.amount };

    let handlers = {
        let mut handlers = BTreeMap::<Part, VmStateHandler>::new();
        handlers.entry(Part::One).or_insert(part_one_handler);
        handlers.entry(Part::Two).or_insert(part_two_handler);

        handlers
    };

    let state = VmState {
        dial,
        handlers,
        results: Default::default(),
    };

    Vm {
        instructions,
        instruction_pointer: 0,
        state,
    }
}

fn solve(input: &Input, part: Part) -> anyhow::Result<usize> {
    {
        let mut vm = setup(parse(input));
        vm.run();
        vm
    }
    .state
    .results
    .get(&Part::Two)
    .copied()
    .ok_or_else(|| anyhow!("Part One not found"))
}

pub fn solve_part1(input: &Input) -> anyhow::Result<usize> {
    solve(input, Part::One)
}

pub fn solve_part2(input: &Input) -> anyhow::Result<usize> {
    solve(input, Part::Two)
}

#[cfg(test)]
mod test {
    use crate::years::year_2025::day01::Dial;
    use rstest::rstest;
    use similar_asserts::assert_eq;

    #[rstest]
    #[case(0, 5, 0)]
    #[case(0, 50, 0)]
    #[case(0, 500, 5)]
    #[case(0, 5000, 50)]
    #[case(0, 100, 1)]
    #[case(0, 1000, 10)]
    fn test_right_amount(#[case] position: usize, #[case] amount: usize, #[case] expected: usize) {
        assert_eq!(
            Dial::rotation_amount_right_inner(position, amount, 100),
            expected
        );
    }

    #[rstest]
    #[case(0, 5, 0)]
    #[case(1, 5, 1)]
    #[case(1, 500, 5)]
    #[case(1, 5000, 50)]
    #[case(99, 99, 1)]
    #[case(99, 50, 0)]
    fn test_left_amount(#[case] position: usize, #[case] amount: usize, #[case] expected: usize) {
        assert_eq!(
            Dial::rotation_amount_left_inner(position, amount, 100),
            expected
        );
    }
}
