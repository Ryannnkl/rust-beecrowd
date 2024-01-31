use std::io;

fn main() {
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");

    let mut numbers = numbers.trim().split_whitespace();
    let a = numbers.next().unwrap().parse::<i32>().unwrap();
    let b = numbers.next().unwrap().parse::<i32>().unwrap();
    let c = numbers.next().unwrap().parse::<i32>().unwrap();

    let maior_ab = (a + b + (a - b).abs()) / 2;
    let maior_abc = (maior_ab + c + (maior_ab - c).abs()) / 2;

    println!("{} eh o maior", maior_abc);
}
