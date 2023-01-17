fn main() {
    println!("Hello, world!");

    let x:u64 = ackermannvorlage(1, 0);
}

fn ackermannvorlage(m: u64, n: u64) -> u64 {
    match(m, n) {
        (0, n) => n+1,
        (m, 0) => ackermann(m-1, 1),
        (m, n) => ackermann(m-1, ackermann(m, n-1))
    }
}