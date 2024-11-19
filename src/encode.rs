/// Represents an item in RLP encoding, which can be either a string (byte array) or a list of items.
/// https://developer.electroneum.com/advanced/data-structures-and-encoding/recursive-length-prefix-rlp
#[derive(Clone, Debug)]


/// Todo : our user is having to take give bit string changethat and make fun to take any string and pass through
pub enum RlpItem {
    String(Vec<u8>),
    List(Vec<RlpItem>),
}

impl From<Vec<&str>> for RlpItem {
    fn from(strings: Vec<&str>) -> Self {
        RlpItem::List(
            strings
                .into_iter()
                .map(|s| RlpItem::String(s.as_bytes().to_vec()))
                .collect(),
        )
    }
}

/// Encodes an `Item` into its RLP byte representation.
pub fn rlp_encode(item: &RlpItem) -> Vec<u8> {
    match item {
        RlpItem::String(s) => encode_string(s),
        RlpItem::List(items) => encode_list(items),
    }
}


/// Encodes a string (byte array) according to RLP rules.
fn encode_string(s: &[u8]) -> Vec<u8> {
    match s.len() {
        // Empty string encoding
        0 => vec![0x80], 
        // Single byte value in between [0x00, 0x7f]
        1 if s[0] < 0x80 => vec![s[0]], 
        len if len <= 55 => {
            //Strings 1-55 bytes long
            //if a string is 0-55 bytes long, the RLP encoding consists of a single byte with value 0x80 
            //plus the length of the string followed by the string. 
            let prefix = 0x80 + len as u8;
            let mut encoded = Vec::with_capacity(1 + len);
            encoded.push(prefix);
            encoded.extend_from_slice(s);
            encoded
        }
        len => {
            // Strings longer than 55 bytes
            let len_bytes = {
                let bytes = len.to_be_bytes();
                let first_non_zero = bytes.iter().position(|&b| b != 0).unwrap();
                bytes[first_non_zero..].to_vec()
            };
            let prefix = 0xb7 + len_bytes.len() as u8;
            let mut encoded = Vec::with_capacity(1 + len_bytes.len() + len);
            encoded.push(prefix);
            encoded.extend_from_slice(&len_bytes);
            encoded.extend_from_slice(s);
            encoded
        }
    }
}


fn encode_list(items: &[RlpItem]) -> Vec<u8> {
    // Encode each item and compute the total length.
    let mut total_len = 0;
    let mut encoded_items = Vec::with_capacity(items.len());

    for item in items {
        let encoded_item = rlp_encode(item);
        total_len += encoded_item.len();
        encoded_items.push(encoded_item);
    }

    // Encode the length prefix for the list.
    let prefix = encode_length(total_len, 0xc0);

    // Preallocate the final vector with the exact capacity needed.
    let mut encoded = Vec::with_capacity(prefix.len() + total_len);

    // Append the prefix and the encoded items.
    encoded.extend_from_slice(&prefix);
    for item in encoded_items {
        encoded.extend_from_slice(&item);
    }

    encoded
}



/// Encodes the length of data with the given offset.
fn encode_length(len: usize, offset: u8) -> Vec<u8> {
    if len < 56 {
        // For lengths less than 56, the length is encoded in a single byte.
        vec![offset + len as u8]
    } else {
        // For lengths >= 56, encode the length of the length.
        let len_bytes = to_binary(len);
        let mut result = vec![offset + 55 + len_bytes.len() as u8];
        result.extend(len_bytes);
        result
    }
}

/// Converts a usize integer into its big-endian byte representation without leading zeros.
fn to_binary(mut x: usize) -> Vec<u8> {
    if x == 0 {
        vec![0]
    } else {
        let mut bytes = Vec::new();
        while x > 0 {
            bytes.push((x % 256) as u8);
            x /= 256;
        }
        bytes.reverse();
        bytes
    }
}
