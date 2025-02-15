use std::{env, fs, path::Path};

fn count_bytes(input: Vec<u8>) -> u32 {
    input.len() as u32
}

pub fn run() -> Result<u32, &'static str> {
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    //open file
    let path = Path::new(config.file_path.as_str());
    let file = match fs::read(path) {
        Ok(file) => file,
        Err(_) => return Err("couldn't open file"),
    };

    match config.option.as_str() {
        "-c" => Ok(count_bytes(file)),
        _ => Err("unknown option"),
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub option: String,
}
impl Config {
    pub fn build(input: impl IntoIterator<Item = String>) -> Result<Config, &'static str> {
        let mut args = input.into_iter();
        args.next(); //skipprogram name
        let option = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't specify an option"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a file path"),
        };

        Ok(Config { file_path, option })
    }
}

#[cfg(test)]
mod tests {
    use std::process;

    use super::*;

    #[test]
    fn test_count_bytes() {
        let want: u32 = 2;
        let input = vec![0x11, 0x1E];

        let got = count_bytes(input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_input_args() {
        let want = Config {
            file_path: "/hello/file.txt".to_string(),
            option: "-c".to_string(),
        };

        let input: Vec<String> = vec![
            "wcc".to_string(),
            "-c".to_string(),
            "/hello/file.txt".to_string(),
        ];
        let got = Config::build(input).unwrap_or_else(|_| {
            assert!(false, "should not fail");
            process::exit(1);
        });

        assert_eq!(want, got);
    }

    #[test]
    fn test_input_less_args() {
        let input: Vec<String> = vec!["wcc".to_string(), "-c".to_string()];
        let _ = match Config::build(input) {
            Ok(_) => assert!(false, "should fail"),
            Err(_) => (),
        };
    }
}
