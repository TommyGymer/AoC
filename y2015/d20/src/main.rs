fn presents_delivered_to_house_n(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        if n % i == 0 {
            total += i;
        }
    }
    total
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
