fn main() {
    let n = 15;
    fibonacci(n);
}

fn fibonacci(n: u32) {
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;
    let mut x: u32 = 0;
    while x < n {
        println!("{}", n1);
        let nth = n1 + n2;
        n1 = n2;
        n2 = nth;
        x += 1;
    }
}
