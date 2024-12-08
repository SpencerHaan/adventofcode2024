use std::fs;

pub fn lines<F>(path: &str, mut f: F) where F: FnMut(&str) {
    match fs::read_to_string(path) {
        Ok(data) => {
            for line in data.split_terminator("\n") {
                f(line);
            }
        },
        Err(e) => panic!("{e:?}")
    }
}

pub fn lines_indexed<F>(path: &str, mut f: F) where F: FnMut(usize, &str) {
    match fs::read_to_string(path) {
        Ok(data) => {
            for (i, line) in data.split_terminator("\n").enumerate() {
                f(i, line);
            }
        },
        Err(e) => panic!("{e:?}")
    }
}
