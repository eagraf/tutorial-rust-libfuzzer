pub fn broken_method(data: &[u8]) {
    if data.len() != 6 {return}
    if data[0] != b'q' {return}
    if data[1] != b'w' {return}
    if data[2] != b'e' {return}
    if data[3] != b'r' {return}
    if data[4] != b't' {return}
    if data[5] != b'y' {return}
    panic!("BOOM")
}
