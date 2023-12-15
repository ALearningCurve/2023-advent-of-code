use std::fs;

static PATH: &'static str = "./src/input.txt";


fn main() {
    let file = fs::read_to_string(PATH).unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut num_found = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                let digit: u32 = char.to_digit(10).unwrap();
                num_found += 1;
                match num_found {
                    1 => first = digit,
                    _ => last = digit
                }
            }
        }

        if num_found == 1 {
            last = first;
        }
        sum += first * 10 + last;
        println!("{}{}", first, last);
    }
    println!("{}", sum);
}
