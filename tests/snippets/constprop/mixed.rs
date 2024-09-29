fn main(x: u8) {
    let m = x + 5u8 - 2u8; // this won't propagate constants, see README.md
    let n = 5u8 - 2u8 + x; // this will propagate constants
}