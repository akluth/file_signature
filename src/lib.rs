use std::fs::File;
use std::io::Read;

pub struct Type;

impl Type {
    pub const GZIP: [u8; 3] = [0x1f, 0x8b, 0x8];
    pub const BZIP2: [u8; 3] = [0x42, 0x5a, 0x68];
    pub const UNKNOWN: bool = false;
}

pub struct FileSignature;

impl FileSignature {

    fn get_bytes(path: &str) -> [u8; 3] {
        let mut file = File::open(path).unwrap();
        let mut buffer = [0u8; 3];
        file.read(&mut buffer).unwrap();

        buffer
    }

    pub fn get(path: &str) -> [u8; 3] {
        FileSignature::get_bytes(path)
    }

    pub fn compare_bytes(source_bytes: [u8; 3], bytes: [u8; 3]) -> bool {
        if source_bytes == bytes {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_a_gzip_file() {
        assert_eq!(FileSignature::compare_bytes([0x1f, 0x8b, 0x8], Type::GZIP), true);
    }

    #[test]
    fn it_is_not_a_gzip_file() {
        assert_eq!(FileSignature::compare_bytes([0x1f, 0x9a, 0x8], Type::GZIP), false);
    }
}
