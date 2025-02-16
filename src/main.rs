fn main() {
    let app = match ccwc::Ccwc::new() {
        Ok(ccwc) => ccwc,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    match app.run() {
        Ok(output) => println!("{output}"),
        Err(e) => eprintln!("{e}"),
    };
}
