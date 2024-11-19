# THE test gen is wrong for now


import json
import re

def sanitize_name(name):
    # Replace non-alphanumeric characters with underscores
    return re.sub(r'\W|^(?=\d)', '_', name)

def parse_hex_string(hex_string):
    # Remove '0x' prefix if present
    hex_string = hex_string.lower()
    if hex_string.startswith('0x'):
        hex_string = hex_string[2:]
    # Split into pairs of hex digits
    bytes_list = []
    for i in range(0, len(hex_string), 2):
        byte = hex_string[i:i+2]
        if byte:
            bytes_list.append(f'0x{byte}')
    return bytes_list

def process_rlp_item(input_value):
    if isinstance(input_value, str):
        # Encode the string into bytes
        input_bytes = input_value.encode('utf-8')
        input_bytes_list = ', '.join([f'0x{byte:02x}' for byte in input_bytes])
        return f'RlpItem::String(vec![{input_bytes_list}])'
    elif isinstance(input_value, list):
        # Process each element recursively
        items = [process_rlp_item(elem) for elem in input_value]
        items_str = ', '.join(items)
        return f'RlpItem::List(vec![{items_str}])'
    else:
        # Handle other types if necessary (e.g., integers)
        return 'RlpItem::Empty'

# Load the invalidRLPTest.json file
with open('RLPTest.json', 'r') as f:
    data = json.load(f)

# Start writing to Eth_test_encode.rs
with open('Eth_test_encode.rs', 'w') as f:
    f.write('use rlp_lib::{RlpItem, rlp_encode};\n\n')
    f.write('#[cfg(test)]\n')
    f.write('mod tests {\n')
    f.write('    use super::*;\n\n')

    # For each test case
    for test_name, test_case in data.items():
        test_func_name = sanitize_name(test_name)

        # Extract input and expected output
        input_value = test_case.get('in', '')
        expected_output = test_case.get('out', '')

        # Process input_value
        item_declaration = process_rlp_item(input_value)

        # Process expected_output
        expected_bytes_list = parse_hex_string(expected_output)
        expected_vec = ', '.join(expected_bytes_list)

        # Generate test function
        f.write(f'    #[test]\n')
        f.write(f'    fn test_{test_func_name}() {{\n')
        f.write(f'        let item = {item_declaration};\n')
        f.write(f'        let encoded = rlp_encode(&item);\n')
        f.write(f'        let expected = vec![{expected_vec}];\n')
        f.write(f'        assert_eq!(encoded, expected);\n')
        f.write(f'    }}\n\n')

    f.write('}\n')
