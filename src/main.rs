use std::env;
use std::fs;
const PATH: &str = "./head/TODO.md";
const CHAR_TO_LOOK_AT: &str = "- [x]"; 
const CHAR_NOT_DONE: &str = "- [ ]"; // This is a "just in case" commit
const CHAR_TASK_IDENTIFIER: &str = "- [";
const HELP: &str = "
Running with cargo or no arguments will use -n
Usage: 
    -n Basic output, outputs the number of unchecked tasks
    -t Outputs the number of tasks, done or not
    -d Outputs the number of done tasks
    -h Shows this help message
";
fn main() {
    // Obtenemos los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", count_unchecked());
    } else {
        let option = &args[1];
        match option.as_str() {
            "-n" => println!("To do: {}", count_unchecked()),
            "-t" => println!("{}", count_total()),
            "-d" => println!("{}", count_done()),
            "-h" => println!("{}", HELP),
            _ => println!("{}", HELP),
        }
    }
}
fn count_unchecked() -> usize {
    // Obtenemos el contenido del archivo
    let content = fs::read_to_string(PATH).expect("Error al leer el archivo");
    // Separamos el contenido en líneas
    let lines: Vec<&str> = content.split("\n").collect();
    // Contamos las líneas que no están marcadas como hechas
    let mut count = 0;
    for line in lines {
        if line.starts_with(CHAR_NOT_DONE) || line.starts_with(CHAR_TASK_IDENTIFIER) {
            count += 1;
        }
    }
    count
}
fn count_total() -> usize {
    let content: String = fs::read_to_string(PATH).expect("Error al leer el archivo");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut count = 0;
    for line in lines {
        if line.starts_with(CHAR_TASK_IDENTIFIER) {
            count += 1;
        }
    }
    count
}
fn count_done() -> usize {
    let content: String = fs::read_to_string(PATH).expect("Error al leer el archivo");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut count = 0;
    for line in lines {
        if line.starts_with(CHAR_TO_LOOK_AT) {
            count += 1;
        }
    }
    count
}
