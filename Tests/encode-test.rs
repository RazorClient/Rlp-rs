use rlp_lib::{RlpItem, rlp_encode};

// test vector found at 
// https://developer.electroneum.com/advanced/data-structures-and-encoding/recursive-length-prefix-rlp
// TODO :
// https://github.com/ethereum/tests/blob/develop/RLPTests/rlptest.json


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_dog() {
        let item = RlpItem::String(b"dog".to_vec());
        let encoded = rlp_encode(&item);
        let expected = vec![0x83, b'd', b'o', b'g'];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_list_cat_dog() {
        let item = RlpItem::List(vec![
            RlpItem::String(b"cat".to_vec()),
            RlpItem::String(b"dog".to_vec()),
        ]);
        let encoded = rlp_encode(&item);
        let expected = vec![
            0xc8, // List prefix for length 8
            0x83, b'c', b'a', b't', // "cat"
            0x83, b'd', b'o', b'g', // "dog"
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_string() {
        let item = RlpItem::String(vec![]);
        let encoded = rlp_encode(&item);
        let expected = vec![0x80];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_list() {
        let item = RlpItem::List(vec![]);
        let encoded = rlp_encode(&item);
        let expected = vec![0xc0];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_integer_zero() {
        let item = RlpItem::String(vec![]);
        let encoded = rlp_encode(&item);
        let expected = vec![0x80]; // Represents integer zero as empty string
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_zero() {
        let item = RlpItem::String(vec![0x00]);
        let encoded = rlp_encode(&item);
        let expected = vec![0x00];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_fifteen() {
        let item = RlpItem::String(vec![0x0f]);
        let encoded = rlp_encode(&item);
        let expected = vec![0x0f];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_1024() {
        let item = RlpItem::String(vec![0x04, 0x00]); // 1024 in big-endian bytes
        let encoded = rlp_encode(&item);
        let expected = vec![0x82, 0x04, 0x00];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_nested_lists() {
        // Represents [ [], [[]], [ [], [[]] ] ]
        let item = RlpItem::List(vec![
            RlpItem::List(vec![]),
            RlpItem::List(vec![
                RlpItem::List(vec![])
            ]),
            RlpItem::List(vec![
                RlpItem::List(vec![]),
                RlpItem::List(vec![
                    RlpItem::List(vec![])
                ]),
            ]),
        ]);
        let encoded = rlp_encode(&item);
        let expected = vec![
            0xc7,
            0xc0,
            0xc1, 0xc0,
            0xc3, 0xc0, 0xc1, 0xc0,
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_long_string() {
        let s = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit".to_vec();
        let item = RlpItem::String(s.clone());
        let encoded = rlp_encode(&item);
        let mut expected = vec![0xb8, 0x38]; // 0xb7 + length of length (1), length (56)
        expected.extend(s);
        assert_eq!(encoded, expected);
    }

    #[test]
    fn bartik(){
        let input = RlpItem::List(vec![
            RlpItem::String(b"hello".to_vec()),
            RlpItem::String(b"world".to_vec()),
        ]);
        let expected = vec![
            0xcc,                   // List prefix: 0xc0 + length (12)
            0x85,                   // String prefix: 0x80 + length (5)
            b'h', b'e', b'l', b'l', b'o', // "hello"
            0x85,                   // String prefix: 0x80 + length (5)
            b'w', b'o', b'r', b'l', b'd', // "world"
        ];

        // Encode the input using the RLP encoding function.
        let encoded = rlp_encode(&input);

        // Assert that the encoded output matches the expected output.
        assert_eq!(encoded, expected);
    }
#[test]
    fn test_encode_hello_world_list() {
        let input: RlpItem = vec!["hello".into(), "world".into()].into();

        let expected = vec![
            0xcc,                   
            0x85,                  
            b'h', b'e', b'l', b'l', b'o', 
            0x85,                   
            b'w', b'o', b'r', b'l', b'd', 
        ];
        let encoded = rlp_encode(&input);
        assert_eq!(encoded, expected);
    }
}

