use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use serde_json::json;
use eval::eval;
use async_recursion::async_recursion;

#[derive(Clone)]
pub struct Function {
    params: Vec<String>,
    body: Vec<String>,
}

pub struct Interpreter {
    variables: HashMap<String, String>,
    lists: HashMap<String, Vec<String>>,
    functions: HashMap<String, Function>,
    client: reqwest::Client,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            lists: HashMap::new(),
            functions: HashMap::new(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn run_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        self.execute_block(&lines).await?;
        Ok(())
    }

    #[async_recursion]
    pub async fn execute_block(&mut self, lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with("#") {
                i += 1;
                continue;
            }

            // 1. Function Definition: to [name] do:
            let re_to = Regex::new(r#"(?i)^to\s+(\S+)\s+do:$"#)?;
            if let Some(cap) = re_to.captures(line) {
                let name = cap[1].to_string();
                let (body, end_idx) = self.grab_block(lines, i + 1)?;
                self.functions.insert(name, Function { params: vec![], body });
                i = end_idx + 1;
                continue;
            }

            // 2. Loop: for each [item] in [list] do:
            let re_for = Regex::new(r#"(?i)^for\s+each\s+(\S+)\s+in\s+(\S+)\s+do:$"#)?;
            if let Some(cap) = re_for.captures(line) {
                let item_var = cap[1].to_string();
                let list_name = cap[2].to_string();
                let (body, end_idx) = self.grab_block(lines, i + 1)?;
                
                if let Some(items) = self.lists.get(&list_name).cloned() {
                    for item in items {
                        self.variables.insert(item_var.clone(), item);
                        self.execute_block(&body).await?;
                    }
                }
                i = end_idx + 1;
                continue;
            }

            // 3. Simple Repeat: repeat [N] times do:
            let re_repeat = Regex::new(r#"(?i)^repeat\s+(\d+)\s+times\s+do:$"#)?;
            if let Some(cap) = re_repeat.captures(line) {
                let count: u32 = cap[1].parse()?;
                let (body, end_idx) = self.grab_block(lines, i + 1)?;
                for _ in 0..count {
                    self.execute_block(&body).await?;
                }
                i = end_idx + 1;
                continue;
            }

            // 4. Logic: if [val] [op] [val] do:
            let re_if = Regex::new(r#"(?i)^if\s+(.*)\s+(is-equal-to|is-greater-than|is-less-than|contains)\s+(.*)\s+do:$"#)?;
            if let Some(cap) = re_if.captures(line) {
                let left = self.replace_vars(&cap[1]);
                let op = &cap[2];
                let right = self.replace_vars(&cap[3]);
                let (body, end_idx) = self.grab_block(lines, i + 1)?;
                
                if self.check_condition(&left, op, &right) {
                    self.execute_block(&body).await?;
                }
                i = end_idx + 1;
                continue;
            }

            // --- Single Line Commands ---
            self.execute_line(line).await?;
            i += 1;
        }
        Ok(())
    }

    pub async fn execute_line(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let line = line.trim();

        // 1. Say/Print
        let re_say = Regex::new(r#"(?i)^(?:say|print)\s+(.*)$"#)?;
        if let Some(cap) = re_say.captures(line) {
            println!("{}", self.replace_vars(&cap[1]));
            return Ok(());
        }

        // 2. Store Value
        let re_store = Regex::new(r#"(?i)^store\s+(.*)\s+as\s+(\S+)$"#)?;
        if let Some(cap) = re_store.captures(line) {
            let val = cap[1].trim().trim_matches('"').to_string();
            let var = cap[2].to_string();
            self.variables.insert(var, val);
            return Ok(());
        }

        // 3. Store List
        let re_list = Regex::new(r#"(?i)^create\s+list\s+"(.*)"\s+as\s+(\S+)$"#)?;
        if let Some(cap) = re_list.captures(line) {
            let items: Vec<String> = cap[1].split(',').map(|s| s.trim().to_string()).collect();
            let var = cap[2].to_string();
            self.lists.insert(var, items);
            return Ok(());
        }

        // 4. Calculate
        let re_calc = Regex::new(r#"(?i)^calc(?:ulate)?\s+(.*)\s+as\s+(\S+)$"#)?;
        if let Some(cap) = re_calc.captures(line) {
            let expr = self.replace_vars(&cap[1]);
            let var = cap[2].to_string();
            if let Ok(val) = eval(&expr) {
                self.variables.insert(var, val.to_string());
            }
            return Ok(());
        }

        // 5. Ask
        let re_ask = Regex::new(r#"(?i)^ask\s+"(.*)"\s+as\s+(\S+)$"#)?;
        if let Some(cap) = re_ask.captures(line) {
            print!("{} ", self.replace_vars(&cap[1]));
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            self.variables.insert(cap[2].to_string(), input.trim().to_string());
            return Ok(());
        }

        // 6. Function Call
        if let Some(func) = self.functions.get(line).cloned() {
            self.execute_block(&func.body).await?;
            return Ok(());
        }

        // 7. Wait
        let re_wait = Regex::new(r#"(?i)^wait\s+(\d+)$"#)?;
        if let Some(cap) = re_wait.captures(line) {
            let secs: u64 = cap[1].parse()?;
            tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
            return Ok(());
        }

        // 8. System/Discord
        if line.to_lowercase().starts_with("run system command") {
            let cmd_str = self.replace_vars(&line[19..]);
            if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", &cmd_str]).status()?;
            } else {
                Command::new("sh").args(["-c", &cmd_str]).status()?;
            };
        } else if line.to_lowercase().starts_with("send to discord webhook:") {
            let parts: Vec<&str> = line[25..].splitn(2, ' ').collect();
            if parts.len() == 2 {
                let url = parts[0];
                let msg = self.replace_vars(parts[1]);
                self.client.post(url).json(&json!({"content": msg})).send().await?;
            }
        }

        Ok(())
    }

    fn grab_block(&self, lines: &[String], start_idx: usize) -> Result<(Vec<String>, usize), Box<dyn std::error::Error>> {
        let mut block = Vec::new();
        let mut depth = 1;
        let mut current_idx = start_idx;

        while current_idx < lines.len() {
            let line = lines[current_idx].trim();
            if line.to_lowercase().ends_with("do:") { depth += 1; }
            if line.to_lowercase() == "end" {
                depth -= 1;
                if depth == 0 { return Ok((block, current_idx)); }
            }
            block.push(lines[current_idx].clone());
            current_idx += 1;
        }
        Err("Error: Each 'do:' must have a matching 'end'".into())
    }

    fn check_condition(&self, left: &str, op: &str, right: &str) -> bool {
        match op.to_lowercase().as_str() {
            "is-equal-to" => left == right,
            "is-greater-than" => left.parse::<f64>().unwrap_or(0.0) > right.parse::<f64>().unwrap_or(0.0),
            "is-less-than" => left.parse::<f64>().unwrap_or(0.0) < right.parse::<f64>().unwrap_or(0.0),
            "contains" => left.contains(right),
            _ => false,
        }
    }

    fn replace_vars(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (name, val) in &self.variables {
            result = result.replace(&format!("{{{}}}", name), val);
        }
        result
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("English-Lang v2.5 Stable - The Easiest Language in the World");
        return Ok(());
    }
    let mut interpreter = Interpreter::new();
    interpreter.run_file(&args[1]).await?;
    Ok(())
}
