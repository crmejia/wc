use core::str;
use std::{env, fs, path::Path};

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
pub fn run() -> Result<String, &'static str> {
    //this is setup, not part of the run
    let config = match Config::build(env::args().collect()) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    //open file
    //todo this could actually be injected as a Vec<u8> so it can be tested
    let path = Path::new(config.file_path.as_str());
    let file = match fs::read(path) {
        Ok(file) => file,
        Err(_) => return Err("couldn't open file"),
    };

    match config.option.as_str() {
        "-c" => Ok(format!("{}", count_bytes(file))),
        "-l" => match count_lines(file) {
            Ok(number_of_lines) => Ok(format!("{}", number_of_lines)),
            Err(e) => Err(e),
        },
        "-w" => match count_words(file) {
            Ok(number_of_words) => Ok(format!("{}", number_of_words)),
            Err(e) => Err(e),
        },
        "-m" => match count_locale_chars(file) {
            Ok(number_of_chars) => Ok(format!("{}", number_of_chars)),
            Err(e) => Err(e),
        },
        "*" => {
            //ineffiecient cloning thanks compiler
            let lines = count_lines(file.clone()).unwrap();
            let words = count_words(file.clone()).unwrap();
            let filename = path.file_name().unwrap().to_str().unwrap();
            Ok(format!(
                "{} {} {} {}",
                lines,
                words,
                count_bytes(file),
                filename
            ))
        }
        _ => Err("unknown option"),
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub option: String,
}
impl Config {
    // pub fn build(input: impl IntoIterator<Item = String>) -> Result<Config, &'static str> {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        // let mut args = input.into_iter();
        // args.
        // args.next(); //skipprogram name

        // let option = match args.next() {
        //     Some(arg) => match arg.as_str() {
        //         "-c" => arg,
        //         "-l" => arg,
        //         "-w" => arg,
        //         "-m" => arg,
        //         _ => return Err("unknown option"),
        //     },
        //     None => return Err("Not enough arguments"),
        // };

        // let file_path = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("not enough arguments"),
        // };

        // Ok(Config { file_path, option })
        match args.len() {
            1 => todo!(), //check stdin
            2 => {
                //check if its option or file name
                let arg = args[1].clone();
                if arg.starts_with("-") {
                    todo!() //use option + stdin
                }

                Ok(Config {
                    file_path: arg,
                    option: "*".to_string(),
                })
            }
            3 => Ok(Config {
                file_path: args[2].clone(),
                option: args[1].clone(),
            }),
            _ => return Err("too many arguments"),
        }
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

    //input Config build tests
    #[test]
    fn test_input_wrong_option() {
        let input: Vec<String> = vec![
            "wcc".to_string(),
            "-q".to_string(),
            "/hello/file.txt".to_string(),
        ];
        match Config::build(input) {
            Ok(_) => (),
            Err(_) => assert!(false, "should not fail"),
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

    #[test]
    fn test_input_no_option_filename() {
        let want = Config {
            file_path: "/hello/file.txt".to_string(),
            option: "*".to_string(),
        };

        let input: Vec<String> = vec!["ccwc".to_string(), "/hello/file.txt".to_string()];

        match Config::build(input) {
            Ok(got) => assert_eq!(want, got),
            Err(_) => assert!(false, "should not fail"),
        }
    }
}
