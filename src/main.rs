use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

fn main() {
    if let Err(e) = write_readme() {
        eprintln!("File system failure: {}", e.to_string());
        return;
    }

    println!("HERE");
    match git_commands() {
        Ok(stdout) => {
            println!("{}", stdout);
        }
        Err(stderr) => {
            eprintln!("{}", stderr);
        }
    }
}

fn git_commands() -> Result<String, String> {
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

fn command(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}
