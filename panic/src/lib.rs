use std::fs::File;
pub fn open_file(s: &str) -> File {
<<<<<<< HEAD
    let f = File::open(s);
    match f {
        Ok(f) => f,
        Err(_) => panic!(),
    }
=======
    File::open(s).unwrap()
>>>>>>> 7429200b1f3aa984f7ea963145f6c57ebdfc133d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
<<<<<<< HEAD
         let filename = "created.txt";
    File::create(filename).unwrap();
        assert_eq!(open_file(filename), { File { fd: 3, path: ".../src/created.txt", read: true, write: false } });
=======
        let filename = "created.txt";
        File::create(filename).unwrap();
        assert_eq!(open_file(filename), {
            File { fd: 3, path: ".../src/created.txt", read: true, write: false }
        });
>>>>>>> 7429200b1f3aa984f7ea963145f6c57ebdfc133d
    }
}
