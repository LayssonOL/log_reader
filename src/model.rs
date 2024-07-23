pub mod greai {
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{self};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ChatMode {
        RAG,
        Normal,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ChatLog {
        pub timestamp: String,
        pub level: String,
        pub question: String,
        pub route: String,
        pub answer_mode: ChatMode,
        pub question_system_notes: String,
        pub search_queries: String,
        pub full_answer: String,
        pub answer: String,
    }

    impl ChatLog {
        pub fn new() -> ChatLog {
            ChatLog {
                timestamp: String::from(""),
                level: String::from(""),
                question: String::from(""),
                route: String::from(""),
                answer_mode: ChatMode::Normal,
                question_system_notes: String::from(""),
                search_queries: String::from(""),
                full_answer: String::from(""),
                answer: String::from(""),
            }
        }

        pub fn to_string(&self) -> String {
            format!(
                "{},{},{},{:?},{},{},{},{},{}",
                self.timestamp,
                self.level,
                self.route,
                self.answer_mode,
                self.search_queries,
                self.full_answer,
                self.question,
                self.question_system_notes,
                self.answer
            )
        }

        pub fn to_csv_string(&self) -> String {
            format!(
                "{};{};{};{:?};{};{};{};{};{}",
                self.timestamp,
                self.level,
                self.route,
                self.answer_mode,
                self.search_queries,
                self.full_answer,
                self.question,
                self.question_system_notes,
                self.answer
            )
        }

        fn split_line<'a>(&self, line: &'a str) -> Vec<&'a str> {
            line.split(" - ").collect::<Vec<&str>>()
        }

        pub fn store_question_line(&mut self, line: &str) {
            self.question = self.split_line(line)[1].split(": ").collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        }

        pub fn store_timestamp_line(&mut self, line: &str) {
            self.timestamp = self.split_line(line)[0].trim().to_string();
        }

        pub fn store_chat_level_line(&mut self, line: &str) {
            self.timestamp = self.split_line(line)[2].trim().to_string();
        }

        pub fn store_question_system_notes(&mut self, line: &str) {
            self.question_system_notes = self.split_line(line)[4]
                .split("SYS NOTE: ")
                .collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        }

        pub fn store_search_queries(&mut self, line: &str) {
            self.search_queries = self.split_line(line)[4].split(": ").collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        }

        pub fn store_route_name(&mut self, line: &str) {
            self.route = self.split_line(line)[4].split(": ").collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        }

        pub fn store_answer(&mut self, line: &str) {
            self.answer = self.split_line(line)[4].split(": ").collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        }

        pub fn store_answer_mode(&mut self, line: &str) {
            self.answer_mode = if line.contains("rag_chain") {
                ChatMode::RAG
            } else {
                ChatMode::Normal
            };
        }

        pub fn store_full_answer(&mut self, line: &str) {
            self.full_answer = self.split_line(line)[4]
                .split("Full response:")
                .collect::<Vec<&str>>()[1]
                .trim()
                .to_string();
        }
    }

    pub fn build_content_structure(lines: io::Lines<io::BufReader<File>>) -> Vec<ChatLog> {
        let mut structured_data: Vec<ChatLog> = Vec::new();
        let mut chat_log = ChatLog::new();

        for line in lines.flatten() {
            if line.contains("ConversationalRagChain") {
                if line.contains("Question:") {
                    chat_log = ChatLog::new();

                    chat_log.store_question_line(&line);
                }

                if line.contains("Question with SYS NOTE") {
                    chat_log.store_timestamp_line(&line);
                    chat_log.store_chat_level_line(&line);
                    chat_log.store_question_system_notes(&line);
                }

                if line.contains("- search_queries") {
                    chat_log.store_search_queries(&line);
                }

                if line.contains("- route") {
                    chat_log.store_route_name(&line);
                }

                if line.contains("Final answer:") {
                    chat_log.store_answer(&line);

                    structured_data.push(chat_log.clone());
                }

                if line.contains("Full response:") {
                    chat_log.store_full_answer(&line);
                    chat_log.store_answer_mode(&line);
                }
            }
        }

        structured_data
    }
}
