use clap::{ App};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::io::Read;
use std::path::{PathBuf};

const DIRNOTE_FILE_NAME : &str = ".dirnote";

// Returns the path of the notes file relative to the current working directory
fn get_note_file_path() -> PathBuf {
    let pwd = env::current_dir().unwrap();
    let dirnote_file_path = pwd.join(DIRNOTE_FILE_NAME);
    return dirnote_file_path;
}

// Sets the contents of the current diretory's note
fn set_note(new_note :  &str) -> std::io::Result<()> {
    let mut note_file = File::create(get_note_file_path())?;
    note_file.write_all(new_note.as_bytes())?;
    Ok(())
}

// Returns the contents of the current directory's note, or None if there is no note
fn get_note(path : PathBuf) -> std::io::Result<Option<String>> {
    if !fs::metadata(&path).is_ok() {
        return Ok(None);
    }
    let mut note_file = File::open(path)?;
    let mut contents = String::new();
    note_file.read_to_string(&mut contents)?;
    return Ok(Some(contents));
}

// Deletes the current directory's note
fn delete_note() -> std::io::Result<()> {
    let note_file = fs::remove_file(get_note_file_path());
    match note_file {
        Ok(_) => {
            println!("{}", "Note deleted");
        }
        _ => println!("{}", "Failed to read notes file"),
    };
    Ok(())
}

// List notes of the subdirectories of the current directory
fn ls() -> std::io::Result<()> {
    let pwd = env::current_dir().unwrap();
    for entry in fs::read_dir(&pwd)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let note_path = path.join(DIRNOTE_FILE_NAME);
        let note_contents = get_note(note_path);

        // Strings to allow  us to print directory names relative to the current dir
        let path_string = path.as_path().display().to_string();
        let pwd_string = pwd.as_path().display().to_string();
        let relative_path_string = path_string.replace(&pwd_string, "");

        match note_contents {
            Ok(contents) => {
                match contents {
                    Some(text) => println!("{}: {}", relative_path_string, text),
                    None => println!("{}: {}", relative_path_string, ""),
                }
            },
            _ => println!("{}: Failure reading note", relative_path_string)
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("Dirnote")
        .version("0.1")
        .author("Subhi Dweik")
        .about("Set notes for a directory")
        .subcommand(App::new("set")
            .about("Set the current directory's note")
            .arg_from_usage("<note>... 'The note to set for this dir'")
        ).subcommand(App::new("delete")
            .about("Delete the current directory's note")
        ).subcommand(App::new("ls")
            .about("Show dirnotes for all subdirectories of the current dir")
        ).get_matches();

    match matches.subcommand() {
        ("set", Some(set_command)) => {
            let values = set_command.values_of("note").unwrap();
            let note_text = values.fold(String::from(""), |acc, val| format!("{} {}", acc, val));
            set_note(note_text.trim()).unwrap();
        }
        ("delete", Some(_)) => delete_note().unwrap(),
        ("ls", Some(_)) => ls().unwrap(),
        ("", None) => {
             let note = get_note(get_note_file_path());
             match note {
                 Ok(contents) => {
                     if let Some(msg) = contents {
                        println!("{}", msg)
                     }
                 },
                 _ => println!("Failed to read note")
             }
        }
        _ => println!("Not sure what this is")
    }
}
