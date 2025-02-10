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
