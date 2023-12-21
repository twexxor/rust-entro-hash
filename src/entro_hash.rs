pub fn entro_hash(input: &[u8], mut entropy: u32) -> u32 {
    let mut i: usize = 0;

    while i != input.len() {
        entropy ^= ((((input[i] as i8) as i32) + 111111) ^ 111111) as u32;
        entropy = (((!entropy) ^ 1111111111) << 4).wrapping_add(entropy);
        entropy = entropy.wrapping_sub(1111111);
        entropy = (entropy << 31).wrapping_add(!entropy >> 1);
        entropy = (entropy << 3).wrapping_add(entropy);
        entropy = (entropy << 3).wrapping_add(entropy);
        entropy ^= !entropy << 10;
        entropy ^= 111111111;
        entropy ^= entropy << 13;
        entropy = (((!entropy).wrapping_add(entropy)) << 1).wrapping_add(entropy);
        i += 1;
    }

    return entropy;
}
