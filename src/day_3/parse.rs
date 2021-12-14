use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{eof, map, opt},
    multi::{many1, separated_list1},
    sequence::{pair, terminated},
    Finish, IResult,
};

fn one(input: &str) -> IResult<&str, bool> {
    map(tag("1"), |_| true)(input)
}

fn zero(input: &str) -> IResult<&str, bool> {
    map(tag("0"), |_| false)(input)
}

fn bit(input: &str) -> IResult<&str, bool> {
    alt((zero, one))(input)
}

fn binary_number(input: &str) -> IResult<&str, Vec<bool>> {
    many1(bit)(input)
}

pub fn parse(input: &str) -> Vec<Vec<bool>> {
    terminated(
        separated_list1(newline, binary_number),
        pair(opt(newline), eof),
    )(input)
    .finish()
    .unwrap()
    .1
}

#[cfg(test)]
mod test {
    use nom::Finish;

    #[test]
    fn one() {
        assert!(matches!(super::one("101").finish(), Ok(("01", true))))
    }

    #[test]
    fn zero() {
        assert!(matches!(super::zero("010").finish(), Ok(("10", false))))
    }

    #[test]
    fn binary_number() {
        assert!(matches!(
            super::binary_number("101").finish().unwrap().1.as_slice(),
            [true, false, true]
        ));
    }

    #[test]
    fn parse() {
        let numbers = super::parse("101\n110");
        assert_eq!(numbers.len(), 2);
        assert!(matches!(numbers[0].as_slice(), [true, false, true]));
        assert!(matches!(numbers[1].as_slice(), [true, true, false]));
    }
}
