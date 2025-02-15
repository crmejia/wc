fn main() {
    match ccwc::run() {
        Ok(count) => println!("{count}"),
        Err(e) => eprintln!("{e}"),
    };
}
