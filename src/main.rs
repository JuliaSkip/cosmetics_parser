use anyhow::{anyhow, Result};
use cosmetics_parser::*;
use pest::Parser;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<()> {
    //_ = parse_file(String::from("./src/input.txt"), String::from("./src/output.json"));
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_info();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => print_info(),
        "--credits" => println!("Cosmetics Parser by Julia Skip"),
        _ => {
            if args.len() < 3 {
                eprintln!("Error: Missing output file path.");
                print_info();
                return Ok(());
            }
            let input_path = &args[1];
            let output_path = &args[2];
            parse_file(input_path, output_path)?;
        }
    }

    Ok(())
}

fn print_info() {
    println!("Cosmetics Parser:");
    println!("  cargo run <input_file> <output_file>          Parses a cosmetics data file and outputs to JSON format.");
    println!("  cargo run -- --help                           Displays help information.");
    println!("  cargo run -- --credits                        Shows project credits.");
}

fn parse_file(input_path: &str, output_path: &str) -> Result<()> {
    let mut input = String::new();
    File::open(input_path)?.read_to_string(&mut input)?;

    let pairs = Grammar::parse(Rule::products, &input)
        .map_err(|e| anyhow!("Parsing failed: {}", e))?
        .next()
        .ok_or_else(|| anyhow!("No products found in input file"))?;

    let products: Vec<Product> = pairs.into_inner().map(Product::from_pair).collect();

    let json_output = serde_json::to_string_pretty(&products)
        .map_err(|e| anyhow!("Failed to serialize data to JSON: {}", e))?;

    let mut output_file = File::create(output_path)?;
    output_file.write_all(json_output.as_bytes())?;

    println!("Parsed data has been written to {}", output_path);

    Ok(())
}

/*
Requirements:

+ ! Brief Description: Include a brief description of the idea in both the Readme.md and Cargo.toml files. The Readme.md must contain a technical description of the parsing process, detailing what exactly is being parsed and how the results of the parsing will be used.
+ ! Project Name: The project name must reflect the idea and be included in both the Readme.md and Cargo.toml files.
+ ! Grammar Rules: Develop at least 4 grammar rules for the parser.
+ ! Unit Tests: Implement unit tests for the parser; these can be placed in a separate file within the tests directory.
+ ! Test Coverage: Ensure each grammar rule is covered by at least one unit test.
+ ! Core Files: The project must include lib.rs and main.rs files.
+ ! CLI: Create a command-line interface that includes commands to parse a file and to display help and credits information.
+ Error Handling: Implement proper error handling by using anyhow for tests and thiserror for the library.
+ cargo fmt should be used for code formatting.
+ cargo clippy should be used for linting to ensure code quality.
+ A makefile should include commands to launch the program and to run tests, format and clipply before committing as well other useful commands.
+ Completeness: The project must be completed fully.
+ Including a diagram or grammar in the Readme.md can facilitate understanding of the parsing logic.
+ Documentation: For each grammar rule, embed documentation in the code, ensuring that the documentation appears on docs.rs.
Publishing: Release the final version of the package on crates.io.
 */
