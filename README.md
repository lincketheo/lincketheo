# Theo Lincke
__Welcome to my github page__
## Visit my [Website](https://lincketheo.com)
## Or browse my [projects](https://github.com/lincketheo?tab=repositories)
This README.md was generated like this:
```rust
use std;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, ExitStatus, Stdio};

fn main() -> io::Result<()> {
    write_readme()?;
    match git_update() {
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("Error executing git command: {}", e);
            std::process::exit(1);
        }
    }
}

fn git_update() -> io::Result<ExitStatus> {
    command_all_stdout("git", &["add", "."])?;
    command_all_stdout("git", &["commit", "-m", "updated README.md"])?;
    command_all_stdout("git", &["push", "-u", "origin", "main", "--force"]) // You could add more logic to
}

struct MkDown {
    output_file: File,
}

impl MkDown {
    fn new(fname: &str) -> io::Result<Self> {
        Ok(Self {
            output_file: fs::File::create(fname)?,
        })
    }

    fn write_codeln(&mut self, contents: &str, lang: &str) -> io::Result<&mut MkDown> {
        self.output_file
            .write_all(format!("```{}\n{}\n```\n", lang, contents).as_bytes())?;
        Ok(self)
    }

    fn write_headerln(&mut self, levels: usize, contents: &str) -> io::Result<&mut MkDown> {
        self.output_file.write_all(
            format!(
                "{} {}\n",
                (0..levels).map(|_| "#").collect::<String>(),
                contents
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    fn write_bodyln(&mut self, contents: &str) -> io::Result<&mut MkDown> {
        self.output_file
            .write_all(format!("{}\n", contents).as_bytes())?;
        Ok(self)
    }
}

fn write_readme() -> io::Result<()> {
    let contents = fs::read_to_string("./src/main.rs")?;

    let _ = MkDown::new("./README.md")?
        .write_headerln(1, "Theo Lincke")?
        .write_bodyln("__Welcome to my github page__")?
        .write_headerln(2, "Visit my [Website](https://lincketheo.com)")?
        .write_headerln(
            2,
            "Or browse my [projects](https://github.com/lincketheo?tab=repositories)",
        )?
        .write_bodyln("This README.md was generated like this:")?
        .write_codeln(&contents, "rust")?;

    Ok(())
}

trait AllStdout {
    fn reroute_stdout(&mut self) -> &mut Command;
}

impl AllStdout for Command {
    fn reroute_stdout(&mut self) -> &mut Command {
        self.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
    }
}

fn command_all_stdout(command: &str, args: &[&str]) -> io::Result<ExitStatus> {
    Command::new(command).args(args).reroute_stdout().status()
}

```
