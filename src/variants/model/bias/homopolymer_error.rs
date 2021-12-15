use std::hash::Hash;

use bio::stats::probs::LogProb;

use crate::variants::evidence::observation::ProcessedObservation;
use crate::variants::model::bias::Bias;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum HomopolymerError {
    None,
    Some,
}

impl HomopolymerError {
    pub(crate) fn values() -> Vec<Self> {
        vec![HomopolymerError::None, HomopolymerError::Some]
    }
}

impl Default for HomopolymerError {
    fn default() -> Self {
        HomopolymerError::None
    }
}

impl Bias for HomopolymerError {
    fn prob_alt(&self, observation: &ProcessedObservation) -> LogProb {
        match (observation.homopolymer_indel_len, self) {
            (Some(len), HomopolymerError::Some) => {
                dbg!((len, observation.prob_observable_at_homopolymer_artifact));
                observation.prob_observable_at_homopolymer_artifact.unwrap()
            }
            (Some(len), HomopolymerError::None) => {
                observation.prob_observable_at_homopolymer_variant.unwrap()
            }
            (None, HomopolymerError::None) => LogProb::ln_one(), // No error, and also no observed homopolymer indel
            (None, HomopolymerError::Some) => LogProb::ln_one(), // ignore observations without homopolymer indel
        }
    }

    fn prob_ref(&self, observation: &ProcessedObservation) -> LogProb {
        self.prob_alt(observation)
    }

    fn prob_any(&self, _observation: &ProcessedObservation) -> LogProb {
        LogProb::ln_one() // TODO check this
    }

    fn is_artifact(&self) -> bool {
        *self != HomopolymerError::None
    }

    fn is_informative(&self, pileups: &[Vec<ProcessedObservation>]) -> bool {
        if !self.is_artifact() {
            return true;
        }
        // METHOD: this bias is only relevant if there is at least one recorded indel operation (indel operations are only recorded for some variants).
        pileups
            .iter()
            .any(|pileup| pileup.iter().any(|obs| self.is_bias_evidence(obs)))
    }

    fn is_possible(&self, pileups: &[Vec<ProcessedObservation>]) -> bool {
        self.is_informative(pileups)
    }

    fn is_likely(&self, pileups: &[Vec<ProcessedObservation>]) -> bool {
        self.is_informative(pileups)
    }

    fn is_bias_evidence(&self, observation: &ProcessedObservation) -> bool {
        observation.homopolymer_indel_len.unwrap_or(0) != 0
    }

    fn min_strong_evidence_ratio(&self) -> f64 {
        0.01 // TODO possibly tune, this is very permissive
    }
}
