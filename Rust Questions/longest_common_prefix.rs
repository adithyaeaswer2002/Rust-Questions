fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    
    let first_str = strs[0].as_str();
    
    for (i, c) in first_str.char_indices() {
        for s in strs.iter().skip(1) {
            match s.chars().nth(i) {
                Some(ch) if ch == c => continue,
                _ => return first_str[..i].to_string(),
            }
        }
    }
    
    first_str.to_string()
}

fn main() {
    let strings = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    
    let prefix = longest_common_prefix(strings);
    println!("The longest common prefix is: {}", prefix);
}
