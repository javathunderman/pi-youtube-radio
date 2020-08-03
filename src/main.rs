#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::{Form};
use rocket::response::NamedFile;
use std::fs;
use std::process::Command;

#[cfg(test)] mod tests;

fn run_command(test: String) -> String {
    fs::remove_file("play.wav");
    let output = Command::new("youtube-dl")
                     .arg(test)
                     .arg("--extract-audio")
                     .arg("--audio-format")
                     .arg("wav")
                     .output()
                     .expect("failed to execute process");
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let file_name = path.unwrap().path().display().to_string();
        if file_name.contains("wav") {
            fs::rename(file_name, "play.wav");
        };
    }
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn play_song() -> () {
    let flag = std::path::Path::new("play.wav").exists();
    if !flag {
        return
    };
    if whoami::username() == "pi" {
        Command::new("sudo ./home/pi/fm_transmitter/fm_transmitter")
          .arg("-f 102.9")
          .arg("play.wav")
          .spawn()
          .expect("failed to play song");
    }
    else {
        Command::new("vlc").arg("play.wav").spawn().expect("Fail");
    };
    println!("Started");
}

#[derive(Debug, FromForm)]
struct FormInput {
    #[form(field = "textarea")]
    text_area: String,
}

#[post("/", data = "<sink>")]
fn sink(sink: Form<FormInput>) -> String {
    let form_data = &sink.text_area;
    let download_output = run_command(form_data.to_string());
    play_song();
    format!("{}", download_output)
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, sink])
}

fn main() {
    rocket().launch();
}
