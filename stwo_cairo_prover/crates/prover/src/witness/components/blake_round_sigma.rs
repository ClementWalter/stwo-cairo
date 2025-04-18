#![allow(unused_parens)]
#![allow(dead_code)]
use std::simd::u32x16;

use cairo_air::components::blake_round_sigma::{Claim, InteractionClaim, LOG_SIZE};
use itertools::{chain, Itertools};

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << LOG_SIZE),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self.mults.into_simd_vec();
        let multiplicity_column = BaseColumn::from_simd(mults.clone());

        let domain = CanonicCoset::new(LOG_SIZE).circle_domain();
        let trace = [multiplicity_column]
            .map(|col| CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, col));
        let lookup_data = LookupData { mults };

        tree_builder.extend_evals(trace);

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at(input[0].0);
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        blake_round_sigma: &relations::BlakeRoundSigma,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        let mults = <_ as Into<PackedQM31>>::into(self.lookup_data.mults[0]);
        let sigmas = PackedBlakeRoundSigma::deduce_output(unsafe {
            PackedM31::from_simd_unchecked(u32x16::from_array([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0,
            ]))
        });
        let seq = unsafe { PackedM31::from_simd_unchecked(SIMD_ENUMERATION_0) };

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        let values = chain![[seq], sigmas].collect_vec();
        let denom = blake_round_sigma.combine(&values);
        col_gen.write_frac(0, -PackedQM31::one() * mults, denom);
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
