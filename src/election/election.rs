use std::collections::HashMap;

use anyhow::{bail, Result};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::vote::Vote;

type F = <PoseidonGoldilocksConfig as GenericConfig<2>>::F;
type Proof = ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2>;

pub struct Election {
    pub total_candidates: usize,
    pub circuit: CircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2>,
    pub votes: HashMap<String, Vote<Proof>>,
    pub is_ongoing: bool,
    pub winner: Option<usize>,
    votes_number: Vec<u32>,
}

impl Election {
    pub fn new(total_candidates: usize) -> Self {
        Self {
            total_candidates,
            circuit: Self::construct_circuit(total_candidates),
            votes: HashMap::new(),
            is_ongoing: true,
            winner: None,
            votes_number: vec![0; total_candidates],
        }
    }

    pub fn add_vote(&mut self, name: String, candidate: usize) -> Result<()> {
        if !self.is_ongoing {
            bail!("Voting closed");
        }

        if candidate >= self.total_candidates {
            bail!("Wrong candidate");
        }

        self.votes_number[candidate] += 1;
        self.votes.insert(
            name,
            Vote::<Proof>::new(self.get_vote_proof(candidate), candidate),
        );

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if !self.is_ongoing {
            bail!("Voting is already closed");
        }

        self.is_ongoing = false;

        let mut winner: usize = 0;
        let mut max_votes: u32 = 0;

        // TODO resolve collisions
        self.votes_number
            .iter()
            .enumerate()
            .for_each(|(candidate, &votes)| {
                if votes >= max_votes {
                    winner = candidate;
                    max_votes = votes;
                }
            });

        self.winner = Some(winner);

        // TODO create election result proof

        Ok(())
    }

    fn get_vote_proof(&self, candidate: usize) -> Proof {
        let mut witness = PartialWitness::new();

        for i in 0..self.total_candidates {
            let val: u32 = if i == candidate { 1 } else { 0 };
            witness.set_target(
                Target::VirtualTarget { index: i },
                F::from_canonical_u32(val),
            );
        }

        witness.set_target(
            Target::VirtualTarget {
                index: self.total_candidates,
            },
            F::from_canonical_u32(1),
        );

        return self.circuit.prove(witness).unwrap();
    }

    fn construct_circuit(
        total_candidates: usize,
    ) -> CircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2> {
        let mut builder = CircuitBuilder::<F, 2>::new(CircuitConfig::default());

        let inputs: Vec<_> = (0..total_candidates)
            .map(|_| builder.add_virtual_target())
            .collect();
        let public_input = builder.add_virtual_target();
        let sum = builder.add_many(&inputs);
        builder.connect(sum, public_input);
        return builder.build::<PoseidonGoldilocksConfig>();
    }
}
