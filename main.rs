
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments provided !!");
        return Ok(());
    }

    let mut num_all = false;
    let mut num_nonblank = false;
    let mut squeeze_blank = false;
    let mut show_nonprinting = false;
    let mut show_tabs = false;
    let mut show_ends = false;
    let mut files = Vec::new();
    let mut arg_index = 1;
    let mut char_index = 0;
    let mut in_flag = false;

    // Parse arguments in a single loop
    while arg_index < args.len() {
        let arg = &args[arg_index];
        if char_index < arg.len() {
            let ch = arg.chars().nth(char_index).unwrap();
            if ch == '-' && char_index == 0 {
                in_flag = true;
            } else if in_flag && char_index > 0 {
                match ch {
                    'n' => num_all = true,
                    'b' => num_nonblank = true,
                    's' => squeeze_blank = true,
                    'v' => show_nonprinting = true,
                    't' => {
                        show_tabs = true;
                        show_nonprinting = true;
                    }
                    'e' => {
                        show_ends = true;
                        show_nonprinting = true;
                    }
                    _ => eprintln!("Unknown flag: -{}", ch),
                }
            } else if char_index == 0 {
                files.push(arg);
            }
            char_index += 1;
        } else {
            in_flag = false;
            arg_index += 1;
            char_index = 0;
        }
    }

    if num_nonblank {
        num_all = false;
    }

    // Process input (stdin or files)
    if files.is_empty() {
        process_input(&mut io::stdin(), num_all, num_nonblank, squeeze_blank, show_nonprinting, show_tabs, show_ends)?;
    } else {
        for file in files {
            let mut file = File::open(file)?;
            process_input(&mut file, num_all, num_nonblank, squeeze_blank, show_nonprinting, show_tabs, show_ends)?;
        }
    }
    io::stdout().flush()?;
    Ok(())
}

fn process_input<R: Read>(input: &mut R, num_all: bool, num_nonblank: bool, squeeze_blank: bool, show_nonprinting: bool, show_tabs: bool, show_ends: bool) -> io::Result<()> {
    let mut buffer = [0; 4096];
    let mut line_number = 1;
    let mut last_was_blank = false;
    let mut in_line = false;

    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for &c in &buffer[..bytes_read] {
            let is_blank = c == b'\n';
            if !in_line {
                if !is_blank || !squeeze_blank || !last_was_blank {
                    if num_all || (num_nonblank && !is_blank) {
                        print!("{:6}\t", line_number);
                        line_number += 1;
                    }
                }
                last_was_blank = is_blank;
                in_line = true;
            }

            if show_nonprinting && c < 32 && c != b'\n' && c != b'\t' {
                print!("^{}", (c + 64) as char);
            } else if show_tabs && c == b'\t' {
                print!("^I");
            } else {
                io::stdout().write_all(&[c])?;
            }

            if c == b'\n' {
                if show_ends {
                    print!("$");
                }
                in_line = false;
            }
        }
    }
    Ok(())
}
