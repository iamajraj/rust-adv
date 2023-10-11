use std::{
    collections::HashMap,
    env::*,
    fs::{self, File},
    io::ErrorKind,
    process::{self, Command},
    vec,
};

macro_rules! my_mac {
    ( $( $x:expr => $s:expr ),* ) => {
        {
            let mut m_hash = HashMap::new();
            $(
                m_hash.insert($x, $s);
            )*
            m_hash
        }
    }
}

use rust_expl::{html, main};

fn take_ref(r: &mut String) {
    r.push_str("Hello World");
}

// tasklist /fi "pid eq 6480" /nh /fo:csv
#[main]
fn main() {
    let mut mystr = String::new();
    take_ref(&mut mystr);
    println!("{:}", mystr);
    let my_mac_init = my_mac![0 => "Raj"];

    html!(
        <div>
            <p>Hello World </p>
        </div>
    );

    println!("{:?}", my_mac_init);
    println!("My Pid {:}", process::id());
    let search_filter = format!("pid eq {:}", process::id());
    let command = Command::new("tasklist")
        .args(["/fi", search_filter.as_str(), "/nh", "/fo:csv"])
        .output()
        .unwrap();

    println!(
        "The current process name is: {:}",
        String::from_utf8(command.stdout)
            .unwrap()
            .split(",")
            .nth(0)
            .unwrap()
            .replace("\"", "")
    );

    let first_argument = args().nth(2).unwrap_or_else(|| String::from("failed"));
    println!("Hello {:}!", first_argument);
    if first_argument == "failed" {
        println!("You didn't provided the argument");
    }
    let f_path = format!("{:}.txt", first_argument);
    create_file(f_path);

    let mut v = vec![String::from("Hello World")];
    v.push(String::from("Next World"));

    v.iter().for_each(|val| {
        println!("{:}", val);
    })
}

fn create_file(f_path: String) {
    match File::create(f_path.clone()) {
        Ok(_) => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            fs::write(f_path, input).unwrap();
            println!("File created successfully!");
        }
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => {
                match fs::remove_file(f_path.clone()) {
                    Ok(_) => {
                        create_file(f_path);
                    }
                    Err(_) => {
                        println!("Failed to remove file");
                    }
                };
            }
            _ => {
                panic!("Something went wrong")
            }
        },
    };
}
