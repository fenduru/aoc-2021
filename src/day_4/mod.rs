mod parse;

#[derive(Clone)]
pub struct Board {
    squares: Vec<Vec<Option<u32>>>,
}

impl Board {
    fn mark(&mut self, num: u32) -> bool {
        for (r, row) in self.squares.iter_mut().enumerate() {
            for (c, square) in row.iter_mut().enumerate() {
                if square == &Some(num) {
                    *square = None;
                    return self.check(r, c);
                }
            }
        }
        false
    }

    fn check(&self, row: usize, col: usize) -> bool {
        let row_complete = self.squares[row].iter().all(|square| *square == None);
        let col_complete = self
            .squares
            .iter()
            .map(|row| row[col])
            .all(|square| square == None);

        row_complete || col_complete
    }

    fn score(&self) -> u32 {
        self.squares
            .iter()
            .flat_map(|row| row.iter())
            .flatten()
            .sum()
    }
}

pub struct Input {
    boards: Vec<Board>,
    drawn_numbers: Vec<u32>,
}
pub struct Solution;

impl crate::Solution for Solution {
    type Input = Input;
    type Output1 = u32;
    type Output2 = u32;

    fn name(&self) -> &'static str {
        "day_4"
    }

    fn parse(&self, input: &str) -> Self::Input {
        let (_, (drawn_numbers, boards)) = parse::parse(input).unwrap();
        Input {
            boards,
            drawn_numbers,
        }
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output1 {
        let mut boards = input.boards.clone();

        for number in &input.drawn_numbers {
            for board in boards.iter_mut() {
                let is_complete = board.mark(*number);
                if is_complete {
                    return board.score() * number;
                }
            }
        }

        panic!("No winner");
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output2 {
        let mut boards = input.boards.clone();

        let (_, score) = boards
            .iter_mut()
            .map(|board| {
                for (turn, number) in input.drawn_numbers.iter().enumerate() {
                    let is_complete = board.mark(*number);
                    if is_complete {
                        return Some((turn, board.score() * number));
                    }
                }
                None
            })
            .flatten()
            .max_by_key(|(turn, _)| *turn)
            .expect("None of the boards win");

        score
    }
}
