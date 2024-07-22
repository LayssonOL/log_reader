mod model;
use model::greai::{ChatLog, ChatMode};
use serde_json;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn build_content_structure(lines: io::Lines<io::BufReader<File>>) -> Vec<ChatLog> {
    let mut structured_data: Vec<ChatLog> = Vec::new();
    let mut chat_log = ChatLog {
        timestamp: String::from(""),
        level: String::from(""),
        question: String::from(""),
        route: String::from(""),
        answer_mode: ChatMode::Normal,
        question_system_notes: String::from(""),
        search_queries: String::from(""),
        full_answer: String::from(""),
        answer: String::from(""),
    };

    // Open the input and output files
    // let mut input = OpenOptions::new().read(true)::open(args[2].as_str())?;
    // let mut output = OpenOptions::new().read(true).write(true).create(true).open(args[3].as_str())?;
    for line in lines.flatten() {
        let line_splits = line.split(" - ").collect::<Vec<&str>>();

        if line.contains("ConversationalRagChain") {
            if line.contains("Question:") {
                chat_log = ChatLog {
                    timestamp: String::from(""),
                    level: String::from(""),
                    question: String::from(""),
                    route: String::from(""),
                    answer_mode: ChatMode::Normal,
                    question_system_notes: String::from(""),
                    search_queries: String::from(""),
                    full_answer: String::from(""),
                    answer: String::from(""),
                };

                chat_log.question = line_splits[1].split(": ").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
            }

            if line.contains("Question with SYS NOTE") {
                chat_log.timestamp = line_splits[0].trim().to_string();
                chat_log.level = line_splits[2].trim().to_string();
                chat_log.question_system_notes =
                    line_splits[4].split("SYS NOTE: ").collect::<Vec<&str>>()[1]
                        .trim()
                        .to_string();
            }

            if line.contains("- search_queries") {
                chat_log.search_queries = line_splits[4].split(": ").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
            }

            if line.contains("- route") {
                chat_log.route = line_splits[4].split(": ").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
            }

            if line.contains("Final answer:") {
                chat_log.answer = line_splits[4].split(": ").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();

                structured_data.push(chat_log.clone());
            }

            if line.contains("Full response:") {
                chat_log.full_answer = line_splits[4].split(": ").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
                chat_log.answer_mode = if line.contains("rag_chain") {
                    ChatMode::RAG
                } else {
                    ChatMode::Normal
                };
            }
        }
    }

    structured_data
}

fn write_content_structure_to_csv<P>(
    output_file: P,
    content_arr: &Vec<ChatLog>,
    mut file_header: String,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    if file_header.is_empty() {
        file_header = String::from("timestamp,level,route,answer_mode,question_system_notes,search_queries,full_answer,question,answer");
    }

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file)?;
    let h = String::from(file_header + "\n");
    output.write(h.as_bytes())?;
    for content in content_arr {
        let cntt = String::from(content.to_string() + "\n");
        output.write(cntt.as_bytes())?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let mut content_arr: Vec<ChatLog> = Vec::new();

    if let Ok(lines) = read_lines(args[2].as_str()) {
        content_arr.extend(build_content_structure(lines));
    }

    let mut rag_count = 0;
    let content_arr_size = content_arr.len();

    // println!("Content Array:");
    for content in &content_arr {
        match content.answer_mode {
            ChatMode::RAG => {
                println!("\n{:?}", content);
                rag_count += 1
            }
            _ => (),
        }
        // println!("\n\n{:?}", content);
    }
    println!("Content Array size: {:?}", content_arr_size);
    println!("Dialogues: {}", content_arr_size);
    println!("RAG dialogues: {}", rag_count);

    if args.len() >= 4 {
        write_content_structure_to_csv(args[3].as_str(), &content_arr, String::from(""))
    } else {
        Ok(())
    }
}
