/// An n-bit Gray code is a sequence of n-bit strings constructed according to certain rules. For example,
/// n = 1: C(1) = ['0','1'].
/// n = 2: C(2) = ['00','01','11','10'].
/// n = 3: C(3) = ['000','001','011','010',´110´,´111´,´101´,´100´].

fn gran(n: u32) -> Vec<String> {
    match n {
        0 => vec![],
        1 => vec!["0".to_string(), "1".to_string()],
        _ => {
            let vec = gran(n - 1);
            let itr1 = vec.iter().map(|s| s.clone() + "0");
            let itr2 = vec.iter().map(|s| s.clone() + "1");
            let mut ret: Vec<String> = itr1.chain(itr2).collect();
            ret.sort();
            ret
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(vec!["0", "1"], gran(1));
        assert_eq!(vec!["00", "01", "10", "11"], gran(2));
        assert_eq!(
            vec!["000", "001", "010", "011", "100", "101", "110", "111"],
            gran(3)
        );
    }
}
