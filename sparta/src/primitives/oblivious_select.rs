#[inline(always)]
/// Returns a if condition, else b
/// An implementation of Oblivious Selection as specified in
/// section 2 of the paper.
///
/// (!(c-1)&a)|((c-1)&b)
///
/// Always inlined to prevent function call
pub fn oblivious_select<T>(condition: bool, a: T, b: T) -> T
where
    T: std::ops::BitAnd<Output = T>
        + std::ops::BitOr<Output = T>
        + std::ops::Not<Output = T>
        + From<u8>
        + Copy,
{
    let c: u8 = condition as u8;

    let zero = T::from(0);
    let mask = if c == 0 { !zero } else { zero };

    (!mask) & a | (mask & b)
}

//TODO: could expand more on testing here
#[cfg(test)]
mod tests {
    use super::oblivious_select;

    #[test]
    fn positive_test() {
        let num = oblivious_select(true, 1, 2);

        assert_eq!(num, 1)
    }

    #[test]
    fn negative_test() {
        let num = oblivious_select(false, 1, 2);
        assert_eq!(num, 2)
    }
}
