use itertools::Itertools;

fn get_increase_frequency<T: PartialOrd>(values: &[T]) -> usize {
    values
        .iter()
        .tuple_windows()
        .filter(|(first, second)| second > first)
        .count()
}

fn numeric_lines(input: &str) -> Vec<usize> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

pub fn solution(input: &str) -> usize {
    get_increase_frequency(&numeric_lines(input))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day_1a::get_increase_frequency;

    #[test]
    fn increasing() {
        let input = [1, 2, 3];
        assert_eq!(get_increase_frequency(&input), 2)
    }

    #[test]
    fn decreasing() {
        let input = [3, 2, 1];
        assert_eq!(get_increase_frequency(&input), 0)
    }

    #[test]
    fn plateaus() {
        let input = [1, 2, 2, 3];
        assert_eq!(get_increase_frequency(&input), 2)
    }

    #[test]
    fn mixed() {
        let input = [1, 2, 3, 2, 3, 2];
        assert_eq!(get_increase_frequency(&input), 3)
    }
}
