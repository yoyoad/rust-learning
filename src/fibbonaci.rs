pub fn nth_fibbonaci(n: u32)-> u32 {
    let (mut n1, mut n2) = (0, 1);
    let mut current_value: u32 = 0;
    for _ in 1..n {
        current_value = n1 + n2;
        n1 = n2;
        n2 = current_value;
    }
    current_value
}
