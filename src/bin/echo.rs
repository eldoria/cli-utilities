use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut flag_e = false;
    let mut output = String::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-e" => flag_e = true,
            _ => output.push_str(&(arg.to_owned()  + " "))
        }
    }

    if flag_e {
        output = output.replace("\\n", "\n").replace("\\t", "\t");
    }
    print!("{}", output.trim_end());
}