mod parse;

use std::default::Default;
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Default)]
struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

pub struct Solution;

impl crate::Solution for Solution {
    type Input = Vec<Command>;

    type Output1 = u32;

    type Output2 = u32;

    fn name(&self) -> &'static str {
        "day_2"
    }

    fn parse(&self, input: &str) -> Self::Input {
        parse::parse(input)
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output1 {
        let final_position =
            input
                .iter()
                .fold(Default::default(), |acc: Position, curr| match curr {
                    Command::Forward(x) => Position {
                        horizontal: acc.horizontal + x,
                        ..acc
                    },
                    Command::Down(x) => Position {
                        depth: acc.depth + x,
                        ..acc
                    },
                    Command::Up(x) => Position {
                        depth: acc.depth - x,
                        ..acc
                    },
                });

        final_position.horizontal * final_position.depth
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output2 {
        let final_position =
            input
                .iter()
                .fold(Default::default(), |acc: Position, curr| match curr {
                    Command::Forward(x) => Position {
                        horizontal: acc.horizontal + x,
                        depth: acc.depth + acc.aim * x,
                        ..acc
                    },
                    Command::Down(x) => Position {
                        aim: acc.aim + x,
                        ..acc
                    },
                    Command::Up(x) => Position {
                        aim: acc.aim - x,
                        ..acc
                    },
                });

        final_position.horizontal * final_position.depth
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Command::*;
    use super::*;

    #[test]
    fn part_one() {
        let input = vec![Down(5), Forward(2), Down(3), Forward(4), Up(7), Down(1)];

        // (6, 2) multiplied = 12
        assert_eq!(Solution.part_one(&input), 12);
    }
}
