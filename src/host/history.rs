use super::command::Command;
use std::path::Path;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;

pub trait History {
    fn save(&self) -> anyhow::Result<()>;
    fn read(&self) -> Vec<String>;
    fn up_before(&mut self) -> String;
    fn down_next(&mut self) -> String;
    fn tab_complement(&self, input: &str) -> String;
}

impl History for Command {
    fn save(&self) -> anyhow::Result<()> {
        let path = Path::new("./history.txt");
        let input = self.input.trim().replace("\n", "").replace("\r", "");
        if path.exists() {
            let cmds = self.read();
            if cmds.len() > 0 {
                let i = cmds.len() - 1;
                let last = &cmds[i];
                if last.trim().replace("\n", "").replace("\r", "") == input {
                    return Ok(());
                }
            }
            let mut file = OpenOptions::new()
                .append(true)
                .open(path)
                .unwrap();
            let s = format!("{}\n", input);
            file.write_all(s.as_bytes())?;
            return Ok(());
        }
        let mut output: File = File::create(path)?;
        writeln!(output, "{}", input)?;
        Ok(())
    }
    fn read(&self) -> Vec<String> {
        let path = Path::new("./history.txt");
        if path.exists() {
            if let Ok(contents) = read_to_string(path) {
                let l = contents.split("\n")
                    .map(|e| e.to_string())
                    .filter(|e| e.trim() != "".to_string())
                    .collect();
                return l;
            }
        }
        vec![]
    }
    fn up_before(&mut self) -> String {
        let l = self.read();
        let n = l.len();
        if n == 0 {
            return "".to_string();
        }
        let index = {
            if self.pos == -1 {
                n as i32 - 1
            } else {
                std::cmp::max(self.pos - 1, 0)
            }
        };
        if index == 0 {
            return "".to_string();
        }
        self.pos = index;
        let s = l[index as usize].clone();
        s.replace("\n", "").replace("\r", "")
    }
    fn down_next(&mut self) -> String {
        let l = self.read();
        let n = l.len() as i32;
        if n == 0 || self.pos == -1 || self.pos == n - 1 {
            return "".to_string();
        }
        let index = self.pos + 1;
        self.pos = index;
        let s = l[index as usize].clone();
        s.replace("\n", "").replace("\r", "")
    }
    fn tab_complement(&self, input: &str) -> String {
        let l = self.read();
        let n = l.len();
        if n == 0 {
            return "".to_string();
        }

        let ll: Vec<String> = l.iter()
            .filter(|e| e.starts_with(input))
            .map(|e| e.to_string())
            .collect();
        if ll.len() == 0 {
            return "".to_string();
        }
        let s = ll[0].clone();
        s.replace("\n", "").replace("\r", "")
    }
}