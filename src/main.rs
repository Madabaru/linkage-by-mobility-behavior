mod cli;
mod frequency;
mod parse;
mod sample;
mod sequence;
mod utils;

use frequency::trace::FreqTrace;
use sequence::trace::SeqTrace;
use simple_logger::SimpleLogger;

use rand::{rngs::StdRng, SeedableRng};
use std::collections::{BTreeMap, HashMap};

fn main() {
    // Load config
    let config = cli::get_cli_config().unwrap();

    // Set up logger
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    // Set random seed for reproducability
    let mut rng = StdRng::seed_from_u64(config.seed);

    // Approach 1: Sequence alignment-based
    if config.approach == "sequence" {
        log::info!("Parsing data for sequence alignment-based approach...");
        let user_to_seq_map: BTreeMap<u32, Vec<SeqTrace>> =
            parse::parse_to_sequence(&config).unwrap();

        log::info!("Sampling users...");
        let user_to_target_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_target_idx_map(
                &user_to_seq_map,
                &mut rng,
                config.user_sample_size,
                config.target_trace_sample_size,
            );

        log::info!("Sampling traces per user...");
        let user_to_sample_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_sample_idx_map(
                &user_to_seq_map,
                &mut rng,
                config.trace_sample_size,
            );

        // Optional
        // let serialized_user_to_test_idx_map = fs::read(&config.path_to_map).unwrap();
        // let user_to_test_idx_map: HashMap<u32, usize> =
        //     serde_pickle::from_slice(&serialized_user_to_test_idx_map, Default::default())
        //         .unwrap();

        log::info!("Sampling a single test traces per user...");
        let user_to_test_idx_map: HashMap<u32, usize> =
            sample::gen_user_to_test_idx_map(&user_to_sample_idx_map, &mut rng);

        log::info!("Starting the evaluation...");
        sequence::evaluation::eval(
            &config,
            &user_to_seq_map,
            &user_to_target_idx_map,
            &user_to_sample_idx_map,
            &user_to_test_idx_map,
        );

    // Approach 2: Frequency-based
    } else {
        log::info!("Parsing data for frequency-based approach...");
        let user_to_freq_map: BTreeMap<u32, Vec<FreqTrace>> =
            parse::parse_to_frequency(&config).unwrap();

        log::info!("Sampling users...");
        let user_to_target_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_target_idx_map(
                &user_to_freq_map,
                &mut rng,
                config.user_sample_size,
                config.target_trace_sample_size,
            );

        log::info!("Sampling traces per user...");
        let user_to_sample_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_user_to_sample_idx_map(
                &user_to_freq_map,
                &mut rng,
                config.trace_sample_size,
            );

        log::info!("Sampling a single test trace per user...");
        let user_to_test_idx_map: HashMap<u32, usize> =
            sample::gen_user_to_test_idx_map(&user_to_sample_idx_map, &mut rng);

        log::info!("Starting the evaluation...");
        frequency::evaluation::eval(
            &config,
            &user_to_freq_map,
            &user_to_target_idx_map,
            &user_to_sample_idx_map,
            &user_to_test_idx_map,
        );
    }
}
