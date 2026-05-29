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
    functions: HashMap<String, Function>,
    client: reqwest::Client,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
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

            // --- Block Detection (to, for, if-do) ---
            
            // 1. Function Definition: to [name] [params] do:
            let re_to = Regex::new(r#"(?i)^to\s+(\S+)\s*(.*)\s+do:$"#)?;
            if let Some(cap) = re_to.captures(line) {
                let name = cap[1].to_string();
                let params: Vec<String> = cap[2].split_whitespace().map(|s| s.to_string()).collect();
                let (body, end_idx) = self.grab_block(lines, i + 1)?;
                self.functions.insert(name, Function { params, body });
                i = end_idx + 1;
                continue;
            }

            // 2. Loop: for each [item] in [list] do:
            let re_for = Regex::new(r#"(?i)^for\s+each\s+(\S+)\s+in\s+(.*)\s+do:$"#)?;
            if let Some(cap) = re_for.captures(line) {
                let item_var = cap[1].to_string();
                let list_raw = self.replace_vars(&cap[2]);
                let (body, end_idx) = self.grab_block(lines, i + 1)?;
                
                let items: Vec<String> = list_raw.split(',').map(|s| s.trim().to_string()).collect();
                for item in items {
                    self.variables.insert(item_var.clone(), item);
                    self.execute_block(&body).await?;
                }
                i = end_idx + 1;
                continue;
            }

            // 3. Logic: if [val] [op] [val] then do:
            let re_if_block = Regex::new(r#"(?i)^if\s+(.*)\s+(is-equal-to|is-greater-than|is-less-than|contains)\s+(.*)\s+then\s+do:$"#)?;
            if let Some(cap) = re_if_block.captures(line) {
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

    #[async_recursion]
    pub async fn execute_line(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let line = line.trim();

        // 1. Single-line IF: if [v] [op] [v] then: [cmd]
        let re_if_single = Regex::new(r#"(?i)^if\s+(.*)\s+(is-equal-to|is-greater-than|is-less-than|contains)\s+(.*)\s+then:\s*(.*)$"#)?;
        if let Some(cap) = re_if_single.captures(line) {
            let left = self.replace_vars(&cap[1]);
            let op = &cap[2];
            let right = self.replace_vars(&cap[3]);
            let cmd = &cap[4];
            if self.check_condition(&left, op, &right) {
                self.execute_line(cmd).await?;
            }
            return Ok(());
        }

        // 2. Ask user "[PROMPT]" as [VAR]
        let re_ask = Regex::new(r#"(?i)^ask\s+user\s+"(.*)"\s+as\s+(\S+)$"#)?;
        if let Some(cap) = re_ask.captures(line) {
            let prompt = self.replace_vars(&cap[1]);
            let var = cap[2].to_string();
            print!("{} ", prompt);
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            self.variables.insert(var, input.trim().to_string());
            return Ok(());
        }

        // 3. Calculate [EXPR] as [VAR]
        let re_calc = Regex::new(r"(?i)^calculate\s+(.*)\s+as\s+(\S+)$")?;
        if let Some(cap) = re_calc.captures(line) {
            let expr = self.replace_vars(&cap[1]);
            let var = cap[2].to_string();
            match eval(&expr) {
                Ok(val) => { self.variables.insert(var, val.to_string()); },
                Err(e) => println!("!! Math Error: {}", e),
            }
            return Ok(());
        }

        // 4. Print [MESSAGE]
        let re_print = Regex::new(r"(?i)^print\s+(.*)$")?;
        if let Some(cap) = re_print.captures(line) {
            println!("{}", self.replace_vars(&cap[1]));
            return Ok(());
        }

        // 5. Store [VALUE] as [VAR]
        let re_store = Regex::new(r"(?i)^store\s+(.*)\s+as\s+(\S+)$")?;
        if let Some(cap) = re_store.captures(line) {
            let val = cap[1].trim().trim_matches('"').to_string();
            let var = cap[2].to_string();
            self.variables.insert(var, val);
            return Ok(());
        }

        // 6. Function Call: [name] [args]
        let words: Vec<&str> = line.split_whitespace().collect();
        if !words.is_empty() {
            let func_name = words[0];
            if let Some(func) = self.functions.get(func_name).cloned() {
                let args = &words[1..];
                for (j, param) in func.params.iter().enumerate() {
                    if let Some(arg) = args.get(j) {
                        self.variables.insert(param.clone(), arg.trim_matches('"').to_string());
                    }
                }
                self.execute_block(&func.body).await?;
                return Ok(());
            }
        }

        // 7. System Bridge
        let re_system = Regex::new(r"(?i)^run system command\s+(.*)$")?;
        if let Some(cap) = re_system.captures(line) {
            let cmd_str = self.replace_vars(&cap[1]);
            if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", &cmd_str]).status()?;
            } else {
                Command::new("sh").args(["-c", &cmd_str]).status()?;
            };
            return Ok(());
        }

        // 8. Discord
        let re_discord = Regex::new(r"(?i)^send to discord webhook:(\S+)\s+(.*)$")?;
        if let Some(cap) = re_discord.captures(line) {
            let url = &cap[1];
            let msg = self.replace_vars(&cap[2]);
            self.client.post(url).json(&json!({"content": msg})).send().await?;
            return Ok(());
        }

        Ok(())
    }

    fn grab_block(&self, lines: &[String], start_idx: usize) -> Result<(Vec<String>, usize), Box<dyn std::error::Error>> {
        let mut block = Vec::new();
        let mut depth = 1;
        let mut current_idx = start_idx;

        while current_idx < lines.len() {
            let line = lines[current_idx].trim();
            if line.ends_with("do:") { depth += 1; }
            if line == "end" {
                depth -= 1;
                if depth == 0 { return Ok((block, current_idx)); }
            }
            block.push(lines[current_idx].clone());
            current_idx += 1;
        }
        Err("Missing 'end' for block".into())
    }

    fn check_condition(&self, left: &str, op: &str, right: &str) -> bool {
        match op {
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
        println!("English-Lang v2.0 - The Easiest Language in the World");
        return Ok(());
    }
    let mut interpreter = Interpreter::new();
    interpreter.run_file(&args[1]).await?;
    Ok(())
}
