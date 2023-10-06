use pulldown_cmark::{Event, Parser, Tag};
use std::fs;
use std::process::Command;

fn main() {
    // Read the markdown file
    let markdown = fs::read_to_string("../Command_Line_Interface.md").expect("Unable to read file");

    // Create a new Parser
    let parser = Parser::new(&markdown);

    // Iterate over all events
    let mut in_code_block = false;
    let mut code_block_text = String::new();
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;

                println!("Code block: {}", code_block_text);

                println!("Executing command: {}", code_block_text);

                let status = Command::new("bash")
                    .arg("-c")
                    .arg(&*code_block_text)
                    .status()
                    .expect("Failed to execute command");

                if !status.success() {
                    println!("Command failed: {}", code_block_text);
                }
                code_block_text.clear();
            }
            Event::Text(text) if in_code_block => {
                code_block_text.push_str(&text);
            }
            _ => (),
        }
    }
}
