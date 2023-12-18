

pub struct Program<'a> {
    pub name: &'a str,
    pub full_name: &'a str,
    pub description: &'a str,
    run: fn(source: &String, program: &String, raw_add_args: &[String]) -> ()
}

macro_rules! get_programs {
    ( $( $name:ident) , *) => {
        $(
            mod $name;
        )*
        
        pub fn get_programs () -> Vec<Program<'static>> {
            vec![
                $(
                    $name::PROGRAM,
                )*
            ]
        }
        
        pub fn select_program(program_bundle: (&String, &String, &[String])) {
            let run_program = |program: Program| {
                if program_bundle.2.len() == 0 {
                    // run description
                    println!("Description for {}", program.full_name);
                    println!();
                    println!("{}", program.description);
                } else {
                    (program.run)(program_bundle.0, program_bundle.1, program_bundle.2)
                }
            };
            
            match program_bundle.1.as_str() {
                $(
                p if p == $name::PROGRAM.name => run_program($name::PROGRAM),
                )*
                _ => {}
            }
        }
    };
}

get_programs![hwd, ls, t];


