mod file;
mod model;

use file::{read_lines, write_content_structure_to_csv};
use model::greai::{build_content_structure, ChatLog, ChatMode};

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let input_file: &str;
    let output_file: &str;
    if args.len() < 4 {
        println!("Example: ./greai_log_reader input.txt output.txt");
        input_file = args[1].as_str();
        output_file = args[2].as_str();
        println!("Input file: {}", input_file);
        println!("Output file: {}", output_file);
    } else {
        input_file = args[2].as_str();
        output_file = args[3].as_str();
        println!("Input file: {}", input_file);
        println!("Output file: {}", output_file);
    }

    let mut content_arr: Vec<ChatLog> = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        content_arr.extend(build_content_structure(lines));
    }

    let mut rag_count = 0;
    let content_arr_size = content_arr.len();

    // println!("Content Array:");
    for content in &content_arr {
        match content.answer_mode {
            ChatMode::RAG => rag_count += 1,
            _ => (),
        }
    }
    println!("Total interactions: {:?}", content_arr_size);
    println!("Chat interactions: {}", (content_arr_size - rag_count));
    println!("RAG interactions: {}", rag_count);

    write_content_structure_to_csv(output_file, &content_arr, String::from(""))?;
    Ok(())
}
