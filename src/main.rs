use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_asciidoc_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Starting parser!");

    // Create a path variable from the filename
    let input_filename = Path::new(_filename);

    // Try to open the file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line-by-line
    let reader = BufReader::new(file);

    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of h1 tags

    for line in reader.lines() {
        // Verbose way:
        /*
        let line_contents = match line {
              Ok(contents) => contents,
              Err(e) => panic!("Garbage: {}", e.description())
         };
        */

        // Condensed way: For each line, unwrap it
        let line_contents = line.unwrap();

        let mut first_char: Vec<char>  = line_contents.chars().take(1).collect();

        let mut output_line = String::new();
        let slice = &line_contents.to_string();

        match first_char.pop() {
            Some('=') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n"); // adding \n for instructional clarity
                }
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n"); // close it if we're already open
                }

                _htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&slice[2..]); // Get all but the first two characters
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&slice);
            }
        };

        // At the very end, check if any of the tag bools are still open. If so,
        // close them.

        // if the paragraph tag is open, close it and push a closing HTML tag.
        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        // if the heading tag is open, close it and push a closing HTML tag.
        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        //  avoid pushing blank lines
        if output_line != "<p></p>\n".to_owned() {
            tokens.push(output_line);
        }
    } // end of "for line in reader.lines()" block

    for token in &tokens {
        println!("{}", token)
    }
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
    return String::from("rustadoc <somefile>.adoc");
}

///
/// Prints the long banner.
///
// fn print_long_banner() {
//     print_short_banner();
//     println!(
//         "Written by: {}\nHomepage: {}\nUsage: {}\n",
//         env!("CARGO_PKG_AUTHORS"),
//         env!("CARGO_PKG_HOMEPAGE"),
//         usage()
//     );
// }

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
        _ => {
            println!("[ ERROR ] Invalid invocation");
            usage();
        }
    }
}
