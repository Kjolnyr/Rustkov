use rand::{Rng, SeedableRng};

use crate::brain_prelude::*;

/// The brain is the main struct of this library.
/// It contains the hashmap that represents the markov chain,
/// and the configuration.
///
/// The configuration is editable at runtime using [`BrainConfig`].
///
/// [`BrainConfig`]: crate::config::BrainConfig
#[derive(Debug, Clone)]
pub struct Brain {
    /// The brain configuration is exposed through this field.
    /// You can edit the configuration at runtime.
    pub config: BrainConfig,

    /// This hashmap represents the markov chain
    /// Each state is a combo of words, and a Transistion
    /// hold what word comes before and after the state
    /// the transitions are weighted
    pub state_transitions: HashMap<State, Transistion>,

    rng: ChaCha8Rng,
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            config: Default::default(),
            state_transitions: Default::default(),
            rng: ChaCha8Rng::from_entropy(),
        }
    }
}

impl Brain {
    /// Creates a new, empty brain
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let brain = Brain::new();
    ///
    /// // alternatively:
    /// let brain = Brain::default();
    ///
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a owned brain from a composition
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let dataset_path = "path/to/your/dataset.txt";
    ///
    /// let brain = Brain::new()
    ///                 .from_dataset(dataset_path).unwrap()
    ///                 .get();
    /// ```
    ///
    pub fn get(&mut self) -> Self {
        self.clone()
    }

    /// Set the brain configuration
    /// using [`BrainConfig`].
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::{Brain, BrainConfig};
    ///
    /// let brain = Brain::new()
    ///                 .config(BrainConfig {
    ///                     training: true,
    ///                     ..Default::default()
    ///                 }).unwrap()
    ///                 .get();
    ///
    /// assert_eq!(brain.config.training, true);
    /// ```
    ///[`BrainConfig`]: crate::config::BrainConfig
    pub fn config(&mut self, config: BrainConfig) -> Result<&mut Self> {
        self.config = config;
        Ok(self)
    }

    /// Create a brain from a dataset.
    ///
    /// It will ingest the dataset line by line.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let dataset_path = "path/to/your/dataset.txt";
    ///
    /// let brain = Brain::new()
    ///                 .from_dataset(dataset_path).unwrap()
    ///                 .get();
    /// ```
    ///
    pub fn from_dataset(&mut self, dataset_path: impl AsRef<str>) -> Result<&mut Self> {
        let dataset_path = dataset_path.as_ref();

        println!("Getting a new brain from '{}`...", dataset_path);
        let dataset_file = File::open(dataset_path)?;
        let mut lines = BufReader::new(dataset_file).lines();

        while let Some(Ok(line)) = lines.next() {
            self.ingest(&line);
        }
        Ok(self)
    }

    // let the brain learn from a text line.
    fn ingest(&mut self, line: &str) {
        let line = line.to_lowercase();

        // We get the input as str, turn it into a vec of StateElement
        let mut split: Vec<StateElement> = line
            .split(&SPLIT_CHARS)
            .filter_map(|word| {
                if word != "" {
                    Some(StateElement::Word(word.to_string()))
                } else {
                    None
                }
            })
            .collect();

        // We add the Start and End sentence markers here, plus placeholders to be able to easily parse the vector in the window below
        let mut elements: Vec<StateElement> = vec![
            StateElement::Marker(SentenceMarker::Placeholder),
            StateElement::Marker(SentenceMarker::Start),
        ];
        elements.append(&mut split);
        elements.push(StateElement::Marker(SentenceMarker::End));
        elements.push(StateElement::Marker(SentenceMarker::Placeholder));

        // We constuct states from ingestion_max_state_size to 1,
        // The more we loop, the bigger the brain will be
        for state_size in 1..=self.config.max_ingestion_state_size {
            if elements.len() <= state_size {
                continue;
            }

            elements.windows(state_size + 2).for_each(|window| {
                let prev_element = window.first().unwrap();
                let next_element = window.last().unwrap();

                let constructed_state = State(window[1..state_size + 1].to_vec());

                let transition =
                    self.state_transitions
                        .entry(constructed_state)
                        .or_insert(Transistion {
                            prev: vec![],
                            next: vec![],
                        });

                transition.increment_occurence(SentenceDirection::Backward, prev_element);
                transition.increment_occurence(SentenceDirection::Forward, next_element);
            });
        }
    }

    /// Save the current brain to disk.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// // ...
    ///
    /// brain.to_file("brain.bin");
    /// ```
    ///
    pub fn to_file(&self, output_path: impl AsRef<str>) -> Result<()> {
        let output_path = output_path.as_ref();
        println!("Saving brain...");

        let serialized = bincode::serialize(&self.state_transitions).unwrap();
        let mut output_file = File::create(output_path)?;

        output_file.write(&serialized)?;

        println!("Saved brain as {}", output_path);
        Ok(())
    }

    /// Load a brain from disk.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let dataset_path = "path/to/your/dataset.txt";
    ///
    /// let brain = Brain::from_file("path/to/brain.bin").unwrap()
    ///                 .get();
    ///
    /// brain.to_file("brain.bin");
    /// ```
    ///
    pub fn from_file(brain_path: impl AsRef<str>) -> Result<Self> {
        let brain_path = brain_path.as_ref();

        println!("Loading brain from {}...", brain_path);
        let mut save_file = File::open(brain_path)?;
        let mut buffer: Vec<u8> = vec![];

        save_file.read_to_end(&mut buffer)?;

        let state_transitions: HashMap<State, Transistion> = bincode::deserialize(&buffer).unwrap();

        Ok(Brain {
            state_transitions,
            ..Default::default()
        })
    }

    fn state_with_element_vec(&self, element: &StateElement) -> Vec<&State> {
        self.state_transitions
            .iter()
            .filter_map(|(state, _)| {
                if state.0.contains(element) {
                    Some(state)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Generate a reponse from an input.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let mut brain = Brain::new()
    ///     .from_dataset("your_dataset.txt")
    ///     .get();
    ///
    ///
    /// // `brain.generate` returns an option, as the reply_chance config might
    /// // be less than 1, or the mute config is set to true.
    /// if let Some(response) = brain.generate("Hello there!").unwrap() {
    ///     println!("{}", response);
    /// }
    /// ```
    ///
    pub fn generate(&mut self, input: impl AsRef<str>) -> Result<Option<String>> {
        self._generate(input, false)
    }

    /// Generate a reponse from an input.
    /// Except this time it will bypass
    /// any [`mute`] or [`reply_rate`] checks.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let mut brain = Brain::new()
    ///     .from_dataset("your_dataset.txt")
    ///     .get();
    ///
    ///
    /// // now it returns a String since it's guarenteed to have
    /// // a response.
    /// println!("{}", brain.generate_bypass_checks("Hello there!").unwrap());
    /// ```
    ///
    /// [`mute`]: crate::config::BrainConfig::mute
    /// [`reply_rate`]: crate::config::BrainConfig::reply_rate
    pub fn generate_bypass_checks(&mut self, input: impl AsRef<str>) -> Result<String> {
        // Safe to unwrap as it will always have a response.
        match self._generate(input, true) {
            Ok(response) => Ok(response.unwrap()),
            Err(e) => Err(e),
        }
    }

    fn _generate(&mut self, input: impl AsRef<str>, bypass_checks: bool) -> Result<Option<String>> {
        let input = input.as_ref();

        if self.state_transitions.len() == 0 {
            if self.config.training {
                self.ingest(input);
            }

            return Ok(None);
        }

        if self.config.mute && !bypass_checks {
            if self.config.training {
                self.ingest(input);
            }

            return Ok(None);
        }

        // using ! bool since the config is about reply chance, not reply non chance.
        if !self.rng.gen_bool(self.config.reply_rate) && !bypass_checks {
            if self.config.training {
                self.ingest(input);
            }

            return Ok(None);
        }

        let mut elements: Vec<&str> = input.trim_end().split(&SPLIT_CHARS).collect();

        let mut sentence: Vec<StateElement> = vec![];
        let mut original_element = None;

        elements.shuffle(&mut self.rng);

        let mut rng_clone = self.rng.clone();

        while let Some(word) = elements.pop() {
            let states = self.state_with_element_vec(&StateElement::Word(word.to_string()));
            let state = match states.choose(&mut rng_clone) {
                Some(state) => *state,
                None => continue,
            };

            original_element = Some(state.random_element(&mut rng_clone));
            break;
        }

        if let None = original_element {
            original_element = Some(
                self.state_transitions
                    .keys()
                    .choose(&mut self.rng)
                    .unwrap()
                    .random_element(&mut self.rng),
            );
        }

        sentence.push(original_element.unwrap().clone());

        while *sentence.first().unwrap() != StateElement::Marker(SentenceMarker::Start) {
            let prev_element = self.get_element(SentenceDirection::Backward, &sentence);
            sentence.insert(0, prev_element.clone());
        }

        while *sentence.last().unwrap() != StateElement::Marker(SentenceMarker::End) {
            let next_element = self.get_element(SentenceDirection::Forward, &sentence);
            sentence.push(next_element.clone());
        }

        if self.config.training {
            self.ingest(input);
        }

        Ok(Some(
            sentence
                .iter()
                .filter_map(|element| {
                    if let StateElement::Word(elem) = element {
                        Some(elem.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<&str>>()
                .join(" "),
        ))
    }

    /// Get a [`BrainStats`] reference for the current brain.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::Brain;
    ///
    /// let brain = Brain::load("path/to/brain.bin")
    ///                 .get();
    ///
    /// let stats = brain.stats();
    ///
    /// println!("{}", stats.get_total_states());
    /// ```
    ///
    /// [`BrainStats`]: crate::stats::BrainStats
    pub fn stats(&self) -> BrainStats {
        BrainStats::new(self)
    }

    fn get_element(
        &mut self,
        direction: SentenceDirection,
        sentence: &[StateElement],
    ) -> &StateElement {
        let mut transition = None;

        for state_size in self.config.get_state_range() {
            let min = state_size.min(sentence.len());
            match direction {
                SentenceDirection::Backward => {
                    transition = self
                        .state_transitions
                        .get(&State((&sentence[0..min]).to_vec()))
                }
                SentenceDirection::Forward => {
                    transition = self.state_transitions.get(&State(
                        (&sentence[sentence.len() - min..sentence.len()]).to_vec(),
                    ))
                }
            }

            if let None = transition {
                continue;
            }

            break;
        }

        if let None = transition {
            return match direction {
                SentenceDirection::Backward => &StateElement::Marker(SentenceMarker::Start),
                SentenceDirection::Forward => &StateElement::Marker(SentenceMarker::End),
            };
        }

        let attribute = match direction {
            SentenceDirection::Backward => &transition.unwrap().prev,
            SentenceDirection::Forward => &transition.unwrap().next,
        };

        &attribute
            .choose_weighted(&mut self.rng, |item| item.1)
            .unwrap()
            .0
    }
}
