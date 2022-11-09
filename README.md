
# Rustkov

Rustkov is a Rust library aiming to build chatbots using a markov chain.

# Example

```rust
use rustkov::prelude::*;

fn main() -> Result<()> {
    // Create a new brain which contains the markov chain
    let mut brain = Brain::new()
        // train him with a dataset
        .from_dataset("path/to/your/dataset.txt")?
        .get();

    // As we didn't specify a config file to the brain,
    // we need to adjust config options here.
    // For instance, let's make it so it can learn from inputs.
    brain.config.training = true;

    // `brain.generate` returns an option, as the reply_chance config might
    // be less than 1.
    if let Some(response) = brain.generate("Hello there!")? {
        println!("{}", response);
    }

    // Get a reference to a BrainStats struct, computing statistics for a given brain
    let stats = brain.stats();

    println!("I know {} words!", stats.get_total_words());

    Ok(())
}
```



# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rustkov = "0.1.0"
```