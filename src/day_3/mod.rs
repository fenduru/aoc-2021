use std::ops::Index;

mod parse;

fn bits_to_u32(bits: Vec<bool>) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0u32, |acc, (i, bit)| acc + ((1 & *bit as u32) << i))
}

fn most_frequent_bits(input: &Vec<Vec<bool>>) -> Vec<Option<bool>> {
    let mut freq = Vec::new();

    for num in input {
        if freq.len() < num.len() {
            freq.resize(num.len(), 0);
        }

        for (i, digit) in num.iter().enumerate() {
            if *digit {
                freq[i] += 1;
            }
        }
    }

    freq.iter()
        .map(|count| {
            if count * 2 == input.len() {
                None
            } else {
                Some(count * 2 > input.len())
            }
        })
        .collect()
}

fn find_rating(input: &Vec<Vec<bool>>, criteria: bool) -> Vec<bool> {
    let mut remaining = input.clone();

    while remaining.len() > 1 {
        for (i, digit) in most_frequent_bits(&remaining)
            .iter()
            .map(|bit| bit.map(|b| b == criteria).unwrap_or(criteria))
            .enumerate()
        {
            remaining.retain(|num| *num.index(i) == digit);

            if remaining.len() == 1 {
                return remaining.remove(0);
            }
        }
    }

    panic!("Could not find rating");
}

pub struct Solution;

impl crate::Solution for Solution {
    type Input = Vec<Vec<bool>>;
    type Output1 = u32;
    type Output2 = u32;

    fn name(&self) -> &'static str {
        "day_3"
    }

    fn parse(&self, input: &str) -> Self::Input {
        parse::parse(input)
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output1 {
        let gamma: Vec<bool> = most_frequent_bits(input)
            .iter()
            .flat_map(|bit| bit.or(Some(false)))
            .collect();

        let epsilon = gamma.iter().map(|bit| !bit).collect();

        bits_to_u32(gamma) * bits_to_u32(epsilon)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output2 {
        let oxygen = find_rating(input, true);
        let co2 = find_rating(input, false);

        bits_to_u32(oxygen) * bits_to_u32(co2)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn bits_to_u32() {
        assert_eq!(super::bits_to_u32(vec![true, false, true]), 5);
    }
}
