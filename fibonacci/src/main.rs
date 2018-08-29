

fn main() {
    for n in 0..=10 {
        println!("fib {} = {}", n, fibonacci(n));
    }
}

// naive implementation
fn fibonacci(n: u32) -> u32 {
    let mut fib = vec![0, 1, 1];

    if n > 2 {
        for i in fib.len()..=n as usize {
            let first = fib[i - 2];
            let second = fib[i - 1];
            fib.push(first + second);
        }
    }

    return fib[n as usize];
}
