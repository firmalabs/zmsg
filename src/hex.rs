use hex::{ToHex, FromHex};
use std::str;

/// Convert a unicode string to a hexadecimal string.
pub fn str_to_hex(s: &str) -> Result<String, &str> {    
    let bmsg = s.as_bytes();
    if bmsg.len() > 512 {
        return Err("Message exceeds 512 bytes!");
    }
    Ok(hex::encode(s))
}

/// Convert a hexadecimal string to the corresponding unicode string.
pub fn hex_to_string(s: &str) -> Result<String, &str> {
    if let Ok(v) = hex::decode(s) {
        return Ok(str::from_utf8(&v).unwrap().to_owned());
    }
    Err("Fail to convert hex string to unicode string")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_str_to_hex() {
        let mut result = str_to_hex("hello, world!");
        if let Ok(s) = result {
            assert!(s.len() > 0);
        } else {
            assert!(false);
        }

        result = str_to_hex("สวัสดีชาวโลก");
        if let Ok(s) = result {
            assert!(s.len() > 0);
        } else {
            assert!(false);
        }

        // Check returning error if exceeds 512 bytes.
        let mut a = [0; 513];
        for mut u in a.iter() {
            u = &b'a';
        }
        let msg = str::from_utf8(&a).unwrap();
        match str_to_hex(msg) {
            Ok(_) => assert!(false),
            Err(s) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_hex_to_string() {
        let mut result = str_to_hex("hello, world!");
        if let Ok(s) = result {
            assert!(hex_to_string(s.as_str()).unwrap() == "hello, world!");
        } else {
            assert!(false);
        }
        result = str_to_hex("สวัสดีชาวโลก");
        if let Ok(s) = result {
            assert!(hex_to_string(s.as_str()).unwrap() == "สวัสดีชาวโลก");
        } else {
            assert!(false);
        }
    }
}

