use core::str;
use std::{env, fs, path::Path, str::Chars};

fn count_bytes(input: Vec<u8>) -> u32 {
    input.len() as u32
}

fn count_lines(input: Vec<u8>) -> Result<u32, &'static str> {
    let input = match str::from_utf8(&input) {
        Ok(input) => input,
        Err(_) => return Err("cannot convert input to string"),
    };
    let lines: Vec<&str> = input.lines().collect();
    Ok(lines.len() as u32)
}

fn count_words(input: Vec<u8>) -> Result<u32, &'static str> {
    let input = match str::from_utf8(&input) {
        Ok(input) => input,
        Err(_) => return Err("cannot convert input to string"),
    };
    let words: Vec<&str> = input.split_whitespace().collect();
    Ok(words.len() as u32)
}

fn count_locale_chars(input: Vec<u8>) -> Result<u32, &'static str> {
    let input = match str::from_utf8(&input) {
        Ok(input) => input,
        Err(_) => return Err("cannot convert input to string"),
    };

    Ok(input.chars().count() as u32)
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
        "-l" => match count_lines(file) {
            Ok(number_of_lines) => Ok(number_of_lines),
            Err(e) => Err(e),
        },
        "-w" => match count_words(file) {
            Ok(number_of_words) => Ok(number_of_words),
            Err(e) => Err(e),
        },
        "-m" => match count_locale_chars(file) {
            Ok(number_of_chars) => Ok(number_of_chars),
            Err(e) => Err(e),
        },
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
            Some(arg) => match arg.as_str() {
                "-c" => arg,
                "-l" => arg,
                "-w" => arg,
                "-m" => arg,
                _ => return Err("unknown option"),
            },
            None => return Err("Not enough arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
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
    fn test_count_lines() {
        let want: u32 = 3;
        let input = "hello \n, how are you\nfine".as_bytes().to_vec();

        match count_lines(input) {
            Ok(got) => assert_eq!(want, got),
            Err(_) => assert!(false, "should not fail"),
        }
    }

    #[test]
    fn test_count_locale_chars() {
        let want: u32 = 9;
        let input = "hello\n123".as_bytes().to_vec();

        match count_locale_chars(input) {
            Ok(got) => assert_eq!(want, got),
            Err(_) => assert!(false, "should not fail"),
        }
    }

    #[test]
    fn test_count_words() {
        let want: u32 = 4;
        let input = "hello world\n this\n that".as_bytes().to_vec();

        match count_words(input) {
            Ok(got) => assert_eq!(want, got),
            Err(_) => assert!(false, "should not fail"),
        }
    }
    #[test]
    fn test_input_wrong_option() {
        let input: Vec<String> = vec![
            "wcc".to_string(),
            "-q".to_string(),
            "/hello/file.txt".to_string(),
        ];
        match Config::build(input) {
            Ok(_) => assert!(false, "should  fail"),
            Err(_) => (),
        };
    }
    #[test]
    fn test_input_count_bytes_option() {
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
    fn test_input_count_line_option() {
        let want = Config {
            file_path: "/hello/file.txt".to_string(),
            option: "-l".to_string(),
        };

        let input: Vec<String> = vec![
            "wcc".to_string(),
            "-l".to_string(),
            "/hello/file.txt".to_string(),
        ];
        let got = Config::build(input).unwrap_or_else(|_| {
            assert!(false, "should not fail");
            process::exit(1);
        });

        assert_eq!(want, got);
    }

    #[test]
    fn test_input_count_words_option() {
        let want = Config {
            file_path: "/hello/file.txt".to_string(),
            option: "-w".to_string(),
        };

        let input: Vec<String> = vec![
            "wcc".to_string(),
            "-w".to_string(),
            "/hello/file.txt".to_string(),
        ];

        let got = Config::build(input).unwrap_or_else(|_| {
            assert!(false, "should not fail");
            process::exit(1);
        });

        assert_eq!(want, got);
    }

    #[test]
    fn test_input_count_locale_chars_option() {
        let want = Config {
            file_path: "/hello/file.txt".to_string(),
            option: "-m".to_string(),
        };

        let input: Vec<String> = vec![
            "wcc".to_string(),
            "-m".to_string(),
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
