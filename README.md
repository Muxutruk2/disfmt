# disfmt

`disfmt` is a simple CLI tool for disformatting Rust code by aligning `{`, `}`, and `;` symbols vertically. It processes your code to disimprove readability by ensuring these symbols are consistently aligned across lines.

## Installation

You can easily install `disfmt` using Cargo, the Rust package manager.

```bash
cargo install disfmt
```

This will install the `disfmt` command globally, allowing you to use it from anywhere in your terminal.

## Usage

Once installed, you can use the `disfmt` tool via the command line to process Rust code. Here's the basic usage:

### Aligning code from a file

```bash
disfmt input.rs -o output.rs
```

This will read the Rust code from `input.rs`, align the `{`, `}`, and `;` symbols, and write the result to `output.rs`.

### Aligning code from stdin

You can also provide input through stdin and output the results to stdout:

```bash
cat input.rs | disfmt > output.rs
```

Or you can simply use:

```bash
disfmt input.rs
```

This will output the formatted code to the terminal (stdout).

### Options

- `-o <output>`: Specify the output file to save the formatted code. If not provided, the result will write to `STDOUT`.
- `input`: The input file to process. If not provided, the tool will read from `STDIN`.

## Example

``` rust
fn parse_matches(matches: &clap::ArgMatches, cmd: &mut clap::Command, conn: &rusqlite::Connection)  {
    let subcommand = matches.subcommand().map_or_else(
        ||                                                                                          {
            cmd.print_help().ok()                                                                   ;
            std::process::exit(0)                                                                   ;
        }                                                                                           ,
        |s| s                                                                                       ,
    )                                                                                               ;

match subcommand.0                                                                                  {
    "add" => parse_add(matches, conn)                                                               ,
    "list" => list_tasks_in_db(conn)                                                                ,
    _ =>                                                                                            {
        eprintln!("Invalid subcommand")                                                             ;
        cmd.print_help().ok()                                                                       ;
        std::process::exit(1)                                                                       ;
```

## Development

If you'd like to contribute to `disfmt`, feel free to fork the repository and submit pull requests! Please make sure to run the tests before submitting changes.

### Building the project

To build the project locally without installing, you can run:

```bash
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- GitHub repository: [https://github.com/Muxutruk2/disfmt](https://github.com/Muxutruk2/disfmt)
- Documentation: [https://docs.rs/disfmt](https://docs.rs/disfmt)
