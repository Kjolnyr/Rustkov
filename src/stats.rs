use std::{collections::HashSet, vec};

use super::{brain_prelude::StateElement, prelude::Brain};

/// This struct let you compute some statistics of a [`Brain`].
///
/// Beware that it is not very optimised, nor complete for now.
///
/// # Example
///
/// ```
/// use rustkov::prelude::Brain;
///
/// let brain = Brain::from_file("path/to/brain.yml").unwrap()
///                         .get();
///
/// let stats = brain.stats();
///
/// println!("{}", stats.get_total_states());
///
/// ```
///
/// [`Brain`]: crate::brain::Brain
pub struct BrainStats<'a> {
    brain: &'a Brain,
}

impl<'a> BrainStats<'a> {
    pub(crate) fn new(brain: &'a Brain) -> Self {
        Self { brain }
    }

    /// Returns the length of states that the brain have.
    pub fn get_total_states(&self) -> usize {
        self.brain.state_transitions.len()
    }

    /// Returns the number of transitions that the brain have.
    ///
    /// A single state has multiple transitions
    pub fn get_total_transitions(&self) -> usize {
        self.brain
            .state_transitions
            .iter()
            .map(|(_, transition)| transition.prev.len() + transition.next.len())
            .sum()
    }

    /// Returns the average of the last two metrics.
    ///
    /// It is useful to see if your chatbot will be able to
    /// construct unique sentences
    pub fn avg_transition_per_state(&self) -> f32 {
        return self.get_total_transitions() as f32 / self.get_total_states() as f32;
    }

    /// Retruns the total number of single words
    /// known to the brain.
    pub fn get_total_words(&self) -> usize {
        let mut words: Vec<&str> = vec![];

        self.brain.state_transitions.iter().for_each(|(state, _)| {
            state
                .0
                .iter()
                .filter_map(|elem| {
                    if let StateElement::Word(e) = elem {
                        Some(e.as_str())
                    } else {
                        None
                    }
                })
                .for_each(|ref word| {
                    words.push(word);
                });
        });

        let set: HashSet<_> = words.drain(..).collect();
        set.len()
    }
}
