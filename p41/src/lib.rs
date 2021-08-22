/// A list of Goldbach compositions.
/// Given a range of integers by its lower and upper limit, print a list of all even numbers and their Goldbach composition.

pub fn goldbach_list(start: u32, end: u32) -> Vec<(u32, u32)> {
    (start..=end)
        .filter(|x| x % 2 == 0)
        .map(p40::goldbach)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            goldbach_list(9, 20),
            vec![(3, 7), (5, 7), (3, 11), (3, 13), (5, 13), (3, 17)]
        );
    }
}
