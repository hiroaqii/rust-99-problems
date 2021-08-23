/// Truth tables for logical expressions.
/// Define predicates and/2, or/2, nand/2, nor/2, xor/2, impl/2 and equ/2 (for logical equivalence) which succeed or fail according to the result of their respective operations;
/// e.g. and(A,B) will succeed, if and only if both A and B succeed. Note that A and B can be Prolog goals (not only the constants true and fail).

pub fn and(x: bool, y: bool) -> bool {
    x && y
}

pub fn or(x: bool, y: bool) -> bool {
    x || y
}

pub fn nand(x: bool, y: bool) -> bool {
    !(x && y)
}

pub fn nor(x: bool, y: bool) -> bool {
    !(x || y)
}

pub fn xor(x: bool, y: bool) -> bool {
    x ^ y
}

pub fn impl_(x: bool, y: bool) -> bool {
    !x || y
}

pub fn equ(x: bool, y: bool) -> bool {
    x == y
}

pub fn table(f: fn(bool, bool) -> bool) -> Vec<(bool, bool, bool)> {
    vec![
        (true, true, f(true, true)),
        (true, false, f(true, false)),
        (false, true, f(false, true)),
        (false, false, f(false, false)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            table(|x, y| and(x, or(x, y))),
            vec![
                (true, true, true),
                (true, false, true),
                (false, true, false),
                (false, false, false)
            ]
        );
    }
}
