fn is_palindrome(s: &str) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;

    while i < j {
        if s.as_bytes()[i] != s.as_bytes()[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn main() {
    let str1 = "racecar";
    let str2 = "hello";

    println!("{} is a palindrome: {}", str1, is_palindrome(str1));
    println!("{} is a palindrome: {}", str2, is_palindrome(str2));
}
