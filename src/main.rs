use efcl::{color, Color};
use std::fs;
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "formatfilename",
    about = "Format filenames to a command-line friendly format"
)]
struct Opt {
    /// Input file
    #[structopt()]
    filename: String,
}

fn should_be_replaced(ch: char) -> bool {
    match ch {
        ' ' | '/' | '\\' | '(' | ')' => true,
        _ => false,
    }
}

fn format_filename(filename: String) -> String {
    let mut new_filename = String::new();

    for c in filename.chars() {
        if should_be_replaced(c) {
            new_filename.push('_');
        } else {
            new_filename.push(c);
        }
    }

    return new_filename;
}

fn main() {
    let opt = Opt::from_args();

    // Get argument of a filename
    let filename = opt.filename;

    if !Path::new(&filename).exists() {
        println!("File not found");
        exit(-1);
    }

    // Generate a formatted version of it
    let mut new_filename = format_filename(filename.clone());

    // Check if anything was changed
    let was_changed: bool = new_filename != filename;

    if !was_changed {
        println!("Nothing has changed. Not renaming file.");
        exit(0);
    }

    // Check if it's now unique
    let new_file_path = Path::new(&new_filename);

    let already_exists = new_file_path.exists();

    //  - If not, add some data to the filename
    if already_exists {
        new_filename += "1234";
    }

    println!(
        "Should {} be changed to {}? [y/N]: ",
        color!(Color::GREEN, &format!("'{}'", filename)),
        color!(Color::GREEN, &format!("'{}'", new_filename))
    );

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();

    match line.to_uppercase().as_str() {
        "YES" | "Y" => {
            // Move the file to the new name
            match fs::rename(filename, new_filename.clone()) {
                Ok(_) => {
                    println!(
                        "Wrote new name to file {}!",
                        color!(Color::GREEN, &format!("'{}'", new_filename))
                    );
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        _ => println!("Did not write new name."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_filename_test() {
        assert_eq!(
            format_filename("This is a file".to_string()),
            "This_is_a_file".to_string()
        );

        assert_eq!(
            format_filename("This/test".to_string()),
            "This_test".to_string()
        );
    }
}
