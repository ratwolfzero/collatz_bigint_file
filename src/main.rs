use std::io;
use colored::Colorize;
use colored::Color;
use num_bigint::BigInt;
use num_traits::{Zero, One};
use regex::Regex;
use std::fs::File;
use std::io::{BufWriter, Write, BufRead};
use std::path::PathBuf; // Import PathBuf for working with file paths

fn collatz(mut n: BigInt, output_file: &mut BufWriter<File>) {
    while n != BigInt::one() {
        if n.clone() % &BigInt::from(2) == BigInt::zero() {
            n /= &BigInt::from(2);
        } else {
            n = &BigInt::from(3) * n.clone() + BigInt::one();
        }
        writeln!(output_file, "{}", n).expect("Failed to write to file");
    }
}

fn main() {
    println!("Enter an integer as start value for the Collatz sequence (e.g., 27 or 2^199-1 or 2^199):");
    println!();

    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    // Use regex to match expressions like "2^199-1" or "2^199"
    let re = Regex::new(r"(\d+)\^(\d+)(?:-(\d+))?").unwrap();
    let input_value = if let Some(captures) = re.captures(&input_value) {
        let base = captures[1].parse::<u32>().unwrap();
        let exponent = captures[2].parse::<u32>().unwrap();
        let subtract = captures.get(3).map(|m| m.as_str()).unwrap_or("0").parse::<u32>().unwrap();

        BigInt::from(base).pow(exponent) - BigInt::from(subtract)
    } else {
        match input_value.trim().parse::<BigInt>() {
            Ok(value) if value > BigInt::zero() => value,
            _ => {
                println!("Invalid input. Please enter a valid positive integer > 0");
                return;
            }
        }
    };

    println!();

    let output_file_path = PathBuf::from("/Users/ralf/Projects/Rust//output_files/collatz_sequence.txt");

    let output_file = File::create(&output_file_path).expect("Failed to create output file");

    let mut max_value = BigInt::zero();
    let mut max_index = 0;
    let mut even = 0;
    let mut odd = 0;
    let mut stopping_time = 0;
    
    // Open the file in append mode
    let mut output_file = BufWriter::new(output_file);

    collatz(input_value.clone(), &mut output_file);

    // Close the output_file to release the write lock
    drop(output_file);

    // Reopen the file for reading
    let file = File::open(&output_file_path).expect("Failed to open file for reading");
    let reader = std::io::BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        let num = line.trim().parse::<BigInt>().expect("Failed to parse BigInt from file");
    
        let color = if num.clone() % &BigInt::from(2) == BigInt::zero() {
            Color::White
        } else {
            Color::Yellow
        };

        if num > max_value {
            max_value = num.clone();
            max_index = line_num + 1; // Increment max_index to account for input_value
        }

        if num.clone() % &BigInt::from(2) == BigInt::zero() {
            even += 1;
        } else {
            odd += 1;
        }

        stopping_time = line_num +1 ; // Increment stopping_time to account for input_value

        let formatted_num=num.to_string().color(color);
        
        print!("{} ", formatted_num);
    }   

    println!();
    println!();
    println!("stopping time: {}", stopping_time);
    println!();
    println!("max value: {}", max_value);
    println!();
    println!("max pos: {}", max_index);
    println!("even: {}", even);
    println!("odd: {}", odd);
    println!();
}


