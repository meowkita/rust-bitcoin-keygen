use std::env;

mod bitcoin;
mod macros;

fn main() {
    let threads_amount: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(4);
    log_info!("Using {} threads.", threads_amount);

    bitcoin::init(threads_amount).unwrap();
}
