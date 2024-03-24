use tracing_subscriber;

pub mod netsrs;
pub mod prove;
pub mod verify;

fn main() {
    tracing_subscriber::fmt::init();
}
