use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Rule {
    from: String,
    to: String,
}

fn generate_molecules(rules: &Vec<Rule>, medicine: &str) -> HashSet<String> {
    let mut new: HashSet<String> = HashSet::new();

    for (i, c) in medicine.char_indices() {
        for rule in rules {
            if rule.from.len() == 1 {
                if rule.from.chars().next().unwrap() == c {
                    let mut mut_medicine = medicine.to_string();
                    mut_medicine.replace_range(i..i + 1, &rule.to);
                    new.insert(mut_medicine);
                }
            }
        }
    }

    let mut prev = None;
    for (i, c) in medicine.char_indices() {
        if let Some(p) = prev {
            for rule in rules {
                if rule.from.len() == 2 {
                    let mut rule_from_iter = rule.from.chars();
                    if rule_from_iter.next().unwrap() == p && rule_from_iter.next().unwrap() == c {
                        let mut mut_medicine = medicine.to_string();
                        mut_medicine.replace_range(i - 1..i + 1, &rule.to);
                        new.insert(mut_medicine);
                    }
                }
            }
        }
        prev = Some(c);
    }

    new
}

/// Part two uses the analysis from `askalski` provided on the
/// [Day 19 solution megathread](https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/).
fn find_shortest_generator(target: &str) -> usize {
    let num_tokens = target.chars().filter(|c| c.is_ascii_uppercase()).count();

    let re_rn_ar = Regex::new(r"Rn|Ar").expect("Unable to compile regex");
    let count_rn_ar = re_rn_ar.find_iter(target).count();

    let re_y = Regex::new(r"Y").expect("Unable to compile regex");
    let count_y = re_y.find_iter(target).count();

    num_tokens - count_rn_ar - 2 * count_y - 1
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut medicine: String = String::from("");
    let mut rules: Vec<Rule> = vec![];

    let mut found_empty = false;
    for line in lines {
        if found_empty {
            medicine = line.trim().to_owned();
        } else if line.len() == 0 {
            found_empty = true;
        } else {
            let mut bits = line.split(" => ");
            rules.push(Rule {
                from: bits.next().unwrap().to_owned(),
                to: bits.next().unwrap().to_owned(),
            })
        }
    }

    let new = generate_molecules(&rules, &medicine);

    println!("{}", new.into_iter().count());

    println!("{}", find_shortest_generator(&medicine));
}
