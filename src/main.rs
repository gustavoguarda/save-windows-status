extern crate gtk;
extern crate serde;
extern crate serde_json;
extern crate dirs;
extern crate dotenv;

use gtk::prelude::*;
use gtk::{Button, CheckButton, Fixed, Window, WindowType, Inhibit};
use dotenv::dotenv;
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};
use serde_json::json;
use dirs::home_dir;

#[derive(Serialize, Deserialize)]
struct AppWindow {
    window_id: String,
    app_command: String,
    window_title: String,
}

fn check_wmctrl_installed() -> bool {
    Command::new("wmctrl")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn get_running_apps() -> Vec<AppWindow> {
    let output = Command::new("wmctrl")
        .arg("-lpG")
        .output()
        .expect("Failed to execute wmctrl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();

    for line in stdout.lines() {
        if let Some(app_window) = parse_wmctrl_line(line) {
            apps.push(app_window);
        }
    }
    apps
}

fn parse_wmctrl_line(line: &str) -> Option<AppWindow> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 9 {
        return None;
    }
    let window_id = parts[0].to_string();
    let pid = parts[2];
    let window_title = parts[8..].join(" ");

    // Get the command used to open the application
    let app_command = match get_command_by_pid(pid) {
        Some(command) => command,
        None => return None,
    };

    Some(AppWindow {
        window_id,
        app_command,
        window_title,
    })
}

fn get_command_by_pid(pid: &str) -> Option<String> {
    let output = Command::new("ps")
        .arg("-p")
        .arg(pid)
        .arg("-o")
        .arg("args=")
        .output()
        .expect("Failed to execute ps");

    let command = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if command.is_empty() {
        None
    } else {
        Some(command)
    }
}

fn get_save_path() -> std::path::PathBuf {
    dotenv().ok();
    let mode = env::var("APPLICATION_ENV").unwrap_or_default();
    if mode == "development" {
        Path::new("apps_state.json").to_path_buf()
    } else {
        home_dir().unwrap().join(".save-windows-status").join("apps_state.json")
    }
}

fn save_app_state(apps: Vec<AppWindow>) {
    let path = get_save_path();

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }
    }

    let file = File::create(&path).expect("Failed to create file");
    let json = json!(apps);
    serde_json::to_writer(file, &json).expect("Failed to write to file");
    println!("Application state saved at: {}", path.display());
}

fn clear_app_state() {
    let path = get_save_path();

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }
    }

    let file = File::create(&path).expect("Failed to create file");
    let empty_json = json!([]);
    serde_json::to_writer(file, &empty_json).expect("Failed to write to file");
    println!("File {} cleared.", path.display());
}

fn restore_apps() {
    let path = get_save_path();

    let file = File::open(&path).expect("Failed to open file");
    let apps: Vec<AppWindow> = serde_json::from_reader(file).expect("Failed to read from file");

    for app in apps {
        restore_app(&app);
    }
}

fn restore_app(app: &AppWindow) {
    Command::new("sh")
        .arg("-c")
        .arg(&app.app_command)
        .spawn()
        .expect("Failed to relaunch app");
}

fn main() {
    if !check_wmctrl_installed() {
        eprintln!("Error: wmctrl is not installed. Please install wmctrl and try again.");
        eprintln!("On Ubuntu/Debian, use: sudo apt-get install wmctrl");
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "--restore" {
            restore_apps();
            println!("Application state restored.");
            return;
        }
    }

    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Save Windows Status");
    window.set_default_size(350, 150);

    // Create a Fixed container
    let fixed = Fixed::new();

    // Create the CheckButton
    let check_button = CheckButton::with_label("Reopen windows on next session start?");
    fixed.put(&check_button, 20, 20);

    // Create the Cancel button
    let cancel_button = Button::with_label("Cancel");
    cancel_button.set_size_request(100, 40);  // Set the button size
    fixed.put(&cancel_button, 50, 70);
    cancel_button.connect_clicked(|_| {
        println!("Cancel clicked!");
        gtk::main_quit();
    });

    // Create the Continue button
    let continue_button = Button::with_label("Continue");
    continue_button.set_size_request(100, 40);  // Set the button size
    fixed.put(&continue_button, 200, 70);
    continue_button.connect_clicked(move |_| {
        let is_checked = check_button.get_active();
        if is_checked {
            let apps = get_running_apps();
            save_app_state(apps);
            println!("Application state saved.");
        } else {
            clear_app_state();
            println!("JSON file cleared.");
        }
        gtk::main_quit();
    });

    // Add the Fixed container to the window
    window.add(&fixed);

    // Set up window close handling
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all window widgets
    window.show_all();

    // Start the GTK main loop
    gtk::main();
}
