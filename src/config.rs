use crate::brain_prelude::*;

pub const SPLIT_CHARS: [char; 2] = [' ', '\n'];

/// This struct let you configure a [`Brain`].
///
/// It can be used to edit configurations at runetime as well.
///
/// # Example
///
/// ```
/// use rustkov::prelude::BrainConfig;
///
/// let config = BrainConfig {
///     training: true,
///     reply_rate: 0.33,
///     ..Default::default()
/// };
/// // Or:
/// let config = BrainConfig::from_file("path/to/config.yml").unwrap();
///
/// ```
///
/// [`Brain`]: crate::brain::Brain
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrainConfig {
    /// Represents the max items we use as a single state
    /// when ingesting an input.
    ///
    /// The greater it is, the bigger the brain struct will be.
    pub max_ingestion_state_size: usize,

    /// Allow the brain to learn from inputs.
    pub training: bool,

    /// Disallow the brain to output any response.
    pub mute: bool,

    /// The rate at which the brain will reply to inputs.
    ///
    /// It should be `0 <= reply_rate <= 1`.
    pub reply_rate: f64,

    /// This setting will be used when constructing a sentence.
    ///
    /// The brain will only take states with a length greater than
    /// this setting.
    pub min_generation_state_size: usize,

    /// This setting will be used when constructing a sentence.
    ///
    /// The brain will only take states with a length smaller than
    /// this setting.
    pub max_generation_state_size: usize,

    /// Let you ban forbidden words from appearing in responses.
    pub excluded_words: Vec<String>,
}
impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            max_ingestion_state_size: 5,
            training: false,
            mute: false,
            reply_rate: 1f64,
            min_generation_state_size: 2,
            max_generation_state_size: 4,
            excluded_words: vec![],
        }
    }
}
impl BrainConfig {
    pub(crate) fn get_state_range(&self) -> Range<usize> {
        Range {
            start: self.min_generation_state_size,
            end: self.max_generation_state_size,
        }
    }

    /// Load a config from disk.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::BrainConfig;
    ///
    /// let config = BrainConfig::from_file("path/to/config.yml").unwrap();
    ///
    /// ```
    pub fn from_file(config_path: impl AsRef<str>) -> Result<Self> {
        let mut config_file = File::open(config_path.as_ref())?;
        let mut buffer = String::new();

        config_file.read_to_string(&mut buffer)?;

        let config: BrainConfig = serde_yaml::from_str(&buffer).unwrap();

        Ok(config)
    }

    /// Save a config to disk.
    ///
    /// # Example
    ///
    /// ```
    /// use rustkov::prelude::BrainConfig;
    ///
    /// config.to_file("path/to/config.yml").unwrap();
    ///
    /// ```
    pub fn to_file(&self, config_path: &str) -> Result<()> {
        let mut config_file = OpenOptions::new().write(true).open(config_path)?;

        let data = serde_yaml::to_string(&self).unwrap();

        config_file.write(data.as_bytes())?;

        Ok(())
    }
}
