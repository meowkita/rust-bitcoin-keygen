mod bitcoin;
mod macros;

fn main() {
    let hashes = match bitcoin::data::from() {
        Ok(hashes) => hashes,
        Err(error) => {
            log_error!("{error}");
            return;
        }
    };
}
