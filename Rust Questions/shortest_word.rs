fn find_shortestword(s: &str) -> &str {
    s.split_whitespace()
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    let shortest_word = find_shortestword(sentence);
    println!("The shortest word in the sentence is: {}", shortest_word);
}
