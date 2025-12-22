use std::{array, fs};

fn insert_select_n_from_k(battery: &Vec<u8>, n: usize) -> u64 {
    (0..n)
        .fold((0usize, 0u64), |acc, i| {
            let (start, v) = battery.as_slice()[acc.0..(battery.len() - (n - 1) + i) as usize]
                .into_iter()
                .enumerate()
                .reduce(|(acc_i, acc_v), (index, value)| {
                    if value <= acc_v {
                        (acc_i, acc_v)
                    } else {
                        (index, value)
                    }
                })
                .unwrap();
            (acc.0 + start + 1, acc.1 * 10 + *v as u64)
        })
        .1
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let total: u64 = input
        .split('\n')
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect::<Vec<u8>>()
        })
        .filter(|l| l.len() > 0)
        .map(|battery| insert_select_n_from_k(&battery, 2))
        .sum();

    println!("{}", total)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char_to_u32() {
        assert_eq!('5'.to_digit(10).unwrap(), 5)
    }

    #[test]
    fn test_char_to_u64() {
        assert_eq!('5'.to_digit(10).unwrap() as u64, 5)
    }
}
