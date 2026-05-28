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

fn find_shortest_generator(
    rules: &Vec<Rule>,
    target: &str,
    molecule: &str,
    applied_rules: usize,
) -> Option<usize> {
    if molecule.len() > target.len() {
        None
    } else {
        if molecule == target {
            Some(applied_rules)
        } else {
            generate_molecules(rules, molecule)
                .iter()
                .filter_map(|molecule| {
                    find_shortest_generator(rules, target, molecule, applied_rules + 1)
                })
                .min()
        }
    }
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

    // println!("Rules: {:#?}", rules);
    // println!("Medicine: {}", medicine);

    let new = generate_molecules(&rules, &medicine);

    println!("{}", new.into_iter().count());

    println!("{:?}", find_shortest_generator(&rules, &medicine, "e", 0));
}
