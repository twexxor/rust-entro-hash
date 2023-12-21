pub mod entro_hash;

#[cfg(test)]
mod test {
    use entro_hash::entro_hash;

    #[test]
    fn test_entro_hash() {
        let input1 = "message1";
        let input2 = "message2";
        let mut digest = entro_hash(input1.as_bytes(), 0);

        digest = entro_hash(input2.as_bytes(), digest);
        assert_eq!(0x3E2A8375, digest);
    }
}
