use std::{env, u64};

mod bitcoin;
mod macros;

fn main() {
    let threads_amount: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(4);
    log_info!("Using {} threads.", threads_amount);

    let max_hashes: u64 = env::args()
        .nth(2)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(u64::MAX);
    log_info!("Storing max {} hashes.", max_hashes);

    bitcoin::init(threads_amount, max_hashes).unwrap();
}
