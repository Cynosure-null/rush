use std::io;
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::io::prelude::*;
use std::path::Path;
use std::io::Write;
use std::process::exit;
use std::process::Command;
use std::io::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum IoEvent{
    Clear,
    Quit,
    Output,
    Nominal,
}
fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .ok()
        .expect("Couldn't read line");
    // Space left if any parsing needs to be done
    return user_input;
}

fn file_io(input: &str) -> IoEvent {
    //TODO: Better name
    let path = Path::new("/tmp/rush.in");
    let display = path.display();

    let mut old_file = std::fs::OpenOptions::new().write(true).truncate(true).open(&path).unwrap();

    if input == "//clear" {
        old_file.write_all(b"");
        old_file.flush();
        return IoEvent::Clear;
    }
    else if input == "//output"{
        //Print file
        return IoEvent::Output;
    }
    else if input == "//exit" {
        std::process::exit(0);
    }

    else {
        // Append new file to old file and compile
        write!(old_file, "{}", &input);
        return IoEvent::Nominal;
    }
}

fn compile(){
    let output =
        Command::new("rustc")
        .arg("/tmp/rush.in")
        .arg("-o")
        .arg("/tmp/rush.out")
        .spawn()
        .expect("Failed to compile");
    
    let chmod =
        Command::new("chmod")
        .arg("+x")
        .arg("/tmp/rush.out")
        .spawn()
        .expect("Failed to make executable");

    let execute =
        Command::new("/tmp/rush.out")
        .status()
        .expect("Failed to run");

}

fn flush() -> std::io::Result<()> {
    fs::remove_file("/tmp/rush.in")?;
    fs::remove_file("/tmp/rush.out")?;

    let mut in_file = File::create("/tmp/rush.in")?;
    let mut out_file = File::create("/tmp/rush.out")?;

    let mut in_perms = fs::metadata("/tmp/rush.in")?.permissions();
    in_perms.set_readonly(false);
    fs::set_permissions("/tmp/rush.in", in_perms)?;

    let mut out_perms = fs::metadata("/tmp/rush.out")?.permissions();
    out_perms.set_readonly(false);
    fs::set_permissions("/tmp/rush.in", fs::Permissions::from_mode(0o655)).unwrap();

    Ok(())
}

fn main() {
    println!("Welcome to the Rust Shell!");

    flush();

    loop {
        print!("R > ");
        let mut input = get_user_input();
        if (file_io(&input) == IoEvent::Nominal){
            compile();
        }
    }
}
