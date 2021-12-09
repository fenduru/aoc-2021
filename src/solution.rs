use std::fmt::Display;

pub trait Solution {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn name(&self) -> &'static str;
    fn parse(&self, input: &str) -> Self::Input;
    fn part_one(&self, input: &Self::Input) -> Self::Output1;
    fn part_two(&self, input: &Self::Input) -> Self::Output2;
}
