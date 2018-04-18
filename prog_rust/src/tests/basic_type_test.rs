#[cfg(test)]
mod test {
    #[test]
    pub fn basic_type_test() {
        assert_eq!(10_i8 as i16, 10_i16);
        assert_eq!(1000_i16 as u8, 232_u8);
        assert_eq!(65535_u16 as i32, 65535_i32);
        assert_eq!(65535_u32 as i16, -1_i16);
    }
}
