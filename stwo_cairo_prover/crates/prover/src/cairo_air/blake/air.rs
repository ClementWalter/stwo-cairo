// use stwo_prover::constraint_framework::TraceLocationAllocator;
// use stwo_prover::core::air::ComponentProver;
// use stwo_prover::core::backend::simd::SimdBackend;

// use crate::cairo_air::air::{CairoClaim, CairoInteractionClaim, CairoInteractionElements};
// use crate::components::{blake_g, blake_round, blake_round_sigma, triple_xor_32};

// pub struct BlakeComponents {
//     round: blake_round::Component,
//     g: blake_g::Component,
//     sigma: blake_round_sigma::Component,
//     triple_xor_32: triple_xor_32::Component,
// }

// impl BlakeComponents {
//     pub fn new(
//         tree_span_provider: &mut TraceLocationAllocator,
//         cairo_claim: &CairoClaim,
//         interaction_elements: &CairoInteractionElements,
//         interaction_claim: &CairoInteractionClaim,
//     ) -> Self {
//         let blake_round_component = blake_round::Component::new(
//             tree_span_provider,
//             blake_round::Eval {
//                 claim: cairo_claim.blake_round,
//                 blake_g_lookup_elements: interaction_elements.blake_g.clone(),
//                 blake_round_lookup_elements: interaction_elements.blake_round.clone(),
//                 blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
//                 memory_address_to_id_lookup_elements: interaction_elements
//                     .memory_address_to_id
//                     .clone(),
//                 memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
//                 range_check_7_2_5_lookup_elements: interaction_elements
//                     .range_checks
//                     .rc_7_2_5
//                     .clone(),
//             },
//             interaction_claim.blake_round.claimed_sum,
//         );

//         let blake_g_component = blake_g::Component::new(
//             tree_span_provider,
//             blake_g::Eval {
//                 claim: cairo_claim.blake_g,
//                 blake_g_lookup_elements: interaction_elements.blake_g.clone(),
//                 verify_bitwise_xor_12_lookup_elements: interaction_elements
//                     .verify_bitwise_xor_12
//                     .clone(),
//                 verify_bitwise_xor_4_lookup_elements: interaction_elements
//                     .verify_bitwise_xor_4
//                     .clone(),
//                 verify_bitwise_xor_7_lookup_elements: interaction_elements
//                     .verify_bitwise_xor_7
//                     .clone(),
//                 verify_bitwise_xor_8_lookup_elements: interaction_elements
//                     .verify_bitwise_xor_8
//                     .clone(),
//                 verify_bitwise_xor_9_lookup_elements: interaction_elements
//                     .verify_bitwise_xor_9
//                     .clone(),
//             },
//             interaction_claim.blake_g.claimed_sum,
//         );

//         let blake_sigma_component = blake_round_sigma::Component::new(
//             tree_span_provider,
//             blake_round_sigma::Eval {
//                 blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
//             },
//             interaction_claim.blake_sigma.claimed_sum,
//         );

//         let triple_xor_32_component = triple_xor_32::Component::new(
//             tree_span_provider,
//             triple_xor_32::Eval {
//                 claim: cairo_claim.triple_xor_32,
//                 triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
//                 verify_bitwise_xor_8_lookup_elements: interaction_elements
//                     .verify_bitwise_xor_8
//                     .clone(),
//             },
//             interaction_claim.triple_xor_32.claimed_sum,
//         );

//         Self {
//             round: blake_round_component,
//             g: blake_g_component,
//             sigma: blake_sigma_component,
//             triple_xor_32: triple_xor_32_component,
//         }
//     }

//     pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
//         vec![
//             &self.round as &dyn ComponentProver<SimdBackend>,
//             &self.g as &dyn ComponentProver<SimdBackend>,
//             &self.sigma as &dyn ComponentProver<SimdBackend>,
//             &self.triple_xor_32 as &dyn ComponentProver<SimdBackend>,
//         ]
//     }
// }
