#[test]
fn test_exercism001() {
    use hello_world as lib;
    assert_eq!(lib::hello(), "Hello, World!");
}
