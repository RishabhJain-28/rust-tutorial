fn main() {
    let a;
    a = 10;
    println!("{a}");
    let mut a;
    a = 100;
    println!("{a}");
    a = 200;
    println!("{a}");
    let b: i32 = another_funtion();
    println!("{b}");
    let sixth_fib = fib(6);
    println!("sixth_fib: {sixth_fib}");
}

fn another_funtion() -> i32 {
    println!("This is another function");
    let x;
    x = 6;
    x
}

fn fib(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}
