mod keccack;
mod shake128;
mod sponge;
mod state;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: shake128 <d>");
        std::process::exit(1);
    }

    let d: usize = args[1].parse().expect("Please enter a valid number");

    let result = shake128::from_stdin(d);

    for i in 0..d {
        print!("{:02x}", result[i]);
    }
    println!();
}
