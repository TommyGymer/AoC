use regex::Regex;

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u64,
    time: u64,
    rest: u64,
}

fn main() {
    let re = Regex::new(
        r"([a-zA-Z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let reindeer: Vec<Reindeer> = re
        .captures_iter(&input)
        .map(|item| {
            let (_, [name, speed, time, rest]) = item.extract();
            Reindeer {
                name: String::from(name),
                speed: u64::from_str_radix(speed, 10).unwrap(),
                time: u64::from_str_radix(time, 10).unwrap(),
                rest: u64::from_str_radix(rest, 10).unwrap(),
            }
        })
        .collect();

    println!("{:?}", reindeer);

    let race_length = 2503;
    // let race_length = 1000;

    let fastest = reindeer
        .iter()
        .map(|r| {
            let full = race_length / (r.time + r.rest);
            let part = race_length % (r.time + r.rest);

            (
                &r.name,
                (r.speed * (r.time * full)) + (part.min(r.time) * r.speed),
            )
        })
        .reduce(|best, next| if best.1 > next.1 { best } else { next });

    println!("{:?}", fastest);
}
