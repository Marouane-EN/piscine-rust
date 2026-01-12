use std::fs::File;
pub fn open_file(s: &str) -> File {
    let f = File::open(s);
    match f {
        Ok(f) => f,
        Err(_) => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
         let filename = "created.txt";
    File::create(filename).unwrap();
        assert_eq!(open_file(filename), { File { fd: 3, path: ".../src/created.txt", read: true, write: false } });
    }
}
