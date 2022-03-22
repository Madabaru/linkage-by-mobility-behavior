mod cli;
mod frequency;
mod parse;
mod sample;
mod sequence;
mod utils;

use frequency::mobility_trace::FreqMobilityTrace;
use sequence::mobility_trace::SeqMobilityTrace;
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
        let client_to_seq_map: BTreeMap<u32, Vec<SeqMobilityTrace>> =
            parse::parse_to_sequence(&config).unwrap();

        log::info!("Sampling clients...");
        let client_to_target_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_client_to_target_idx_map(&client_to_seq_map, &mut rng, config.client_sample_size, config.target_mobility_trace_sample_size);

        log::info!("Sampling mobility traces per client...");
        let client_to_sample_idx_map: HashMap<u32, Vec<usize>> = sample::gen_client_to_sample_idx_map(
            &client_to_seq_map,
            &mut rng,
            config.mobility_trace_sample_size,
        );

        // Optional
        // let serialized_client_to_test_idx_map = fs::read(&config.path_to_map).unwrap();
        // let client_to_test_idx_map: HashMap<u32, usize> =
        //     serde_pickle::from_slice(&serialized_client_to_test_idx_map, Default::default())
        //         .unwrap();

        log::info!("Sampling test mobility traces per client...");
        let client_to_test_idx_map: HashMap<u32, usize> = sample::gen_client_to_test_idx_map(
            &client_to_sample_idx_map,
            &mut rng,
        );
        

        log::info!("Starting the evaluation...");
        sequence::evaluation::eval(
            &config,
            &client_to_seq_map,
            &client_to_target_idx_map,
            &client_to_sample_idx_map,
            &client_to_test_idx_map,
        );

    // Approach 2: Frequency-based
    } else {
        log::info!("Parsing data for frequency-based approach...");
        let client_to_freq_map: BTreeMap<u32, Vec<FreqMobilityTrace>> =
            parse::parse_to_frequency(&config).unwrap();

        log::info!("Sampling clients...");
        let client_to_target_idx_map: HashMap<u32, Vec<usize>> =
            sample::gen_client_to_target_idx_map(&client_to_freq_map, &mut rng, config.client_sample_size, config.target_mobility_trace_sample_size);

        log::info!("Sampling mobility traces per client...");
        let client_to_sample_idx_map: HashMap<u32, Vec<usize>> = sample::gen_client_to_sample_idx_map(
            &client_to_freq_map,
            &mut rng,
            config.mobility_trace_sample_size,
        );
        
        log::info!("Sampling mobility click traces per client...");
        let client_to_test_idx_map: HashMap<u32, usize> = sample::gen_client_to_test_idx_map(
            &client_to_sample_idx_map,
            &mut rng,
        );

        log::info!("Starting the evaluation...");
        frequency::evaluation::eval(
            &config,
            &client_to_freq_map,
            &client_to_target_idx_map,
            &client_to_sample_idx_map,
            &client_to_test_idx_map,
        );
    }
}
