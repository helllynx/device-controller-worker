use std::process::Command;

fn unlock_screen_kde() -> &str {
    // loginctl lock-session 2
    let output = Command::new("loginctl")
    .arg("lock-session 2")
    .output()
    .expect("Failed to execute unlock screen command");

    return  output.stdout.as_slice()
}

