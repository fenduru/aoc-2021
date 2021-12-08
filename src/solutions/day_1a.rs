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

fn numeric_lines(input: &str) -> Vec<u32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

pub fn solution_a(input: &str) -> usize {
    count_increases(&numeric_lines(input))
}

pub fn solution_b(input: &str) -> usize {
    let smoothed_data = smooth_data(&numeric_lines(input), 3);
    count_increases(&smoothed_data)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day_1a::{count_increases, smooth_data};

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
