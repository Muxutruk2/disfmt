use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};

/// A CLI tool to un-format Rust code by aligning `{`, `}`, and `;` vertically.
///
/// This tool processes Rust code to ensure that the curly braces `{`, `}`, and the semicolon `;`
/// symbols are aligned vertically, unimproving the readability of code that might have consistent formatting.
///
/// # Arguments
/// - `input`: The input file to process (defaults to `STDIN` if not provided).
/// - `output`: The output file where the result will be written (defaults to `STDOUT` if not provided).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file to read the Rust code from. If not provided, will read from STDIN.
    ///
    /// # Example
    /// `input.rs` will be processed.
    input: Option<String>,

    /// Output file to write the formatted code to. If not provided, the output will be written to STDOUT.
    ///
    /// # Example
    /// `output.rs` will be the file that receives the result.
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> io::Result<()> {
    // Parse the command-line arguments using Clap.
    let args = Cli::parse();

    // Read input from a file or from standard input.
    let input = if let Some(input_file) = args.input {
        // If an input file is provided, read its content.
        fs::read_to_string(input_file)?
    } else {
        // If no input file is provided, read from standard input (STDIN).
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Process the input code and transform its formatting.
    let output = transform_code(&input);

    // Write the transformed code to either a file or STDOUT.
    if let Some(output_file) = args.output {
        // If an output file is provided, write the result to it.
        fs::write(output_file, output)?;
    } else {
        // If no output file is provided, print the result to STDOUT.
        io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}

/// Transforms the input code by aligning the `{`, `}`, and `;` symbols vertically.
///
/// This function processes each line of code, detecting lines that end with `{`, `}`, or `;`,
/// and aligning those symbols to make the unformatting consistent.
///
/// # Arguments
/// - `input`: A string slice containing the Rust code to be processed.
///
/// # Returns
/// A `String` representing the transformed Rust code, with symbols aligned.
///
/// # Example
/// ```
/// let input = "fn main() { return 0; }";
/// let output = transform_code(input);
/// println!("{}", output);
/// ```
/// This would produce code with `{`, `}`, and `;` aligned vertically.
///
fn transform_code(input: &str) -> String {
    // Split the input code into individual lines.
    let lines: Vec<&str> = input.lines().collect();

    // Symbols to align in the code.
    let symbols = ['{', '}', ';', ','];

    // Find the maximum length of a line to calculate the amount of padding needed.
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Process each line and align symbols as needed.
    lines
        .into_iter()
        .map(|line| {
            if line.trim_end().ends_with(symbols) {
                // If the line ends with a symbol, find its position and calculate padding.
                line.rfind(|c| symbols.contains(&c)).map_or_else(|| line.to_string(), |pos| {
                    let padding = max_len - pos;
                    // Add padding spaces before the symbol to align it.
                    format!("{}{}{}", &line[..pos], " ".repeat(padding), &line[pos..])
                })
            } else {
                // If the line does not end with a symbol, return it unchanged.
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
