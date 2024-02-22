pub fn fibo(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}

pub fn fibo_rec(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibo(n - 1) + fibo(n - 2)
}
