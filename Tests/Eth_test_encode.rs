// these dont work for now 

// use rlp_lib::{RlpItem, rlp_encode};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_emptystring() {
//         let item = RlpItem::String(vec![]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x80];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_bytestring00() {
//         let item = RlpItem::String(vec![0x00]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x00];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_bytestring01() {
//         let item = RlpItem::String(vec![0x01]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x01];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_bytestring7F() {
//         let item = RlpItem::String(vec![0x7f]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x7f];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_shortstring() {
//         let item = RlpItem::String(vec![0x64, 0x6f, 0x67]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x83, 0x64, 0x6f, 0x67];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_shortstring2() {
//         let item = RlpItem::String(vec![0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xb7, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_longstring() {
//         let item = RlpItem::String(vec![0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69, 0x74]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xb8, 0x38, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69, 0x74];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_longstring2() {
//         let item = RlpItem::String(vec![0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x2e, 0x20, 0x43, 0x75, 0x72, 0x61, 0x62, 0x69, 0x74, 0x75, 0x72, 0x20, 0x6d, 0x61, 0x75, 0x72, 0x69, 0x73, 0x20, 0x6d, 0x61, 0x67, 0x6e, 0x61, 0x2c, 0x20, 0x73, 0x75, 0x73, 0x63, 0x69, 0x70, 0x69, 0x74, 0x20, 0x73, 0x65, 0x64, 0x20, 0x76, 0x65, 0x68, 0x69, 0x63, 0x75, 0x6c, 0x61, 0x20, 0x6e, 0x6f, 0x6e, 0x2c, 0x20, 0x69, 0x61, 0x63, 0x75, 0x6c, 0x69, 0x73, 0x20, 0x66, 0x61, 0x75, 0x63, 0x69, 0x62, 0x75, 0x73, 0x20, 0x74, 0x6f, 0x72, 0x74, 0x6f, 0x72, 0x2e, 0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x73, 0x75, 0x73, 0x63, 0x69, 0x70, 0x69, 0x74, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20, 0x6d, 0x61, 0x6c, 0x65, 0x73, 0x75, 0x61, 0x64, 0x61, 0x2e, 0x20, 0x44, 0x75, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x72, 0x74, 0x6f, 0x72, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x2c, 0x20, 0x64, 0x69, 0x63, 0x74, 0x75, 0x6d, 0x20, 0x71, 0x75, 0x69, 0x73, 0x20, 0x74, 0x72, 0x69, 0x73, 0x74, 0x69, 0x71, 0x75, 0x65, 0x20, 0x65, 0x75, 0x2c, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x61, 0x74, 0x20, 0x72, 0x69, 0x73, 0x75, 0x73, 0x2e, 0x20, 0x4d, 0x6f, 0x72, 0x62, 0x69, 0x20, 0x61, 0x20, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x65, 0x72, 0x64, 0x69, 0x65, 0x74, 0x20, 0x6d, 0x69, 0x20, 0x75, 0x6c, 0x6c, 0x61, 0x6d, 0x63, 0x6f, 0x72, 0x70, 0x65, 0x72, 0x20, 0x61, 0x6c, 0x69, 0x71, 0x75, 0x65, 0x74, 0x20, 0x73, 0x75, 0x73, 0x63, 0x69, 0x70, 0x69, 0x74, 0x20, 0x6e, 0x65, 0x63, 0x20, 0x6c, 0x6f, 0x72, 0x65, 0x6d, 0x2e, 0x20, 0x41, 0x65, 0x6e, 0x65, 0x61, 0x6e, 0x20, 0x71, 0x75, 0x69, 0x73, 0x20, 0x6c, 0x65, 0x6f, 0x20, 0x6d, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x2c, 0x20, 0x76, 0x75, 0x6c, 0x70, 0x75, 0x74, 0x61, 0x74, 0x65, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x20, 0x76, 0x61, 0x72, 0x69, 0x75, 0x73, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x69, 0x6d, 0x2e, 0x20, 0x4e, 0x75, 0x6c, 0x6c, 0x61, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x74, 0x75, 0x72, 0x70, 0x69, 0x73, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x6f, 0x2c, 0x20, 0x65, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x75, 0x65, 0x72, 0x65, 0x20, 0x75, 0x72, 0x6e, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x6e, 0x65, 0x63, 0x2e, 0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x6e, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x6c, 0x6c, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x2e, 0x20, 0x44, 0x6f, 0x6e, 0x65, 0x63, 0x20, 0x74, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x61, 0x75, 0x72, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x67, 0x75, 0x65, 0x20, 0x73, 0x6f, 0x6c, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x75, 0x64, 0x69, 0x6e, 0x2e, 0x20, 0x56, 0x65, 0x73, 0x74, 0x69, 0x62, 0x75, 0x6c, 0x75, 0x6d, 0x20, 0x61, 0x6e, 0x74, 0x65, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x66, 0x61, 0x75, 0x63, 0x69, 0x62, 0x75, 0x73, 0x20, 0x6f, 0x72, 0x63, 0x69, 0x20, 0x6c, 0x75, 0x63, 0x74, 0x75, 0x73, 0x20, 0x65, 0x74, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x75, 0x65, 0x72, 0x65, 0x20, 0x63, 0x75, 0x62, 0x69, 0x6c, 0x69, 0x61, 0x20, 0x43, 0x75, 0x72, 0x61, 0x65, 0x3b, 0x20, 0x53, 0x75, 0x73, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x73, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x6c, 0x6c, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6d, 0x20, 0x76, 0x65, 0x6c, 0x20, 0x6d, 0x61, 0x73, 0x73, 0x61, 0x20, 0x66, 0x61, 0x75, 0x63, 0x69, 0x62, 0x75, 0x73, 0x2c, 0x20, 0x65, 0x67, 0x65, 0x74, 0x20, 0x6c, 0x61, 0x63, 0x69, 0x6e, 0x69, 0x61, 0x20, 0x6c, 0x61, 0x63, 0x75, 0x73, 0x20, 0x74, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x2e, 0x20, 0x4e, 0x75, 0x6c, 0x6c, 0x61, 0x20, 0x71, 0x75, 0x69, 0x73, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20, 0x70, 0x75, 0x72, 0x75, 0x73, 0x2e, 0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x61, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x72, 0x68, 0x6f, 0x6e, 0x63, 0x75, 0x73, 0x20, 0x6e, 0x69, 0x62, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x75, 0x6d, 0x20, 0x6d, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x2e, 0x20, 0x41, 0x6c, 0x69, 0x71, 0x75, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x69, 0x6d, 0x20, 0x61, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x20, 0x6c, 0x75, 0x63, 0x74, 0x75, 0x73, 0x2c, 0x20, 0x61, 0x20, 0x65, 0x6c, 0x65, 0x69, 0x66, 0x65, 0x6e, 0x64, 0x20, 0x70, 0x75, 0x72, 0x75, 0x73, 0x20, 0x65, 0x67, 0x65, 0x73, 0x74, 0x61, 0x73, 0x2e, 0x20, 0x43, 0x75, 0x72, 0x61, 0x62, 0x69, 0x74, 0x75, 0x72, 0x20, 0x61, 0x74, 0x20, 0x6e, 0x69, 0x62, 0x68, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x2e, 0x20, 0x4e, 0x61, 0x6d, 0x20, 0x62, 0x69, 0x62, 0x65, 0x6e, 0x64, 0x75, 0x6d, 0x2c, 0x20, 0x6e, 0x65, 0x71, 0x75, 0x65, 0x20, 0x61, 0x74, 0x20, 0x61, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x72, 0x69, 0x73, 0x74, 0x69, 0x71, 0x75, 0x65, 0x2c, 0x20, 0x6c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x6c, 0x69, 0x62, 0x65, 0x72, 0x6f, 0x20, 0x61, 0x6c, 0x69, 0x71, 0x75, 0x65, 0x74, 0x20, 0x61, 0x72, 0x63, 0x75, 0x2c, 0x20, 0x6e, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x64, 0x75, 0x6d, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x75, 0x73, 0x20, 0x6c, 0x65, 0x63, 0x74, 0x75, 0x73, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x20, 0x65, 0x72, 0x6f, 0x73, 0x2e, 0x20, 0x43, 0x72, 0x61, 0x73, 0x20, 0x72, 0x68, 0x6f, 0x6e, 0x63, 0x75, 0x73, 0x2c, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x20, 0x61, 0x63, 0x20, 0x6f, 0x72, 0x6e, 0x61, 0x72, 0x65, 0x20, 0x63, 0x75, 0x72, 0x73, 0x75, 0x73, 0x2c, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x6f, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x2c, 0x20, 0x61, 0x74, 0x20, 0x75, 0x6c, 0x6c, 0x61, 0x6d, 0x63, 0x6f, 0x72, 0x70, 0x65, 0x72, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x70, 0x61, 0x74]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xb9, 0x04, 0x00, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x2e, 0x20, 0x43, 0x75, 0x72, 0x61, 0x62, 0x69, 0x74, 0x75, 0x72, 0x20, 0x6d, 0x61, 0x75, 0x72, 0x69, 0x73, 0x20, 0x6d, 0x61, 0x67, 0x6e, 0x61, 0x2c, 0x20, 0x73, 0x75, 0x73, 0x63, 0x69, 0x70, 0x69, 0x74, 0x20, 0x73, 0x65, 0x64, 0x20, 0x76, 0x65, 0x68, 0x69, 0x63, 0x75, 0x6c, 0x61, 0x20, 0x6e, 0x6f, 0x6e, 0x2c, 0x20, 0x69, 0x61, 0x63, 0x75, 0x6c, 0x69, 0x73, 0x20, 0x66, 0x61, 0x75, 0x63, 0x69, 0x62, 0x75, 0x73, 0x20, 0x74, 0x6f, 0x72, 0x74, 0x6f, 0x72, 0x2e, 0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x73, 0x75, 0x73, 0x63, 0x69, 0x70, 0x69, 0x74, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20, 0x6d, 0x61, 0x6c, 0x65, 0x73, 0x75, 0x61, 0x64, 0x61, 0x2e, 0x20, 0x44, 0x75, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x72, 0x74, 0x6f, 0x72, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x2c, 0x20, 0x64, 0x69, 0x63, 0x74, 0x75, 0x6d, 0x20, 0x71, 0x75, 0x69, 0x73, 0x20, 0x74, 0x72, 0x69, 0x73, 0x74, 0x69, 0x71, 0x75, 0x65, 0x20, 0x65, 0x75, 0x2c, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x61, 0x74, 0x20, 0x72, 0x69, 0x73, 0x75, 0x73, 0x2e, 0x20, 0x4d, 0x6f, 0x72, 0x62, 0x69, 0x20, 0x61, 0x20, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x65, 0x72, 0x64, 0x69, 0x65, 0x74, 0x20, 0x6d, 0x69, 0x20, 0x75, 0x6c, 0x6c, 0x61, 0x6d, 0x63, 0x6f, 0x72, 0x70, 0x65, 0x72, 0x20, 0x61, 0x6c, 0x69, 0x71, 0x75, 0x65, 0x74, 0x20, 0x73, 0x75, 0x73, 0x63, 0x69, 0x70, 0x69, 0x74, 0x20, 0x6e, 0x65, 0x63, 0x20, 0x6c, 0x6f, 0x72, 0x65, 0x6d, 0x2e, 0x20, 0x41, 0x65, 0x6e, 0x65, 0x61, 0x6e, 0x20, 0x71, 0x75, 0x69, 0x73, 0x20, 0x6c, 0x65, 0x6f, 0x20, 0x6d, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x2c, 0x20, 0x76, 0x75, 0x6c, 0x70, 0x75, 0x74, 0x61, 0x74, 0x65, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x20, 0x76, 0x61, 0x72, 0x69, 0x75, 0x73, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x69, 0x6d, 0x2e, 0x20, 0x4e, 0x75, 0x6c, 0x6c, 0x61, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x74, 0x75, 0x72, 0x70, 0x69, 0x73, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x6f, 0x2c, 0x20, 0x65, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x75, 0x65, 0x72, 0x65, 0x20, 0x75, 0x72, 0x6e, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20, 0x6e, 0x65, 0x63, 0x2e, 0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x6e, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x6c, 0x6c, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x2e, 0x20, 0x44, 0x6f, 0x6e, 0x65, 0x63, 0x20, 0x74, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x61, 0x75, 0x72, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x67, 0x75, 0x65, 0x20, 0x73, 0x6f, 0x6c, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x75, 0x64, 0x69, 0x6e, 0x2e, 0x20, 0x56, 0x65, 0x73, 0x74, 0x69, 0x62, 0x75, 0x6c, 0x75, 0x6d, 0x20, 0x61, 0x6e, 0x74, 0x65, 0x20, 0x69, 0x70, 0x73, 0x75, 0x6d, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x66, 0x61, 0x75, 0x63, 0x69, 0x62, 0x75, 0x73, 0x20, 0x6f, 0x72, 0x63, 0x69, 0x20, 0x6c, 0x75, 0x63, 0x74, 0x75, 0x73, 0x20, 0x65, 0x74, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x75, 0x65, 0x72, 0x65, 0x20, 0x63, 0x75, 0x62, 0x69, 0x6c, 0x69, 0x61, 0x20, 0x43, 0x75, 0x72, 0x61, 0x65, 0x3b, 0x20, 0x53, 0x75, 0x73, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x73, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x6c, 0x6c, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6d, 0x20, 0x76, 0x65, 0x6c, 0x20, 0x6d, 0x61, 0x73, 0x73, 0x61, 0x20, 0x66, 0x61, 0x75, 0x63, 0x69, 0x62, 0x75, 0x73, 0x2c, 0x20, 0x65, 0x67, 0x65, 0x74, 0x20, 0x6c, 0x61, 0x63, 0x69, 0x6e, 0x69, 0x61, 0x20, 0x6c, 0x61, 0x63, 0x75, 0x73, 0x20, 0x74, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x2e, 0x20, 0x4e, 0x75, 0x6c, 0x6c, 0x61, 0x20, 0x71, 0x75, 0x69, 0x73, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20, 0x70, 0x75, 0x72, 0x75, 0x73, 0x2e, 0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x61, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x72, 0x68, 0x6f, 0x6e, 0x63, 0x75, 0x73, 0x20, 0x6e, 0x69, 0x62, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x75, 0x6d, 0x20, 0x6d, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x2e, 0x20, 0x41, 0x6c, 0x69, 0x71, 0x75, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x69, 0x6d, 0x20, 0x61, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x20, 0x6c, 0x75, 0x63, 0x74, 0x75, 0x73, 0x2c, 0x20, 0x61, 0x20, 0x65, 0x6c, 0x65, 0x69, 0x66, 0x65, 0x6e, 0x64, 0x20, 0x70, 0x75, 0x72, 0x75, 0x73, 0x20, 0x65, 0x67, 0x65, 0x73, 0x74, 0x61, 0x73, 0x2e, 0x20, 0x43, 0x75, 0x72, 0x61, 0x62, 0x69, 0x74, 0x75, 0x72, 0x20, 0x61, 0x74, 0x20, 0x6e, 0x69, 0x62, 0x68, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x2e, 0x20, 0x4e, 0x61, 0x6d, 0x20, 0x62, 0x69, 0x62, 0x65, 0x6e, 0x64, 0x75, 0x6d, 0x2c, 0x20, 0x6e, 0x65, 0x71, 0x75, 0x65, 0x20, 0x61, 0x74, 0x20, 0x61, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x72, 0x69, 0x73, 0x74, 0x69, 0x71, 0x75, 0x65, 0x2c, 0x20, 0x6c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x6c, 0x69, 0x62, 0x65, 0x72, 0x6f, 0x20, 0x61, 0x6c, 0x69, 0x71, 0x75, 0x65, 0x74, 0x20, 0x61, 0x72, 0x63, 0x75, 0x2c, 0x20, 0x6e, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x64, 0x75, 0x6d, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x75, 0x73, 0x20, 0x6c, 0x65, 0x63, 0x74, 0x75, 0x73, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x20, 0x65, 0x72, 0x6f, 0x73, 0x2e, 0x20, 0x43, 0x72, 0x61, 0x73, 0x20, 0x72, 0x68, 0x6f, 0x6e, 0x63, 0x75, 0x73, 0x2c, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x20, 0x61, 0x63, 0x20, 0x6f, 0x72, 0x6e, 0x61, 0x72, 0x65, 0x20, 0x63, 0x75, 0x72, 0x73, 0x75, 0x73, 0x2c, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x6f, 0x20, 0x75, 0x6c, 0x74, 0x72, 0x69, 0x63, 0x65, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x75, 0x73, 0x2c, 0x20, 0x61, 0x74, 0x20, 0x75, 0x6c, 0x6c, 0x61, 0x6d, 0x63, 0x6f, 0x72, 0x70, 0x65, 0x72, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x70, 0x61, 0x74];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_zero() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x80];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_smallint() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x01];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_smallint2() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x10];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_smallint3() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x4f];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_smallint4() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x7f];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_mediumint1() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x81, 0x80];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_mediumint2() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x82, 0x03, 0xe8];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_mediumint3() {
//         let item = RlpItem::Empty;
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x83, 0x01, 0x86, 0xa0];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_mediumint4() {
//         let item = RlpItem::String(vec![0x23, 0x38, 0x33, 0x37, 0x32, 0x39, 0x36, 0x30, 0x39, 0x36, 0x39, 0x39, 0x38, 0x38, 0x34, 0x38, 0x39, 0x36, 0x38, 0x31, 0x35, 0x32, 0x38, 0x36, 0x33, 0x33, 0x31, 0x37, 0x30, 0x31, 0x37, 0x38, 0x30, 0x37, 0x32, 0x32]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x8f, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf2];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_mediumint5() {
//         let item = RlpItem::String(vec![0x23, 0x31, 0x30, 0x35, 0x33, 0x31, 0x35, 0x35, 0x30, 0x35, 0x36, 0x31, 0x38, 0x32, 0x30, 0x36, 0x39, 0x38, 0x37, 0x32, 0x34, 0x36, 0x32, 0x35, 0x33, 0x38, 0x38, 0x30, 0x31, 0x39, 0x30, 0x37, 0x38, 0x33, 0x35, 0x35, 0x38, 0x39, 0x33, 0x35, 0x37, 0x38, 0x35, 0x39, 0x33, 0x33, 0x38, 0x36, 0x32, 0x39, 0x37, 0x34, 0x38, 0x32, 0x32, 0x33, 0x34, 0x37, 0x30, 0x36, 0x38, 0x39, 0x33, 0x35, 0x36, 0x38, 0x31]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0x9c, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00, 0x09, 0x00, 0x0a, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x0e, 0x01];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_emptylist() {
//         let item = RlpItem::List(vec![]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xc0];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_stringlist() {
//         let item = RlpItem::List(vec![RlpItem::String(vec![0x64, 0x6f, 0x67]), RlpItem::String(vec![0x67, 0x6f, 0x64]), RlpItem::String(vec![0x63, 0x61, 0x74])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xcc, 0x83, 0x64, 0x6f, 0x67, 0x83, 0x67, 0x6f, 0x64, 0x83, 0x63, 0x61, 0x74];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_multilist() {
//         let item = RlpItem::List(vec![RlpItem::String(vec![0x7a, 0x77]), RlpItem::List(vec![RlpItem::Empty]), RlpItem::Empty]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xc6, 0x82, 0x7a, 0x77, 0xc1, 0x04, 0x01];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_shortListMax1() {
//         let item = RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76]), RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76]), RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76]), RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xf7, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_longList1() {
//         let item = RlpItem::List(vec![RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xf8, 0x40, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_longList2() {
//         let item = RlpItem::List(vec![RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])]), RlpItem::List(vec![RlpItem::String(vec![0x61, 0x73, 0x64, 0x66]), RlpItem::String(vec![0x71, 0x77, 0x65, 0x72]), RlpItem::String(vec![0x7a, 0x78, 0x63, 0x76])])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xf9, 0x02, 0x00, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76, 0xcf, 0x84, 0x61, 0x73, 0x64, 0x66, 0x84, 0x71, 0x77, 0x65, 0x72, 0x84, 0x7a, 0x78, 0x63, 0x76];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_listsoflists() {
//         let item = RlpItem::List(vec![RlpItem::List(vec![RlpItem::List(vec![]), RlpItem::List(vec![])]), RlpItem::List(vec![])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xc4, 0xc2, 0xc0, 0xc0, 0xc0];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_listsoflists2() {
//         let item = RlpItem::List(vec![RlpItem::List(vec![]), RlpItem::List(vec![RlpItem::List(vec![])]), RlpItem::List(vec![RlpItem::List(vec![]), RlpItem::List(vec![RlpItem::List(vec![])])])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_dictTest1() {
//         let item = RlpItem::List(vec![RlpItem::List(vec![RlpItem::String(vec![0x6b, 0x65, 0x79, 0x31]), RlpItem::String(vec![0x76, 0x61, 0x6c, 0x31])]), RlpItem::List(vec![RlpItem::String(vec![0x6b, 0x65, 0x79, 0x32]), RlpItem::String(vec![0x76, 0x61, 0x6c, 0x32])]), RlpItem::List(vec![RlpItem::String(vec![0x6b, 0x65, 0x79, 0x33]), RlpItem::String(vec![0x76, 0x61, 0x6c, 0x33])]), RlpItem::List(vec![RlpItem::String(vec![0x6b, 0x65, 0x79, 0x34]), RlpItem::String(vec![0x76, 0x61, 0x6c, 0x34])])]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xec, 0xca, 0x84, 0x6b, 0x65, 0x79, 0x31, 0x84, 0x76, 0x61, 0x6c, 0x31, 0xca, 0x84, 0x6b, 0x65, 0x79, 0x32, 0x84, 0x76, 0x61, 0x6c, 0x32, 0xca, 0x84, 0x6b, 0x65, 0x79, 0x33, 0x84, 0x76, 0x61, 0x6c, 0x33, 0xca, 0x84, 0x6b, 0x65, 0x79, 0x34, 0x84, 0x76, 0x61, 0x6c, 0x34];
//         assert_eq!(encoded, expected);
//     }

//     #[test]
//     fn test_bigint() {
//         let item = RlpItem::String(vec![0x23, 0x31, 0x31, 0x35, 0x37, 0x39, 0x32, 0x30, 0x38, 0x39, 0x32, 0x33, 0x37, 0x33, 0x31, 0x36, 0x31, 0x39, 0x35, 0x34, 0x32, 0x33, 0x35, 0x37, 0x30, 0x39, 0x38, 0x35, 0x30, 0x30, 0x38, 0x36, 0x38, 0x37, 0x39, 0x30, 0x37, 0x38, 0x35, 0x33, 0x32, 0x36, 0x39, 0x39, 0x38, 0x34, 0x36, 0x36, 0x35, 0x36, 0x34, 0x30, 0x35, 0x36, 0x34, 0x30, 0x33, 0x39, 0x34, 0x35, 0x37, 0x35, 0x38, 0x34, 0x30, 0x30, 0x37, 0x39, 0x31, 0x33, 0x31, 0x32, 0x39, 0x36, 0x33, 0x39, 0x39, 0x33, 0x36]);
//         let encoded = rlp_encode(&item);
//         let expected = vec![0xa1, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
//         assert_eq!(encoded, expected);
//     }

// }
