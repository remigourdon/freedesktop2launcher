use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::{io, io::prelude::*};

use freedesktop_desktop_entry::DesktopEntry;

#[derive(Debug)]
#[allow(dead_code)]
struct SimplifiedDesktopEntry {
    path: PathBuf,
    name: String,
    comment: String,
    exec: String,
    terminal: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut total_failed = 0;
    let mut sdes = vec![];
    for line in io::stdin().lock().lines() {
        let line = line?;
        let path = Path::new(&line);
        if let Ok(bytes) = fs::read_to_string(path) {
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
        println!(
            "{}\t{}\t{}\t{}\t{}",
            sde.path.to_string_lossy(),
            sde.name,
            sde.comment,
            sde.exec,
            sde.terminal
        );
    }
    eprintln!("Failed parsing {} desktop files", total_failed);
    Ok(())
}
