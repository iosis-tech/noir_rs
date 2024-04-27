use acvm::{
    acir::native_types::{Witness, WitnessMap},
    FieldElement,
};
use prove::prove;
use tracing::info;
use verify::verify;

pub mod prove;
pub mod srs;
pub mod verify;

const BYTECODE: &str = "H4sIAAAAAAAA/7WUUQ6DIAyGqTK3192kFdDy5lVmhvc/wTKNmBEke7H8CaEJSfn4Wwpql17XTZ3Vxn2Ku8HB2jD2gQy9sPczO7RuHpiYHLt3z8YEtjz62Y/oyZpAi/NmwV1pLrwmAkGuRo4LN8+g4CVkXuI1kSQzZDW/x7gr8B96rKuJ8UeQ5WDYlPbkcZdOzruEBeRZSGW+5B48C/6caj8JwtRoorZCXq1kh0aNd2v5GqEqNEQNT/GiQP0+ERSY/w0w9QU2ntcLNgYAAA==";

fn main() {
    tracing_subscriber::fmt::init();

    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::zero());
    initial_witness.insert(Witness(1), FieldElement::one());

    let (proof, vk) = prove(String::from(BYTECODE), initial_witness).unwrap();
    // let verdict = verify(String::from(BYTECODE), proof, vk).unwrap();
    // info!("proof verification verdict: {}", verdict);
    todo!()
}
