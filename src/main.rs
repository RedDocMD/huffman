use huffman;
use std::error::Error;

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

    Ok(())
}
