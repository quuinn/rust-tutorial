use std::io;

fn main() {
    println!("[Fibonacci 數列 n 位產生器]");
    loop {
        let mut user_in: String = String::new();
        println!("請輸入大於 0 的數字");
        io::stdin().read_line(&mut user_in).expect("err!");
        let user_in: i32 = user_in.trim().parse().expect("ERR: 請輸入正整數");

        if user_in > 0 {
            println!(
                "Fibonacci 數列第 {} 位的值為: {}",
                user_in,
                fibonacci(user_in, 1, 1)
            );
            break;
        }
    }
}
fn fibonacci(n: i32, previous: u64, current: u64) -> u64 {
    if n < 2 {
        current
    } else {
        fibonacci(n - 1, current, previous + current)
    }
}
