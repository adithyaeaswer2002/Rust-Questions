fn reverse_number(num: i32) -> i32 {
    let mut reversed = 0;
    let mut original = num;

    while original != 0 {
        let digit = original % 10;
        reversed = reversed * 10 + digit;
        original /= 10;
    }

    reversed
}

fn main() {
    let number1 = 12345;
    let number2 = -98765;

    println!("The reverse of {} is {}", number1, reverse_number(number1));
    println!("The reverse of {} is {}", number2, reverse_number(number2));
}

