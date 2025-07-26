use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    println!(".desktop gen but rusty :D");

    let name = read_input("Application Name: ");
    let exec = read_input("Path to exec: ");
    let icon = read_input("Path to icon: ");
    let comment = read_input("Comment: ");

    let desktop_file_content = format!(
        "[Desktop Entry]
Name={}
Exec={}
Icon={}
Comment={}
Type=Application",
        name, exec, icon, comment
    );

    let filename = format!("{}.desktop", name.to_lowercase());
    let filepath = format!("{}{}", get_home_dir(), filename);
    println!("{}", filepath);

    let file = File::create(filepath.clone()).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(desktop_file_content.as_bytes()).unwrap();
    writer.flush().unwrap();

    println!("The .desktop was created succesfully :)");
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn get_home_dir() -> String {
    let home_dir = env::var("HOME").unwrap();
    let applications_dir = format!("{}/.local/share/applications/", home_dir);

    return applications_dir;
}
