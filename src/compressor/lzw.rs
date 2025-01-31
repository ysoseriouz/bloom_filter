use std::collections::HashMap;

type CodeType = u16;
const CODE_SIZE: usize = 2; // in bytes
const DMS: usize = CodeType::MAX as usize; // dictionary last
const INITIAL_CODE: CodeType = CodeType::MAX;

pub fn compress(bytes: &[u8]) -> Vec<u8> {
    let mut dictionary = compress_dictionary();
    let mut output: Vec<u8> = Vec::new();
    let mut curr_code = INITIAL_CODE;

    for &byte in bytes {
        if dictionary.len() == DMS {
            dictionary = compress_dictionary();
        }

        match dictionary.get(&(curr_code, byte)) {
            Some(&code) => {
                curr_code = code;
            }
            None => {
                output.extend_from_slice(&curr_code.to_be_bytes());
                let next_code = dictionary.len() as CodeType;
                dictionary.insert((curr_code, byte), next_code);
                curr_code = *dictionary.get(&(INITIAL_CODE, byte)).unwrap();
            }
        }
    }
    if curr_code != INITIAL_CODE {
        output.extend_from_slice(&curr_code.to_be_bytes());
    }

    output
}

pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut dictionary = decompress_dictionary();
    let mut prev_code = INITIAL_CODE;

    for code in bytes.chunks_exact(CODE_SIZE) {
        let curr_code = CodeType::from_be_bytes([code[0], code[1]]);
        let dict_len = dictionary.len();
        let code_usize = curr_code as usize;

        if dictionary.len() == DMS {
            dictionary = decompress_dictionary();
        }
        if code_usize > dict_len {
            panic!("invalid compressed code");
        }

        // Decompress
        let entry: Vec<u8>;
        if code_usize == dict_len {
            // Case: <char><String><char>
            let tmp = translate(&dictionary, &prev_code);
            dictionary.push((prev_code, tmp[0]));
            entry = translate(&dictionary, &curr_code);
        } else {
            entry = translate(&dictionary, &curr_code);
            if prev_code != INITIAL_CODE {
                dictionary.push((prev_code, entry[0]));
            }
        }
        prev_code = curr_code;
        output.extend_from_slice(&entry);
    }

    output
}

fn compress_dictionary() -> HashMap<(CodeType, u8), CodeType> {
    let mut dictionary = HashMap::with_capacity(DMS);
    for (i, c) in (u8::MIN..u8::MAX).enumerate() {
        dictionary.insert((INITIAL_CODE, c), i as CodeType);
    }
    dictionary
}

fn decompress_dictionary() -> Vec<(CodeType, u8)> {
    let mut dictionary = Vec::with_capacity(DMS);
    for c in u8::MIN..u8::MAX {
        dictionary.push((INITIAL_CODE, c));
    }
    dictionary
}

// Translate encoded string
fn translate(dictionary: &[(CodeType, u8)], code: &CodeType) -> Vec<u8> {
    let mut output = Vec::with_capacity(DMS);
    let mut curr_code = *code;

    while curr_code != INITIAL_CODE {
        let decoded_val = dictionary[curr_code as usize];
        output.push(decoded_val.1);
        curr_code = decoded_val.0;
    }
    output.reverse();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_compression(bytes: &[u8]) {
        let compressed = compress(bytes);
        let decompressed = decompress(&compressed);
        assert_eq!(decompressed, bytes);
    }

    #[test]
    fn test_compressor() {
        assert_compression(b"banana bandana");
        assert_compression(b"abababab");
        assert_compression(b"ababbbab");
        assert_compression(b"");
    }
}
