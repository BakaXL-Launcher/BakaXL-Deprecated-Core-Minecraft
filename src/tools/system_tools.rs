pub fn arch() -> String {
    let pointer_size = std::mem::size_of::<usize>();
    if pointer_size == 8 {
        "x64".to_owned()
    } else {
        "x86".to_owned()
    }
}