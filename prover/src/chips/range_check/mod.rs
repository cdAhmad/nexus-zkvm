//! Range-checking implementation. Constrains certain columns to only contain values from a specified range,
//! optionally depending on a flag.
//!
//! This is done with the use of [`stwo_constraint_framework::logup::LookupElements`]
//! (excluding {0, 1} bool check).
//!
//! Currently a verifier is not protected against summing up multiplicity of the tuple to the modulus of M31.
//! This may allow the prover to lookup invalid values, but it also requires using the same constrained tuple
//! exactly `M31::P` times.
//!
//! The current guard is to limit the size of the trace such that `2.pow(trace_log_size) * NUM_CHECKED_COLS < M31::P`
//! for every chip.

pub(crate) mod range128;
pub(crate) mod range16;
pub(crate) mod range256;
pub(crate) mod range32;
pub(crate) mod range8;
pub(crate) mod range_bool;

pub type RangeCheckChip = (
    range8::Range8Chip,
    range16::Range16Chip,
    range32::Range32Chip,
    range128::Range128Chip,
    range256::Range256Chip,
    range_bool::RangeBoolChip,
);
