pub fn bcd_add_u8(b1: u8, b2: u8) -> u8 {
    let b1_ll = b1 & 0b0000_1111;
    let b1_hh = (b1 & 0b1111_0000) >> 4;
    let b1 = b1_hh * 10 + b1_ll;
    dbg!(b1_ll);
    dbg!(b1_hh);
    dbg!(b1);

    let b2_ll = b2 & 0b0000_1111;
    let b2_hh = (b2 & 0b1111_0000) >> 4;
    let b2 = b2_hh * 10 + b2_ll;
    dbg!(b2_ll);
    dbg!(b2_hh);
    dbg!(b2);

    let sum = b1 + b2;

    dbg!(sum);

    let sum_tens = (sum / 10) << 4;
    let sum_ones = sum % 10;

    sum_tens | sum_ones
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_bcd_add_u8() {
        let sum = super::bcd_add_u8(0b0000_1000, 0b0000_0011);
        assert_eq!(sum, 0b0001_0001);

        // 11 + 22 should yield 33 (0011 0011)
        let sum = super::bcd_add_u8(0b0001_0001, 0b0010_0010);
        assert_eq!(sum, 0b0011_0011);

        // 33 + 33 = 66
        let sum = super::bcd_add_u8(0b0011_0011, 0b0011_0011);
        assert_eq!(sum, 0b0110_0110);

        // 19 + 29 = 48 = 0100 1000
        let sum = super::bcd_add_u8(0b0001_1001, 0b0010_1001);
        assert_eq!(sum, 0b0100_1000);

        // 49 + 50 = 99 = 1001 1001
        let sum = super::bcd_add_u8(0b0100_1001, 0b0101_0000);
        assert_eq!(sum, 0b1001_1001);
    }
}
