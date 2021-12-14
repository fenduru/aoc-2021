use nom::{
    bytes::complete::tag,
    character::{
        complete::{newline, space0},
        streaming::space1,
    },
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

fn drawn_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), nom::character::complete::u32)(input)
}

fn row(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    preceded(
        space0,
        separated_list1(space1, map(nom::character::complete::u32, From::from)),
    )(input)
}

fn board(input: &str) -> IResult<&str, super::Board> {
    map(separated_list1(newline, row), |rows| super::Board {
        squares: rows,
    })(input)
}

pub fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<super::Board>)> {
    separated_pair(
        drawn_numbers,
        many1(newline),
        separated_list1(many1(newline), board),
    )(input)
}
