use std::fs;

pub(crate) fn verify(dir: String) -> bool {
    let verifs = verify_file_exists(&dir, "docker-compose.yaml", "")
    && verify_file_exists(&dir, ".github/workflows/dockerpush.yaml", "\
    Initialise the project with init.");
    
    if verifs {
        println!("All good!");
        println!("Make sure your docker-compose is valid and updated.");
        true
    } else { false }
}

fn verify_file_exists(dir: &String, file: &str, extra_steps: &str) -> bool {
    if let Ok(meta) = fs::metadata(format!("{dir}/{file}")) {
        if meta.is_file() { return true; }
    }
    
    println!("Verification error: {dir}/{file} not found. This is required! {extra_steps}");
    false
}