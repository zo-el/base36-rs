use failure::*;

const ALPHABET: &[u8] = b"0123456789abcdefghijklmnoqprstuvwxyz";

pub fn decode(s: &str) -> Fallible<Vec<u8>> {
    base_x::decode(ALPHABET, s).map_err(|e| e.into())
}

pub fn encode(buf: &[u8]) -> String {
    base_x::encode(ALPHABET, buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("fg3h7vpw7een6jwwnzmq").unwrap(), b"Hello, World!");
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(b"Hello, World!"), "fg3h7vpw7een6jwwnzmq");
    }
}
