mod c;

#[cfg(test)]
mod tests {
    use crate::{decode, encode};
    #[test]
    fn safe80() {
        for _ in 0..100 {
            let bin: [u8; 32] = rand::random();
            assert_eq!(decode(encode(bin)), bin);
        }
    }
}
