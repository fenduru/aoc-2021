use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline, u32 as parse_u32};
use nom::combinator::{eof, map_opt, opt};
use nom::multi::separated_list0;
use nom::sequence::{pair, separated_pair, terminated};
use nom::{Finish, IResult};

use super::Command;

fn command(input: &str) -> IResult<&str, Command> {
    map_opt(
        separated_pair(alphanumeric1, tag(" "), parse_u32),
        |(command, value)| match command {
            "forward" => Some(Command::Forward(value)),
            "down" => Some(Command::Down(value)),
            "up" => Some(Command::Up(value)),
            _ => None,
        },
    )(input)
}

fn many_commands(input: &str) -> IResult<&str, Vec<Command>> {
    terminated(separated_list0(newline, command), pair(opt(newline), eof))(input)
}

pub fn parse(input: &str) -> Vec<Command> {
    let (_, result) = many_commands(input).finish().unwrap();
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn command() {
        match super::command("forward 5") {
            Ok((_, super::super::Command::Forward(x))) => assert_eq!(x, 5),
            _ => assert!(false),
        }

        match super::command("down 3") {
            Ok((_, super::super::Command::Down(x))) => assert_eq!(x, 3),
            _ => assert!(false),
        }

        match super::command("up 7") {
            Ok((_, super::super::Command::Up(x))) => assert_eq!(x, 7),
            _ => assert!(false),
        }

        match super::command("bad 7") {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn many_command() {
        match super::many_commands("forward 5\ndown 3") {
            Ok((_, commands)) => assert_eq!(commands.len(), 2),
            _ => assert!(false),
        }
    }
}
