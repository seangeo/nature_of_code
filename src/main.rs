pub mod nature_of_code;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <exercise>", args[0]);
        std::process::exit(1);
    }

    let exercise = &args[1];
    
    match exercise.as_str() {
        "ex_0_1" => nature_of_code::ex_0_1::run(),
        _ => {
            eprintln!("Unknown exercise: {}", exercise);
            std::process::exit(1);
        }
    }
}
