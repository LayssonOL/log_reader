pub mod greai {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ChatMode {
        RAG,
        Normal,
    }

    impl ChatMode {
        pub fn from_string(s: &str) -> ChatMode {
            match s {
                "RAG" => ChatMode::RAG,
                "Normal" => ChatMode::Normal,
                _ => ChatMode::Normal,
            }
        }

        pub fn to_string(&self) -> String {
            match self {
                ChatMode::RAG => String::from("RAG"),
                ChatMode::Normal => String::from("Normal"),
            }
        }
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
    }
}
