use core::str;
use std::{
    env, error, fmt, fs,
    io::{self, Read},
    path::Path,
};

fn count_bytes(input: Vec<u8>) -> u32 {
    input.len() as u32
}

// fn count_lines(input: &str) -> Result<u32, &'static str> {
fn count_lines(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.len() as u32
}

fn count_words(input: &str) -> u32 {
    let words: Vec<&str> = input.split_whitespace().collect();
    words.len() as u32
}

fn count_locale_chars(input: &str) -> u32 {
    input.chars().count() as u32
}

pub struct Ccwc {
    config: Config,
    data: Vec<u8>,
    input_string: String,
}
impl Ccwc {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = match Config::build(env::args().collect()) {
            Ok(config) => config,
            Err(e) => return Err(Box::new(e)),
        };

        let mut data: Vec<u8> = Vec::new();
        if config.file_path != "".to_string() {
            //open file
            let path = Path::new(config.file_path.as_str());
            data = fs::read(path)?;
        } else {
            let mut stdin = io::stdin();
            stdin.read_to_end(&mut data)?;
        }
        let input_string = str::from_utf8(&data)?.to_string();
        Ok(Ccwc {
            config,
            data,
            input_string,
        })
    }

    pub fn run(self) -> Result<String, UnknownOptionError> {
        let output = match self.config.option.as_str() {
            "-c" => format!("{}", count_bytes(self.data)),
            "-l" => {
                let number_of_lines = count_lines(&self.input_string);
                format!("{}", number_of_lines)
            }
            "-w" => {
                let number_of_words = count_words(&self.input_string);
                format!("{}", number_of_words)
            }
            "-m" => {
                let number_of_chars = count_locale_chars(&self.input_string);
                format!("{}", number_of_chars)
            }
            "*" => {
                let lines = count_lines(&self.input_string);
                let words = count_words(&self.input_string);
                format!("{} {} {}", lines, words, count_bytes(self.data))
            }
            _ => return Err(UnknownOptionError),
        };
        Ok(format!("{output} {}", self.config.file_path))
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub option: String,
}
impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, TooManyArgumentsError> {
        match args.len() {
            1 => Ok(Config {
                file_path: "".to_string(),
                option: "*".to_string(),
            }),
            2 => {
                //check if its option or file name
                let arg = args[1].clone();
                if arg.starts_with("-") {
                    return Ok(Config {
                        file_path: "".to_string(),
                        option: arg,
                    });
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
            _ => Err(TooManyArgumentsError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TooManyArgumentsError;

impl fmt::Display for TooManyArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "too many arguments")
    }
}

impl error::Error for TooManyArgumentsError {}

#[derive(Debug, Clone)]
pub struct UnknownOptionError;

impl fmt::Display for UnknownOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknownOption")
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
        let input = "hello \n, how are you\nfine";

        let got = count_lines(input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_count_locale_chars() {
        let want: u32 = 9;
        let input = "hello\n123";

        let got = count_locale_chars(input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_count_words() {
        let want: u32 = 4;
        let input = "hello world\n this\n that";

        let got = count_words(input);
        assert_eq!(want, got);
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
    fn test_input_no_file_name() {
        let input: Vec<String> = vec!["wcc".to_string(), "-c".to_string()];
        let want = Config {
            file_path: "".to_string(),
            option: "-c".to_string(),
        };

        match Config::build(input) {
            Ok(got) => assert_eq!(want, got),
            Err(_) => assert!(false, "should not fail"),
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

    #[test]
    fn test_input_no_args() {
        let input: Vec<String> = vec!["wcc".to_string()];
        let want = Config {
            file_path: "".to_string(),
            option: "*".to_string(),
        };

        match Config::build(input) {
            Ok(got) => assert_eq!(want, got),
            Err(_) => assert!(false, "should not fail"),
        };
    }
}
