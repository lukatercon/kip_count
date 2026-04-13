use std::env;

mod config;
mod gram_counter;

use crate::config::Config;
use crate::gram_counter::GramCounter;

fn main() {
    // CLI usage: cargo run k n input_dir output_file
    let args: Vec<String> = env::args().collect();

    // parse arguments into a config struct and check if everything looks ok
    let count_config: Config = Config::parse_args(&args);
    count_config.check_validity();

    // construct the counter object
    let mut skipgram_counter: GramCounter = GramCounter::new(count_config.get_k(), count_config.get_n());

    // count from the specified directory
    skipgram_counter.count_from_directory(count_config.get_input_dir());

    println!("Counting done! Starting sorting phase");

    // write the sorted data to file
    GramCounter::write_to_file(skipgram_counter.get_sorted_skipgrams(), count_config.get_output_file());

}
