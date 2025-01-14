pub struct Vote<TProof> {
    pub proof: TProof,
    pub(super) candidate: usize,
}

impl<TProof> Vote<TProof> {
    pub(super) fn new(proof: TProof, candidate: usize) -> Self {
        Self { candidate, proof }
    }
}
