use colored::Color;
use colored::Colorize;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter, Write};
use std::path::PathBuf;

//function to read start value for collatz sequence
fn read_input() -> String {
    println!(
        "Enter an integer as start value for the Collatz sequence (e.g., 27 or 2^199-1 or 2^199):"
    );
    println!();

    let mut input_value = String::default();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");
    input_value
}

// Function to parse the input value
fn parse_input(input_value: String) -> Option<BigInt> {
    // Use regex to match expressions like "2^199-1" or "2^199"
    let re = Regex::new(r"(\d+)\^(\d+)(?:-(\d+))?").unwrap();
    match re.captures(&input_value) {
        Some(captures) => {
            let base = captures[1].parse::<u32>().unwrap();
            let exponent = captures[2].parse::<u32>().unwrap();
            let subtract = captures
                .get(3)
                .map(|m| m.as_str())
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap();

            // Calculate the parsed value as (base^exponent) - subtract
            Some(BigInt::from(base).pow(exponent) - BigInt::from(subtract))
        }
        None => match input_value.trim().parse::<BigInt>() {
            Ok(value) if value > BigInt::zero() => Some(value), // Return parsed BigInt if valid
            _ => None,                                          // Return None for invalid input
        },
    }
}

//function to define path for output file
fn def_output() -> (PathBuf, File) {
    let output_file_path =
        PathBuf::from("/Users/ralf/Projects/Rust/output_files/collatz_sequence.txt");
    let output_file = File::create(&output_file_path).expect("Failed to create output file");
    (output_file_path, output_file)
}

//function to calculate the collatz sequence and write it to file
fn collatz(mut n: BigInt, output_file: &mut BufWriter<File>) {
    while n != BigInt::one() {
        match n.clone() % BigInt::from(2) {
            x if x == BigInt::zero() => n /= BigInt::from(2),
            _ => n = BigInt::from(3) * n + BigInt::one(),
        }
        writeln!(output_file, "{}", n).expect("Failed to write to file");
    }
}

// Function to read the file line by line, calculate statistics, format and print sequence
fn line_read(
    reader: io::BufReader<File>, // Input: Buffered file reader
    even: &mut i32,              // Output: Count of even numbers
    odd: &mut i32,               // Output: Count of odd numbers
    max_value: &mut BigInt,      // Output: Maximum value encountered
    max_index: &mut usize,       // Output: Line number where maximum value was encountered
    stopping_time: &mut usize,   // Output: Total lines processed (stopping time)
) {
    println!();
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        let num = line
            .trim()
            .parse::<BigInt>()
            .expect("Failed to parse BigInt from file");

        let color = match num.clone() % &BigInt::from(2) {
            x if x == BigInt::zero() => {
                *even += 1;
                Color::White
            }
            _ => {
                *odd += 1;
                Color::Yellow
            }
        };

        if num > *max_value {
            *max_value = num.clone();
            *max_index = line_num + 1; // Increment max_index to account for input_value
        }

        *stopping_time = line_num + 1; // Increment stopping_time to account for input_value

        let formatted_num = num.to_string().color(color);
        print!("{} ", formatted_num);
    }
}
fn main() {
    //inizialize variables
    let mut max_value = BigInt::zero();
    let mut max_index = 0;
    let mut even = 0;
    let mut odd = 0;
    let mut stopping_time = 0;

    //call function to read the start value of the collatz sequence
    let input_value = read_input();

    //call function to parse the input value
    if let Some(parsed_input) = parse_input(input_value.clone()) {
        // call function to define the path for the output file
        let (output_file_path, output_file) = def_output();

        // Open the file in append mode
        let mut output_file = BufWriter::new(output_file);

        //call collatz function
        collatz(parsed_input.clone(), &mut output_file);

        // Close the output_file to release the write lock
        drop(output_file);

        // Reopen the file for reading
        let file = File::open(output_file_path).expect("Failed to open file for reading");
        let reader = std::io::BufReader::new(file);

        //call the function to read the file line by line, calculate statistics, format and print sequence
        line_read(
            reader,
            &mut even,
            &mut odd,
            &mut max_value,
            &mut max_index,
            &mut stopping_time,
        );

        println!();
        println!();
        //print print input value, parsed input value and statistics
        print!("Input: {}", input_value);
        println!("Parsed input: {}", parsed_input);
        println!();
        println!("stopping time: {}", stopping_time);
        println!("even (white): {}", even);
        println!("odd (yellow): {}", odd);
        println!("max pos: {}", max_index);
        println!("max value: {}", max_value);
        println!()
    } else {
        println!("Invalid input. Please enter a valid positive integer or a valid expression like '2^199-1'.");
    }
}
