mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod input;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        let day = args[1]
            .replace("day", "")
            .parse::<u32>()
            .expect("pick a day number");
        match day {
            1 => day01::run(),
            2 => day02::run(),
            3 => day03::run(),
            4 => day04::run(),
            5 => day05::run(),
            _ => println!("unimplemented day"),
        }
    } else {
        println!("Which day?");
    }
}
