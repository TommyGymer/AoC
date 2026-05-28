use core::iter::Iterator;

fn presents_delivered_to_house_n(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        if n % i == 0 && n / i <= 50 {
            total += i;
        }
    }
    total
}

fn create_lookup_table(n: u64) -> Vec<u64> {
    // the upper bound for the house we are looking for is
    // n, the number of presents delivered off by a factor
    // of 10, as this will be dilvered by the first elf
    let mut lookup_table = vec![0; n as usize];

    // for each elf
    for i in 1..=n {
        for j in (i..=n).step_by(i as usize) {
            lookup_table[j as usize - 1] += i;
        }
    }

    lookup_table
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let bound = u64::from_str_radix(lines.first().expect("Missing the first line"), 10)
        .expect("Input was not number")
        / 10;

    println!("{}", bound);

    for i in 1..10 {
        println!(
            "house {} got {} presents",
            i,
            presents_delivered_to_house_n(i)
        );
    }

    let lookup_table = create_lookup_table(bound);
    for (i, n) in lookup_table.into_iter().enumerate() {
        if n > bound {
            println!("house number {}", i + 1);
            break;
        }
    }

    let mut n = 1;
    while presents_delivered_to_house_n(n) < bound {
        n += 1;
    }

    println!(
        "house {} got {} presents",
        n,
        presents_delivered_to_house_n(n)
    );
}
