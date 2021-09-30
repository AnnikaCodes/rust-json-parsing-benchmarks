#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::{fs, path::Path};

    lazy_static! {
        static ref SMALL_JSON: String = fs::read_to_string(Path::new("json/small.json"))
            .expect("Error loading 'json/small.json'; make sure you are operating from the root directory");
        static ref LARGE_JSON: String = fs::read_to_string(Path::new("json/large.json"))
            .expect("Error loading 'json/large.json'; make sure you are operating from the root directory");
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
