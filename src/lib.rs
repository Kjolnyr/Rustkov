//! Rustkov is a Rust library aiming to build chatbots using a markov chain.
//! The main struct, called [`Brain`], is in charge of interacting with the markov chain.
//! All you need to do it feed it data, and it will spit out something related to it.
//!
//! There's also a struct called [`BrainStats`] to have statistics about the brain data.
//! You can get a [`BrainStats`] reference using [`Brain::stats`].
//!
//! # Examples
//!
//! ```
//! use rustkov::prelude::*;
//!
//! fn main() -> Result<()> {
//!     
//!     // The brain is created using composition.
//!     let mut brain = Brain::new()
//!         .from_dataset("your_dataset.txt")?
//!         .get();
//!
//!     // As we didn't specify a config file to the brain,
//!     // we need to adjust config options here.
//!     // For instance, let's make it so it can learn from inputs.
//!     brain.config.training = true;
//!
//!     // `brain.generate` returns an option, as the reply_chance config might
//!     // be less than 1.
//!     if let Some(response) = brain.generate("Hello there!")? {
//!         println!("{}", response);
//!     }
//! }
//!
//! ```
//!
//!
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```
//! [dependencies]
//! rustkov = "0.1.0"
//! ```
//!
//! [`Brain`]: crate::brain::Brain
//! [`BrainStats`]: crate::stats::BrainStats
//! [`Brain::stats`]: crate::brain::Brain::stats

mod brain;
mod brain_components;
mod config;
mod enums;
mod error;
mod stats;

mod brain_prelude {
    pub use std::{
        collections::HashMap,
        fs::{File, OpenOptions},
        io::{BufRead, BufReader, Read, Write},
        ops::Range,
    };

    pub use rand::prelude::{IteratorRandom, RngCore, SliceRandom};
    pub use rand_chacha::ChaCha8Rng;
    pub use serde::{Deserialize, Serialize};

    pub use crate::brain_components::*;
    pub use crate::config::*;
    pub use crate::enums::*;
    pub use crate::error::*;
    pub use crate::stats::*;
    pub use crate::stats::*;
}

pub mod prelude {
    pub use crate::brain::Brain;
    pub use crate::config::BrainConfig;
    pub use crate::error::{Error, Result};
    pub use crate::stats::BrainStats;
}
