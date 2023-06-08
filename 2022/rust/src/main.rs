use std::process::{Command, ExitStatus};
use std::{env, fs};

use std::io::Result;

pub fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}

fn main() -> Result<()> {
    fs::create_dir("tmp")?;
    env::args().skip(1).for_each(|arg| {
        match arg.parse::<u8>() {
            Ok(day) => {
                if day <= 25 {
                    //Curl - Input fetching
                    execute(
                        "curl",
                        &[
                            format!("https://adventofcode.com/2022/day/{}/input", day).as_str(),
                            "-o",
                            format!("./tmp/{}.in",day).as_str(),
                            "--cookie",
                            "session=53616c7465645f5fd39df44e5c5c3f394a4ce2a1fd1f561860dfff1c0bdee7fb1e2c4dccd139b84fa5abeb8373e0bdf0dfc234e22025f515263b2abe65151c03",
                        ],
                    )
                    .expect(format!("Problem fetching the input for {}", day).as_str());
                    
                    //Cargo - Code invoker
                    execute("cargo", &["run", "--bin", day.to_string().as_str()])
                        .expect("There was a problem running your command");
                }
            }
            Err(_) => println!("{} isn't a valid day arg", arg),
        };
    });
    fs::remove_dir_all("tmp")?;
    Ok(())
}
