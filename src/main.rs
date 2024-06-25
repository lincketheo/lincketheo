use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, ExitStatus, Stdio};
use std;

fn main() -> io::Result<()> {
    write_readme()?;
    match git_commands() {
        Ok(status) => {
            std::process::exit(status.code().unwrap_or(1))
        },
        Err(e) => {
            eprintln!("Error executing git command: {}", e);
            std::process::exit(1);
        }
    }
}

fn git_commands() -> io::Result<ExitStatus> {
    command("git", &["add", "."])?;
    command("git", &["commit", "-m", "updated README.md"])?;
    command("git", &["push", "-u", "origin", "master"])
}

fn write_readme() -> io::Result<()> {
    let contents = fs::read_to_string("./src/main.rs")?;
    let mut output_file = fs::File::create("./README.md")?;

    write_headerln(1, &mut output_file, "Theo Lincke")?;
    write_bodyln(&mut output_file, "__Welcome to my github page__")?;
    write_headerln(
        2,
        &mut output_file,
        "Visit my [Website](https://lincketheo.com)",
    )?;
    write_headerln(
        2,
        &mut output_file,
        "Or browse my [projects](https://github.com/lincketheo?tab=repositories)",
    )?;
    write_bodyln(&mut output_file, "This script was generated like this:")?;
    write_codeln(&mut output_file, &contents, "rust")?;

    Ok(())
}

fn write_codeln(output_file: &mut File, contents: &str, lang: &str) -> io::Result<()> {
    output_file.write_all(format!("```{}\n{}\n```\n", lang, contents).as_bytes())
}

fn write_headerln(levels: usize, output_file: &mut File, contents: &str) -> io::Result<()> {
    output_file.write_all(
        format!(
            "{} {}\n",
            (0..levels).map(|_| "#").collect::<String>(),
            contents
        )
        .as_bytes(),
    )
}

fn write_bodyln(output_file: &mut File, contents: &str) -> io::Result<()> {
    output_file.write_all(format!("{}\n", contents).as_bytes())
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

fn command(command: &str, args: &[&str]) -> io::Result<ExitStatus> {
    Command::new(command).args(args).reroute_stdout().status()
}
