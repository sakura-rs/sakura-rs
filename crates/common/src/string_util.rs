pub fn get_string_hash(s: &str) -> u32 {
    s.chars().fold(0, |hash, c| {
        (((c as u64) + 131 * hash as u64) & 0xFFFFFFFF) as u32
    })
}
