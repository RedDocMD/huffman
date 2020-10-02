use huffman;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let data_filename = "data/giant.txt";
    let frequencies = huffman::get_frequencies(data_filename)?;

    let alphabets: Vec<char> = ('a'..(('z' as u8 + 1) as char)).collect();
    // let mut digits: Vec<char> = ('0'..(('9' as u8 + 1) as char)).collect();
    // let mut symbols = Vec::new();
    // symbols.append(&mut alphabets);
    // symbols.append(&mut digits);

    let huffman_code = huffman::generate_huffman_code(&frequencies, &alphabets);
    println!("Huffman encoding:\n{}", huffman_code);
    println!("Creating encoded file...");
    encode_file("data/giant.txt", "data/giant.huff", &huffman_code)?;
    println!("... created encoded file");

    Ok(())
}

fn encode_file(inp_filename: &str, out_filename: &str, code: &huffman::HuffmanCode) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(inp_filename)?;
    let mut output_file = File::create(out_filename)?;
    let mut buf_reader = BufReader::new(input_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines = contents.split('\n');
    for line in lines {
        let line_string = String::from(line);
        let words = line_string.split(' ');
        for word in words {
            output_file.write(&code.encode(word))?;
            output_file.write(b" ")?;
        }
    }
    Ok(())
}