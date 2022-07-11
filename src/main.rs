mod fibbonaci;

fn main() {
    let n = 10;
    let fb = fibbonaci::nth_fibbonaci(n);
    println!("{n}th fibbonaci - {fb}");
}
