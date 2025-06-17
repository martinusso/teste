mod palindrome;

fn main() {
    println!("{}", palindrome::is_palindrome("radar"));
    println!("{}", palindrome::is_palindrome("hello"));
    println!("{}", palindrome::is_palindrome("Omissíssimo"));
    println!("{}", palindrome::is_palindrome(""));
}
