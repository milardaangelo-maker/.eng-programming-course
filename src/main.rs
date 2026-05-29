use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use serde_json::json;
use eval::eval;
use async_recursion::async_recursion;

pub struct Interpreter {
    variables: HashMap<String, String>,
    client: reqwest::Client,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn run_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            self.execute(line).await?;
        }
        Ok(())
    }

    #[async_recursion]
    pub async fn execute(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let line = line.trim();

        // 1. Repeat [N] times: [COMMAND]
        let re_repeat = Regex::new(r"(?i)^repeat\s+(\d+)\s+times:\s*(.*)$")?;
        if let Some(cap) = re_repeat.captures(line) {
            let count: u32 = cap[1].parse()?;
            let cmd = &cap[2];
            for _ in 0..count {
                self.execute(cmd).await?;
            }
            return Ok(());
        }

        // 2. If file [PATH] exists then: [COMMAND]
        let re_if_file = Regex::new(r"(?i)^if\s+file\s+(\S+)\s+exists\s+then:\s*(.*)$")?;
        if let Some(cap) = re_if_file.captures(line) {
            let path = self.replace_vars(&cap[1]);
            let cmd = &cap[2];
            if std::path::Path::new(&path).exists() {
                self.execute(cmd).await?;
            }
            return Ok(());
        }

        // 3. Ask user "[PROMPT]" as [VAR]
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

        // 4. Calculate [EXPR] as [VAR]
        let re_calc = Regex::new(r"(?i)^calculate\s+(.*)\s+as\s+(\S+)$")?;
        if let Some(cap) = re_calc.captures(line) {
            let expr = self.replace_vars(&cap[1]);
            let var = cap[2].to_string();
            match eval(&expr) {
                Ok(val) => {
                    self.variables.insert(var, val.to_string());
                },
                Err(e) => println!("!! Math Error in '{}': {}", expr, e),
            }
            return Ok(());
        }

        // 5. Send to discord webhook:[URL] [MESSAGE]
        let re_discord = Regex::new(r"(?i)^send to discord webhook:(\S+)\s+(.*)$")?;
        if let Some(cap) = re_discord.captures(line) {
            let url = &cap[1];
            let msg = self.replace_vars(&cap[2]);
            println!(">> Sending to Discord...");
            self.client.post(url)
                .json(&json!({"content": msg}))
                .send()
                .await?;
            return Ok(());
        }

        // 6. Print [MESSAGE]
        let re_print = Regex::new(r"(?i)^print\s+(.*)$")?;
        if let Some(cap) = re_print.captures(line) {
            let msg = self.replace_vars(&cap[1]);
            println!("{}", msg);
            return Ok(());
        }

        // 7. Wait [SECONDS]
        let re_wait = Regex::new(r"(?i)^wait\s+(\d+)$")?;
        if let Some(cap) = re_wait.captures(line) {
            let secs: u64 = cap[1].parse()?;
            println!(">> Waiting {} seconds...", secs);
            sleep(Duration::from_secs(secs)).await;
            return Ok(());
        }

        // 8. Run system command [COMMAND]
        let re_system = Regex::new(r"(?i)^run system command\s+(.*)$")?;
        if let Some(cap) = re_system.captures(line) {
            let cmd_str = self.replace_vars(&cap[1]);
            println!(">> Running system command: {}", cmd_str);
            if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", &cmd_str]).status()?;
            } else {
                Command::new("sh").args(["-c", &cmd_str]).status()?;
            };
            return Ok(());
        }

        // 9. Store [VALUE] as [VAR]
        let re_store = Regex::new(r"(?i)^store\s+(.*)\s+as\s+(\S+)$")?;
        if let Some(cap) = re_store.captures(line) {
            let val = cap[1].trim().trim_matches('"').to_string();
            let var = cap[2].to_string();
            self.variables.insert(var, val);
            return Ok(());
        }

        // 10. Create file [NAME] with content [CONTENT]
        let re_create_file = Regex::new(r"(?i)^create file\s+(\S+)\s+with content\s+(.*)$")?;
        if let Some(cap) = re_create_file.captures(line) {
            let name = &cap[1];
            let content = self.replace_vars(&cap[2]);
            fs::write(name, content)?;
            println!(">> Created file: {}", name);
            return Ok(());
        }

        if !line.is_empty() {
            println!("?? Unknown command: {}", line);
        }
        Ok(())
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
        println!("Usage: english_lang <script.eng>");
        return Ok(());
    }

    let mut interpreter = Interpreter::new();
    interpreter.run_file(&args[1]).await?;

    Ok(())
}
