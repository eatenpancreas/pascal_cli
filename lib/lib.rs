use std::env;
use std::fs::File;

pub fn get_local_file(local_path: &str) -> File {
    let exe = env::current_exe()
        .expect("Couldn't find current exe");
    let path = exe.parent()
        .expect("Couldn't find current exe");
    let string_path = path.to_str()
        .expect("Couldn't find current exe, the path may be corrupt");
    File::open(format!("{}/pan_c/{}", string_path, local_path))
        .expect(&*format!("Couldn't find pan_c directory next to executable! at {}", string_path))
}

pub fn write_local_file(local_path: &str) -> File {
    let exe = env::current_exe()
        .expect("Couldn't find current exe");
    let path = exe.parent()
        .expect("Couldn't find current exe");
    let string_path = path.to_str()
        .expect("Couldn't find current exe, the path may be corrupt");
    File::create(format!("{}/pan_c/{}", string_path, local_path))
        .expect(&*format!("Couldn't find pan_c directory next to executable! at {}", string_path))
}