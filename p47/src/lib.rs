pub trait Expr {
    fn and(&self, flg: &bool) -> bool;
    fn or(&self, flg: &bool) -> bool;
    fn nand(&self, flg: &bool) -> bool;
    fn nor(&self, flg: &bool) -> bool;
    fn xor(&self, flg: &bool) -> bool;
    fn impl_(&self, flg: &bool) -> bool;
    fn equ(&self, flg: &bool) -> bool;
    fn not(&self) -> bool;
}

impl Expr for bool {
    fn and(&self, flg: &bool) -> bool {
        p46::and(*self, *flg)
    }

    fn or(&self, flg: &bool) -> bool {
        p46::or(*self, *flg)
    }

    fn nand(&self, flg: &bool) -> bool {
        p46::nand(*self, *flg)
    }

    fn nor(&self, flg: &bool) -> bool {
        p46::nor(*self, *flg)
    }

    fn xor(&self, flg: &bool) -> bool {
        p46::xor(*self, *flg)
    }

    fn impl_(&self, flg: &bool) -> bool {
        p46::impl_(*self, *flg)
    }

    fn equ(&self, flg: &bool) -> bool {
        p46::equ(*self, *flg)
    }

    fn not(&self) -> bool {
        !*self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            p46::table(|a: bool, b: bool| a.and(&a.or(&b.not()))),
            vec![
                (true, true, true),
                (true, false, true),
                (false, true, false),
                (false, false, false)
            ]
        );
    }
}
