use std::fs::File;
use std::io::Read;

pub struct FileSignature;

impl FileSignature {
    pub const GZIP: [u8; 3] = [0x1f, 0x8b, 0x8];
    pub const BZIP2: [u8; 3] = [0x42, 0x5a, 0x68];

    pub fn get(path: &str) -> Option<&[u8]> {
        let mut file = File::open(path).unwrap();
        let mut buffer = [0u8; 3];
        file.read(&mut buffer).unwrap();

        if buffer == FileSignature::GZIP {
            Some(&FileSignature::GZIP)
        } else if buffer == FileSignature::BZIP2 {
            Some(&FileSignature::BZIP2)
        } else {
            None
        }
    }

    pub fn is_a(source_bytes: [u8; 3], bytes: [u8; 3]) -> bool {
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
        assert_eq!(FileSignature::is_a([0x1f, 0x8b, 0x8], FileSignature::GZIP), true);
    }

    #[test]
    fn it_is_not_a_gzip_file() {
        assert_eq!(FileSignature::is_a([0x1f, 0x9a, 0x8], FileSignature::GZIP), false);
    }
}
