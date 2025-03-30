use binary_protocol::Sizeable;

// Define simple structs to test the Sizeable trait
#[derive(Debug)]
struct FixedSizeStruct {
    a: u32, // 4 bytes
    b: u64, // 8 bytes
}

impl FixedSizeStruct {
    fn new(a: u32, b: u64) -> Self {
        Self { a, b }
    }
}

impl Sizeable for FixedSizeStruct {
    fn byte_size(&self) -> usize {
        12 // 4 bytes for a + 8 bytes for b
    }
}

#[derive(Debug)]
struct VariableSizeStruct {
    fixed_size: u32,        // 4 bytes
    variable_data: Vec<u8>, // variable size
}

impl VariableSizeStruct {
    fn new(fixed_size: u32, variable_data: Vec<u8>) -> Self {
        Self {
            fixed_size,
            variable_data,
        }
    }
}

impl Sizeable for VariableSizeStruct {
    fn byte_size(&self) -> usize {
        4 + self.variable_data.len() // 4 bytes for fixed_size + size of variable_data
    }
}

#[derive(Debug)]
struct NestedStruct {
    fixed: FixedSizeStruct,
    variable: VariableSizeStruct,
}

impl NestedStruct {
    fn new(fixed: FixedSizeStruct, variable: VariableSizeStruct) -> Self {
        Self { fixed, variable }
    }
}

impl Sizeable for NestedStruct {
    fn byte_size(&self) -> usize {
        self.fixed.byte_size() + self.variable.byte_size()
    }
}

#[test]
fn test_fixed_size_struct() {
    let fixed = FixedSizeStruct::new(42, 1234567890);

    assert_eq!(fixed.a, 42);
    assert_eq!(fixed.b, 1234567890);

    assert_eq!(fixed.byte_size(), 12);
}

#[test]
fn test_variable_size_struct_empty() {
    let variable = VariableSizeStruct::new(42, vec![]);

    assert_eq!(variable.fixed_size, 42);
    assert_eq!(variable.variable_data.len(), 0);

    assert_eq!(variable.byte_size(), 4); // Just the fixed part
}

#[test]
fn test_variable_size_struct_with_data() {
    let variable = VariableSizeStruct::new(42, vec![1, 2, 3, 4, 5]);
    assert_eq!(variable.byte_size(), 9); // 4 bytes fixed + 5 bytes variable
}

#[test]
fn test_nested_struct() {
    let fixed = FixedSizeStruct::new(42, 1234567890);
    let variable = VariableSizeStruct::new(42, vec![1, 2, 3, 4, 5]);
    let nested = NestedStruct::new(fixed, variable);

    assert_eq!(nested.byte_size(), 21); // 12 bytes fixed + 9 bytes variable
}

#[test]
fn test_collection_of_sizeables() {
    let items = [
        FixedSizeStruct::new(1, 100),
        FixedSizeStruct::new(2, 200),
        FixedSizeStruct::new(3, 300),
    ];

    let total_size = items.iter().fold(0, |acc, item| acc + item.byte_size());
    assert_eq!(total_size, 36); // 3 * 12 bytes
}

#[test]
fn test_large_variable_size() {
    let large_data = vec![0; 1000];
    let variable = VariableSizeStruct::new(42, large_data);
    assert_eq!(variable.byte_size(), 1004); // 4 bytes fixed + 1000 bytes variable
}
