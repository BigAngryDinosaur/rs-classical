
pub fn fib(n: u32) -> u32 {
    match n {
        n if n < 2 => n,
        _ => fib(n - 1) + fib(n - 2)
    }
}

pub fn fib_iter(n: u32) -> u32 {
    let mut pair = (0, 1);
    match n {
        0 | 1 => n,
        _ => {
            for _ in 2..=n {
                pair = (pair.1, pair.0 + pair.1)
            }
            pair.1
        }
    }
}
