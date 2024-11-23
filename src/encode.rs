/// Represents an item in RLP encoding, which can be either a string (byte array) or a list of items.
/// https://developer.electroneum.com/advanced/data-structures-and-encoding/recursive-length-prefix-rlp



/// Todo : our user is having to take give bit string changethat and make fun to take any string and pass through
/// make more fixes / think of optimastions and also look at 
/// https://github.com/alloy-rs/rlp/blob/main/crates/rlp/src/encode.rshttps://github.com/alloy-rs/rlp/blob/main/crates/rlp/src/encode.rs
/// https://github.com/paritytech/parity-common/blob/master/rlp/src/stream.rs
#[derive(Clone, Debug)]
pub enum RlpItem {
    //string of represented as raw bytes.
    // 
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
impl From<String> for RlpItem {
    fn from(s: String) -> Self {
        RlpItem::String(s.into_bytes())
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
        vec![offset + len as u8]
    } else {
        // Calculate the big-endian bytes of `len` without leading zeros.
        let mut temp_len = len;
        let mut len_bytes = [0u8; std::mem::size_of::<usize>()];
        let mut i = len_bytes.len();

        while temp_len > 0 {
            i -= 1;
            //extracts least significant digit
            len_bytes[i] = (temp_len & 0xFF) as u8;
            //shifts temp_len right by 8 bits
            temp_len >>= 8;
        }

        let len_of_len = len_bytes.len() - i;
        let mut result = Vec::with_capacity(1 + len_of_len);
        result.push(offset + 55 + len_of_len as u8);
        result.extend_from_slice(&len_bytes[i..]);
        result
    }
}

