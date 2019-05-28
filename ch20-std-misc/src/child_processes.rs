use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() -> std::io::Result<()> {
  let output = Command::new("rustc")
      .arg("--version")
      .output()
      .unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
      });
  if output.status.success() {
    let s = String::from_utf8_lossy(&output.stdout);
    print!("rustc succeeded and stdout was:\n{}", s);
  } else {
    let s = String::from_utf8_lossy(&output.stderr);
    print!("rustc failed and stderr was:\n{}", s);
  }
  let command: &str;
  if cfg!(target_os = "linux") {
    command = "wc";
  } else if cfg!(target_os = "windows") {
    command = "Measure-Object -Line -Character -Word";
  } else {
    command = "ls";
  }
  let process = Command::new(command)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()?;

  process.stdin.unwrap().write_all(PANGRAM.as_bytes())?;

  let mut s = String::new();
  match process.stdout.unwrap().read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {} stdout: {}", command, why.description()),
    Ok(_) => println!("{} responded with: \n{}", command, s),
  }
  Ok(())
}