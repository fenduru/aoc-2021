pub fn parse(input: &str) -> Vec<u32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}
