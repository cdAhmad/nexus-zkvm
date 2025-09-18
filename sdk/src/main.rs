use nexus_sdk::stwo::seq::{ Proof, Stwo };
use nexus_sdk::{ Local, Prover };
use sha3::{ Digest, Keccak256 };
use thiserror::Error;
const ELF_PROVER: &[u8; 104004] = include_bytes!("../assets/fib_input_initial");

pub fn create_fib_prover() -> Result<Stwo<Local>, TestError> {
    Stwo::<Local>
        ::new_from_bytes(ELF_PROVER)
        .map_err(|e| {
            TestError::new(format!("Failed to load fib_input_initial guest program: {}", e))
        })
}

pub fn main() {
    match prov() {
        Ok(new_hash) => {
            let hash = "ca611e7d9f8fab9e0a5298ebe9db671b1cd84e7d3663420d3572a4d4f96f60c0";
            if new_hash == hash {
                println!("equals");
            } else {
                println!("not equalas hash {hash}\n new_hash {new_hash}");
            }
        }
        Err(e) => {}
    }
}

fn prov() -> Result<String, TestError> {
    let now = std::time::Instant::now();
    let inputs = &(5u32, 3791366113u32, 4014011445u32);
    let prover = create_fib_prover()?;
      let prover_bytes = postcard::to_allocvec(&prover).expect("Failed to serialize prover");
     println!("prover hash {}", format!("{:x}", Keccak256::digest(&prover_bytes)));
    let (view, proof) = prover
        .prove_with_input::<(), (u32, u32, u32)>(&(), inputs)
        .map_err(|e| {
            TestError::new(format!("Failed to generate proof for inputs {:?}: {}", inputs, e))
        })?;
    println!("prove_with_input {} milliseconds", now.elapsed().as_millis());
    Ok(generate_proof_hash(&proof))
}

pub fn generate_proof_hash(proof: &Proof) -> String {
    let proof_bytes = postcard::to_allocvec(proof).expect("Failed to serialize proof");
    format!("{:x}", Keccak256::digest(&proof_bytes))
}

/// Result of a proof generation, including combined hash for multiple inputs
pub struct TestError {
    pub msg: String,
}
impl TestError {
    pub fn new(msg: String) -> TestError {
        TestError { msg }
    }
}
