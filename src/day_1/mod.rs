fn count_increases<T: PartialOrd>(values: &[T]) -> usize {
    values
        .windows(2)
        .filter(|window| match window {
            [first, second] => second > first,
            _ => false,
        })
        .count()
}

fn smooth_data(values: &[u32], samples: usize) -> Vec<f64> {
    values
        .windows(samples)
        .map(|window| f64::from(window.iter().sum::<u32>()) / (window.len() as f64))
        .collect()
}

pub struct Solution;

impl crate::Solution for Solution {
    type Input = Vec<u32>;
    type Output1 = usize;
    type Output2 = usize;

    fn name(&self) -> &'static str {
        "day_1"
    }

    fn parse(&self, input: &str) -> Self::Input {
        input.lines().filter_map(|line| line.parse().ok()).collect()
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output1 {
        count_increases(input)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output2 {
        let smoothed_data = smooth_data(input, 3);
        count_increases(&smoothed_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing() {
        let input = [1, 2, 3];
        assert_eq!(count_increases(&input), 2)
    }

    #[test]
    fn decreasing() {
        let input = [3, 2, 1];
        assert_eq!(count_increases(&input), 0)
    }

    #[test]
    fn plateaus() {
        let input = [1, 2, 2, 3];
        assert_eq!(count_increases(&input), 2)
    }

    #[test]
    fn mixed() {
        let input = [1, 2, 3, 2, 3, 2];
        assert_eq!(count_increases(&input), 3)
    }

    #[test]
    fn smoothed() {
        let input = [1, 5, 3, 4];
        assert_eq!(smooth_data(&input, 3), [3.0, 4.0])
    }
}
