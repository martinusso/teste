pub fn reverse(nums: &[i32]) -> Vec<i32> {
    nums.iter().cloned().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let input = vec![1, 2, 3, 4];
        let expected = vec![4, 3, 2, 1];
        assert_eq!(reverse(&input), expected);
    }
}
