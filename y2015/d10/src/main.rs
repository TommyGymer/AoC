use std::fs;

fn do_round(input: String) -> String {
    let mut out = String::with_capacity(input.len());

    let mut char = None;
    let mut count = 0;
    for c in input.chars() {
        if let Some(m) = char {
            if c == m {
                count += 1;
            } else {
                out.push_str(&format!("{}", count));
                out.push(m);
                char = Some(c);
                count = 1;
            }
        } else {
            char = Some(c);
            count = 1;
        }
    }

    out.push_str(&format!("{}", count));
    out.push(char.unwrap());

    out
}

fn main() {
    let mut input: String = fs::read_to_string("input.txt").unwrap().trim().to_owned();

    for _ in 0..50 {
        input = do_round(input)
    }

    println!("{}", input.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(do_round(String::from("1")), "11")
    }
}
