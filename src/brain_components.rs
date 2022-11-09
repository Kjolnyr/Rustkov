use crate::brain_prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct State(pub(crate) Vec<StateElement>);
impl State {
    pub(crate) fn random_element(&self, rng: &mut dyn RngCore) -> &StateElement {
        self.0.choose(rng).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transistion {
    pub(crate) prev: Vec<(StateElement, usize)>,
    pub(crate) next: Vec<(StateElement, usize)>,
}
impl Transistion {
    pub(crate) fn increment_occurence(
        &mut self,
        direction: SentenceDirection,
        new_element: &StateElement,
    ) {
        if *new_element == StateElement::Marker(SentenceMarker::Placeholder) {
            return;
        }

        let working_vec = match direction {
            SentenceDirection::Backward => &mut self.prev,
            SentenceDirection::Forward => &mut self.next,
        };

        let mut exists = false;
        for (element, occurence) in working_vec.iter_mut() {
            if element != new_element {
                continue;
            }
            *occurence += 1;
            exists = true;
        }

        if !exists {
            working_vec.push((new_element.clone(), 1));
        }
    }
}
