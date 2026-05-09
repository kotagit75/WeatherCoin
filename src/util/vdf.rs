use vdf::{PietrzakVDF, PietrzakVDFParams, VDF, VDFParams};

const VDF_DIFFICULTY: u64 = 100000;
const VDF_BITS: u16 = 1024;

fn create_vdf() -> PietrzakVDF {
    PietrzakVDFParams(VDF_BITS).new()
}

pub fn verify_solution(challenge: &[u8], solution: &[u8]) -> bool {
    create_vdf()
        .verify(challenge, VDF_DIFFICULTY, solution)
        .is_ok()
}

pub fn solve(challenge: &[u8]) -> Result<Vec<u8>, vdf::InvalidIterations> {
    create_vdf().solve(challenge, VDF_DIFFICULTY)
}
