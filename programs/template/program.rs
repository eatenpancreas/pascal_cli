use std::{env};
use std::fs::File;
use std::process::Command;
use clap::Args;

#[derive(serde::Deserialize)]
struct Templates {
    templates: Vec<Template>,
}

#[derive(serde::Deserialize)]
struct Template {
    name: String,
    link: String,
    description: String,
    additional_descriptions: Vec<String>,
    additional_steps: Vec<String>,
    additional_commands: Vec<Vec<String>>,
}

fn get_templates_file() -> Vec<Template> {
    let exe = env::current_exe()
        .expect("Couldn't find current exe");
    let path = exe.parent()
        .expect("Couldn't find current exe");
    let string_path = path.to_str()
        .expect("Couldn't find current exe, the path may be corrupt");
    let f = File::open(format!("{}/pan_c/templates.json", string_path))
        .expect(&*format!("Couldn't find pan_c directory next to executable! at {}", string_path));
    let t: Templates = serde_json::from_reader(f).unwrap();
    t.templates
}

#[derive(Debug, Args)]
pub struct ProgramArgs {
    /// The name of the template. (ls to list templates)
    #[arg(short, long, default_value = "ls")]
    name: String,
    /// The directory that the template will be put in. [default: name of template]
    #[arg(short, long)]
    dir: Option<String>,
}

pub fn run(args: ProgramArgs) {
    let templates= get_templates_file();
    if args.name == "ls" {
        println!("No template selected");
        println!("For more info, run pan template --help");
        println!();
        println!("Templates:");
        println!();
        for template in &templates {
            println!("{} - {}", template.name, template.description);
            for additional_description in &template.additional_descriptions {
                println!("  | {}", additional_description);
            }
            println!();
        }
    } else {
        let selected_template = templates.iter().find(|t| t.name == args.name)
            .expect("Could not find template!");
        
        println!("Cloning template {}...", args.name);
        
        let dir_name = args.dir.unwrap_or(args.name);
        
        call(Command::new("git").arg("clone")
            .arg("--depth=1")
            .arg("--branch=master")
            .arg(&selected_template.link)
            .arg(&dir_name)
        );

        call(Command::new("rm").arg("-rf")
            .arg(format!("./{}/.git", dir_name))
        );

        env::set_current_dir(format!("./{}/", dir_name)).unwrap();
        
        for args in &selected_template.additional_commands {
            let mut args = args.iter();
            if let Some(first_arg) = args.next() {
                let mut cmd = Command::new(first_arg);
                
                for arg in args { cmd.arg(arg); }
                
                call(&mut cmd);
            }
        }
        
        if selected_template.additional_steps.len() > 0 {
            println!("The following steps may be required:");
        }
        for step in &selected_template.additional_steps {
            println!("{}", step);
        }
    }
}

fn call(cmd: &mut Command) {
    let output = cmd.output().expect("failed to execute process");
    let stderr = String::from_utf8 (output.stderr).unwrap();
    let stdout = String::from_utf8 (output.stdout).unwrap();
    
    if stderr.contains("fatal") || stdout.contains("fatal") {
        panic!("Fatal error encountered in creating template! {} | {}", stderr, stdout);
    }

    if !stdout.is_empty() { println!("out > {}", stdout); }
    if !stderr.is_empty() { println!("err > {}", stderr); }
}