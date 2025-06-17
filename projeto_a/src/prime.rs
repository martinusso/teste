use std::thread;

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime_parallel(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut handles = Vec::new();
    for i in 2..=((n as f64).sqrt() as u32) {
        handles.push(thread::spawn(move || n % i == 0));
    }
    for handle in handles {
        if handle.join().unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(9), false);
    }

    #[test]
    fn test_is_prime_parallel() {
        assert_eq!(is_prime_parallel(2), true);
        assert_eq!(is_prime_parallel(4), false);
        assert_eq!(is_prime_parallel(7), true);
        assert_eq!(is_prime_parallel(9), false);
    }
}
