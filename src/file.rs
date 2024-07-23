use crate::model::greai::ChatLog;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_content_structure_to_csv<P>(
    output_file: P,
    content_arr: &Vec<ChatLog>,
    mut file_header: String,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    if file_header.is_empty() {
        file_header = String::from("timestamp,level,route,answer_mode,search_queries,full_answer,question,question_system_notes,answer");
    }

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file)?;

    let h = String::from(file_header + "\n");
    output.write(h.as_bytes())?;
    for content in content_arr {
        let cntt = String::from(content.to_string() + "\n");
        output.write(cntt.as_bytes())?;
    }
    println!("File successfully created/updated!");
    Ok(())
}
