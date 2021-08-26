use std::{
    env::{args, Args},
    io::{stdin, stdout, BufRead, Lines, Stdin, StdinLock, Stdout, Write},
    process::exit,
};

fn main() {
    let mut find_mode: bool = true;

    let mut case_insensitive: bool = false;

    let string: String = {
        let mut string: Option<String> = None;

        {
            let argument_count: usize;

            for (zeroth, argument) in {
                let args: Args = args();

                argument_count = args.len();

                let mut zeroth: bool = true;

                args.map(move |argument: String| {
                    let result_bool: bool = zeroth;

                    if zeroth {
                        zeroth = false;
                    }

                    (result_bool, argument)
                })
            } {
                if let Some(_) = argument.strip_prefix("--") {
                    eprintln!("Long name options are not supported!");

                    exit(1);
                } else if let Some(argument) = argument.strip_prefix('-') {
                    if argument.is_empty() {
                        eprintln!("Found empty option argument!");

                        return;
                    }

                    for option in argument.chars() {
                        match option {
                            'n' => find_mode = false,
                            'i' => case_insensitive = true,
                            _ => {
                                eprintln!("Unknown short name option '{}'!", option);

                                exit(1);
                            }
                        }
                    }
                } else {
                    if string.is_some() {
                        eprintln!("More than one matching string passed!");

                        return;
                    } else if argument_count == 1 {
                        eprintln!(
                            "[WARNING] Found only one argument! Assuming it is matching string!"
                        );
                    } else if !zeroth {
                        string = Some(argument);
                    }
                }
            }

            if let Some(mut string) = string {
                if case_insensitive {
                    string = string.to_lowercase();
                }

                string
            } else {
                eprintln!("No matching string passed!");

                exit(1);
            }
        }
    };

    let stdout: Stdout = stdout();

    let stdin: Stdin = stdin();

    let stdin: StdinLock = stdin.lock();

    let mut lines: Lines<StdinLock> = stdin.lines();

    while let Some(Ok(mut line)) = lines.next() {
        if case_insensitive {
            line = line.to_lowercase();
        }

        if line.find(&string).is_some() == find_mode {
            stdout
                .lock()
                .write_fmt(format_args!("{}\n", line))
                .expect("Failed writing to StdOut!");
        }
    }
}
