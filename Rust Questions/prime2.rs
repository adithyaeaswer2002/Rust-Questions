fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numbers = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    for num in &numbers {
        println!("{} is prime: {}", num, is_prime(*num));
    }
}
