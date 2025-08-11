// Demonstrates compile-time overflow detection in a const context.
const _: u8 = 255 + 1; // Line 2: COMPILE ERROR — attempt to compute `256_u8` which overflows
fn main() {}
