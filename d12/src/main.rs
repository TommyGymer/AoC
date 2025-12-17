struct Shape {
    shape: [[bool; 3]; 3],
}

impl From<&Vec<&str>> for Shape {
    fn from(value: &Vec<&str>) -> Self {
        let mut shape = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                shape[i][j] = if value
                    .get(i)
                    .unwrap()
                    .chars()
                    .into_iter()
                    .collect::<Vec<char>>()
                    .get(j)
                    .unwrap()
                    == &'#'
                {
                    true
                } else {
                    false
                }
            }
        }
        Self { shape }
    }
}

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
}
