fn parse_asciidoc_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);
}

///
/// Prints the short banner.
///
fn print_short_banner() {
    println!("{}", get_title());
}

///
/// Returns the usage string.
///
fn usage() -> String {
    return String::from("rustadoc <somefile>.md");
}

///
/// Prints the long banner.
///
fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: {}\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE"),
        usage()
    );
}

///
/// Returns the title.
///
fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_asciidoc_file(&args[1]),
        _ => println!("{}", usage())
    }
    print_long_banner()
}
