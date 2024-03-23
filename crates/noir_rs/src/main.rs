use tracing_subscriber;

pub mod netsrs;

fn main() {
    tracing_subscriber::fmt::init();
}
