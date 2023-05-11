mod base;
use clap::Parser;

fn main() {
    let args = Params::parse();
    let result = base::sort_csv(
        args.file_path,
        args.reverse,
        args.delimiter,
        args.pos,
        args.output,
    );
    match result {
        Ok(n) => println!("{n}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

#[derive(Parser, Debug)]
struct Params {
    /// Path to file or filename. Ends with .csv
    #[arg(short, long)]
    pub file_path: String,

    /// Sorting in reverse? (from z to a)
    #[arg(short, long)]
    pub reverse: bool,

    /// Position of sorting item (count from 0)
    /// in example `email,name,number` - email is 0, number is 2
    #[arg(short, long, default_value_t = 0)]
    pub pos: usize,

    /// Output filename
    #[arg(short, long, default_value_t = String::from("sorted_output.csv"))]
    pub output: String,

    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}
