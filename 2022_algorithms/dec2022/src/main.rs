fn main() {
    println!(
        "'\\0'={}, '0'={}, 'A'={}, 'a'={}, 'b'={}",
        '\0' as i8, '0' as i8, 'A' as i8, 'a' as i8, 'b' as i8,
    ); // 0, 48, 65, 97, 98

    assert!('\0' < 'a');
    assert!('\0' < '0');

    assert!("a" < "b"); // ['a'] < ['b']
    assert!("ab" < "ac"); // ['a', 'b'] < ['a', 'c']
    assert!("ab" < "b"); // ['a', 'b'] < ['b', '\0']
    assert!("ab" > ""); // ['a', 'b'] < ['\0', '\0']
    assert!("ab" < "abc"); // ['a', 'b', '\0'] < ['a', 'b', 'c']
}
