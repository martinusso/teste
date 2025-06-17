mod prime;

fn main() {
    println!("{}", prime::is_prime(11));
    println!("{}", prime::is_prime(42));
    println!("{}", prime::is_prime_parallel(11));
    println!("{}", prime::is_prime_parallel(42));
}
