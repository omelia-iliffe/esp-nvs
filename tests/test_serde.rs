mod common;

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NewType<T>(T);

    const NAMESPACE: &esp_nvs::Key = &esp_nvs::Key::from_str("test");

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum X {
        A,
        B,
        C,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Test {
        field0: bool,
        field1: i8,
        field2: i16,
        field3: i32,
        enum1: X,
        option1: Option<u8>,
        option2: Option<u16>,
        unit_type: (),
        new_type: NewType<u32>,
        char: char,
        string: String,
        // str: &'a str,
        // bytes: &'a [u8],
    }
    let mut flash = common::Flash::new(2);

    let mut nvs = esp_nvs::Nvs::new(0, flash.len(), &mut flash).unwrap();
    let params = Test {
        field0: false,
        field1: 42,
        field2: 43,
        field3: 44,
        enum1: X::A,
        option1: Some(0),
        option2: None,
        unit_type: (),
        new_type: NewType(1_000_000),
        char: 'X',
        string: "TEST".into(),
        // str: "hello",
        // bytes: &[0; 16],
    };

    nvs.serialize(NAMESPACE, &params).unwrap();

    let p: Test = nvs.deserialize(NAMESPACE).unwrap();

    assert_eq!(p, params);
}
