use acir::{
    native_types::{Witness, WitnessMap},
    FieldElement,
};
use prove::prove;
use tracing::info;
use verify::verify;

pub mod netsrs;
pub mod prove;
pub mod verify;

const BYTECODE: &str = "H4sIAAAAAAAA/7WUPQ7DIAyFTZNWHXsUm59gtlylqOT+J6iqqqmCiDfMW2CwzGc/mxkArnDWtJ/rfjpcvC/RFnL0RJsyB/QhL0xMgcPLsnOFPceUU8RE3hXaQnIb/lTnwj6RUeS66HHht2dG6KVpeol9Ik1m03j+n4WbwF/Htfd7FfdWrLV9t2V5CJwnD1ZFmBFmTgPyzqC7vCPqnvU9QhAGYkRPsVMGjuUxArP0kcAH+JIvC64FAAA=";

fn main() {
    tracing_subscriber::fmt::init();

    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(1), FieldElement::zero());
    initial_witness.insert(Witness(2), FieldElement::one());

    let (proof, vk) = prove(String::from(BYTECODE), initial_witness).unwrap();
    let verdict = verify(String::from(BYTECODE), proof, vk).unwrap();
    info!("proof verification verdict: {}", verdict);
}
