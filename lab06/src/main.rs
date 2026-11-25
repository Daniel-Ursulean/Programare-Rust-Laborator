use rusqlite::Connection;
use std::fs::{self};

use crate::Errors::BkInsertFail;

enum Errors<'a> {
    ReadError,
    UnknownCommand(&'a str, &'a mut Terminal),
    BkFormat,
    BkDbFail,
    BkIncompleteArgs,
    BkInsertFail(&'a str),
}

fn error_handler(er: Errors) {
    match er {
        Errors::ReadError => println!("Error: Was not able to read the file :(("),
        Errors::UnknownCommand(cmd, terminal) => {
            println!("Error: Unknown command");
            suggest_command(cmd, terminal);
        }
        Errors::BkFormat => {
            println!("Error: 'bk' should be used like -> bk add|search <name> [url]")
        }
        Errors::BkDbFail => println!("Error: Something went wrong with the database"),
        Errors::BkIncompleteArgs => println!("Error: Not enough arguments for 'bk'"),
        Errors::BkInsertFail(name) => println!("Error: Failed to insert {} into bookmarks", name),
    }
}

trait Commands {
    fn get_name(&self) -> String;
    fn exec(&mut self, arg: &[&str]);
}

struct PingCommand {}
impl Commands for PingCommand {
    fn get_name(&self) -> String {
        String::from("ping")
    }
    fn exec(&mut self, _arg: &[&str]) {
        println!("pong!")
    }
}

struct CountCommand {}
impl Commands for CountCommand {
    fn get_name(&self) -> String {
        String::from("count")
    }
    fn exec(&mut self, arg: &[&str]) {
        println!("counted {} args", arg.len())
    }
}

struct TimesCommand {
    times_used: usize,
}
impl Commands for TimesCommand {
    fn get_name(&self) -> String {
        String::from("times")
    }
    fn exec(&mut self, _arg: &[&str]) {
        self.times_used += 1;
        if self.times_used == 1 {
            println!("'times' used one time")
        } else {
            println!("'times' used {} times", self.times_used)
        }
    }
}

struct EchoCommand {}
impl Commands for EchoCommand {
    fn get_name(&self) -> String {
        String::from("echo")
    }
    fn exec(&mut self, args: &[&str]) {
        println!("{}", args.join(" "));
    }
}

struct BkCommand {}
impl Commands for BkCommand {
    fn get_name(&self) -> String {
        String::from("bk")
    }
    fn exec(&mut self, arg: &[&str]) {
        let Ok(conn) = Connection::open("bookmarks.db") else {
            error_handler(Errors::BkDbFail);
            return;
        };
        let create = r"
        create table if not exists bookmarks (
            name text    not null,
            url  text not null
        );
        ";
        if conn.execute(create, ()).is_err() {
            error_handler(Errors::BkDbFail);
            return;
        }

        let mut iter = arg.iter().copied();
        match iter.next() {
            Some("add") => {
                let (Some(name), Some(url)) = (iter.next(), iter.next()) else {
                    error_handler(Errors::BkIncompleteArgs);
                    return;
                };
                match conn.execute(
                    "insert into bookmarks (name, url) values (?1, ?2);",
                    (name, url),
                ) {
                    Ok(_) => {
                        println!("Aded {} into bookmarks", name)
                    }
                    Err(_) => {
                        error_handler(BkInsertFail(name));
                    }
                }
            }
            Some("search") => {
                let Some(to_search) = iter.next() else {
                    error_handler(Errors::BkIncompleteArgs);
                    return;
                };

                let Ok(mut stmt) =
                    conn.prepare("select name, url from bookmarks where name like ?1")
                else {
                    error_handler(Errors::BkDbFail);
                    return;
                };

                let pattern = format!("%{}%", to_search);
                let Ok(rows) = stmt.query_map((pattern,), |row| {
                    let name: String = row.get("name")?;
                    let url: String = row.get("url")?;
                    Ok((name, url))
                }) else {
                    error_handler(Errors::BkDbFail);
                    return;
                };

                for (name, url) in rows.flatten() {
                    println!("{} -> {}", name, url);
                }
            }
            _ => {
                error_handler(Errors::BkFormat);
            }
        }
    }
}
struct Terminal {
    commands: Vec<Box<dyn Commands>>,
    file_path: String,
}
impl Terminal {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
            file_path: String::from("input.txt"),
        }
    }
    fn register(&mut self, command: Box<dyn Commands>) {
        self.commands.push(command);
    }
    fn run(&mut self) {
        if let Ok(s) = fs::read_to_string(&self.file_path) {
            for line in s.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let cmd_name = parts[0];
                let args = &parts[1..];

                if cmd_name == "stop" {
                    break;
                }
                let mut found = false;
                for comm in &mut self.commands {
                    if cmd_name == comm.get_name() {
                        comm.exec(args);
                        found = true;
                        break;
                    }
                }
                if !found {
                    error_handler(Errors::UnknownCommand(cmd_name, self));
                }
            }
        } else {
            error_handler(Errors::ReadError);
        }
    }
}

fn count_differences(word1: &str, word2: &str) -> usize {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();

    if chars1.len() == chars2.len() {
        let mut diff = 0;
        for i in 0..chars1.len() {
            if chars1[i] != chars2[i] {
                diff += 1;
            }
        }
        return diff;
    }

    let (longer, shorter) = if chars1.len() > chars2.len() {
        (&chars1, &chars2)
    } else {
        (&chars2, &chars1)
    };

    if longer.len() - shorter.len() == 1 {
        for skip in 0..longer.len() {
            let mut without_one: Vec<char> = Vec::new();
            for (i, _) in longer.iter().enumerate() {
                if i != skip {
                    without_one.push(longer[i]);
                }
            }

            if without_one == *shorter {
                return 1;
            }
        }
    }
    100
}
fn suggest_command(cmd: &str, terminal: &mut Terminal) {
    let written_comm = cmd.to_lowercase();
    let mut min_diff = 3;
    let mut best_match: Option<String> = None;
    for comm in &terminal.commands {
        let comm_name = comm.get_name();
        if comm_name == written_comm {
            best_match = Some(comm_name);
            break;
        }
        let diff = count_differences(&written_comm, &comm_name);
        if diff < min_diff {
            min_diff = diff;
            best_match = Some(comm_name.clone());
        }
    }
    if let Some(misscomm) = best_match {
        println!("Did you mean {}?", misscomm)
    }
}
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { times_used: 0 }));
    terminal.register(Box::new(EchoCommand {}));
    terminal.register(Box::new(BkCommand {}));
    terminal.run();
}
