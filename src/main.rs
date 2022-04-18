use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};

#[derive(Debug)]
#[allow(dead_code)]
struct SimplifiedDesktopEntry {
    path: PathBuf,
    name: String,
    comment: String,
    exec: String,
    terminal: bool,
}

impl fmt::Display for SimplifiedDesktopEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            self.path.to_string_lossy(),
            self.name,
            self.comment,
            self.exec,
            self.terminal
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut total_failed = 0;
    let mut sdes = vec![];
    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                let path = entry.path.to_path_buf();
                let name = entry.name(Some("en")).unwrap().to_string();
                let comment = entry
                    .comment(Some("en"))
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| String::from(""));
                if let Some(actions) = entry.actions() {
                    let actions: Vec<&str> = actions
                        .split(";")
                        .filter(|a| a.chars().count() != 0)
                        .collect();
                    for action in actions {
                        let full_action_name = format!(
                            "{} ({})",
                            name,
                            entry.action_name(action, Some("en")).unwrap()
                        );
                        sdes.push(SimplifiedDesktopEntry {
                            path: path.to_owned(),
                            name: full_action_name,
                            comment: comment.to_owned(),
                            exec: entry.action_exec(action).unwrap().to_string(),
                            terminal: entry.terminal(),
                        });
                    }
                }
                if !entry.no_display() {
                    sdes.push(SimplifiedDesktopEntry {
                        path: path.to_owned(),
                        name,
                        comment: comment.to_owned(),
                        exec: entry.exec().unwrap().to_string(),
                        terminal: entry.terminal(),
                    });
                }
            } else {
                total_failed += 1;
            }
        }
    }
    for sde in sdes {
        println!("{}", sde);
    }
    if total_failed > 0 {
        eprintln!("Failed parsing {} desktop files", total_failed);
    }
    Ok(())
}
