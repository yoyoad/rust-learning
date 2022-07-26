use std::env;
mod fibbonaci;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let n = if args.len() >= 2 {
        args[1].to_string().parse::<u128>().unwrap()
    } else {
        println!("{}",args.len());
        0
    };

    let fb = fibbonaci::nth_fibbonaci(&n);
    println!("{n}th fibbonaci - {fb}");
}
