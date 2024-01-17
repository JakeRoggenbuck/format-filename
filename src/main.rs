use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "filenameformat",
    about = "Format filenames to a command-line friendly format"
)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    // get argument of a filename

    // generate a formatted version of it

    // check if anything was changed

    // check if it's now unique

    //  - if not, add some data to the filename

    // move the file to the new name
}
