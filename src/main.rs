mod bitcoin;
mod macros;

fn main() {
    match bitcoin::data::from() {
        Ok(_) => println!("File loaded!"),
        Err(error) => eprintln!("{error}"),
    }
}
