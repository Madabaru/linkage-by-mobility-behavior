use crate::{parse::DataFields};

use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub delay_limit: f64,
    pub fields: Vec<DataFields>,
    pub max_trace_len: usize,
    pub min_trace_len: usize,
    pub max_trace_duration: f64,
    pub min_num_traces: usize,
    pub user_sample_size: usize,
    pub trace_sample_size: usize,
    pub target_trace_sample_size: usize,
    pub metric: String,
    pub path: String,
    pub seed: u64,
    pub typical: bool,
    pub dependent: bool, 
    pub strategy: String,
    pub scoring_matrix: Vec<isize>,
    pub approach: String,
    pub scope: String,
    pub reverse: bool,
}


pub fn get_cli_config() -> Result<Config, clap::Error> {
    let matches = clap::App::new("Linking-by-Mobility-Behavior")
        .version("1.0")
        .author("Felix John")
        .arg(
            clap::Arg::new("approach")
                .long("approach")
                .about("Sequence alignment-based or frequency-based approach.")
                .possible_values(&["sequence", "frequency"])
                .default_value("frequency"),
        )
        .arg(
            clap::Arg::new("scoring_matrix")
                .long("scoring_matrix")
                .allow_hyphen_values(true)
                .about("The scoring matrix to use for the alignment approach: ['equal', 'align', 'insert', 'delete'].")
                .multiple_values(true)
                .default_values(&["1", "-1", "-1", "-1"])
        )
        .arg(
            clap::Arg::new("scope")
                .long("scope")
                .about("The scope of the alignment algorithm: local or global.")
                .possible_values(&["local", "global"])
                .default_value("global"),
        )
        .arg(
            clap::Arg::new("strategy")
                .long("strategy")
                .about("The alignment strategy to use.")
                .possible_values(&["sw", "nw"])
                .default_value("nw"),
        )
        .arg(
            clap::Arg::new("delay_limit")
                .long("delay_limit")
                .about("Maximum delay between two consecutive s.")
                .default_value("1800.0"),
        )
        .arg(
            clap::Arg::new("fields")
                .long("fields")
                .possible_values(&["speed", "heading", "street", "postcode", "state", "hour", "day", "highway", "hamlet", "suburb", "village", "location_code"])
                .about("Data fields to consider for the analysis.")
                .multiple_values(true)
                .default_values(&["speed", "street", "postcode", "suburb", "village"])
        )
        .arg(
            clap::Arg::new("max_trace_len")
                .long("max_trace_len")
                .default_value("500")
                .about("Maximum length of a single trace."),
        )
        .arg(
            clap::Arg::new("min_trace_len")
                .long("min_trace_len")
                .default_value("10")
                .about("Minimum length of a single trace."),
        )
        .arg(
            clap::Arg::new("max_trace_duration")
                .long("max_trace_duration")
                .default_value("86400.0")
                .about("Maximum duration of a single trace."),
        )
        .arg(
            clap::Arg::new("min_num_traces")
                .long("min_num_traces")
                .default_value("4")
                .about("Minimum number of traces per user."),
        )
        .arg(
            clap::Arg::new("user_sample_size")
                .long("user_sample_size")
                .default_value("400")
                .about("Number of clients to sample."),
        )
        .arg(
            clap::Arg::new("trace_sample_size")
                .long("trace_sample_size")
                .default_value("500")
                .about("Number of traces to sample per user"),
        )
        .arg(
            clap::Arg::new("target_trace_sample_size")
                .long("target_trace_sample_size")
                .default_value("1")
                .about("Number of target traces per user."),
        )
        .arg(
            clap::Arg::new("metric")
                .long("metric")
                .default_value("euclidean")
                .about("Distance metric to compare a pair of  traces.")
                .possible_values(&["euclidean", "manhattan", "cosine", "non_intersection", "bhattacharyya", "kullbrack_leibler", "total_variation", "jeffries_matusita", "chi_quared"]),
        )
        .arg(
            clap::Arg::new("path")
                .long("path")
                .default_value("/home/john/data/mobility/driving_sampled_8k.csv")
                .about("Path to the dataset.")
        )
        .arg(
            clap::Arg::new("seed")
                .long("seed")
                .default_value("0")
                .about("Random seed for reproducability.")
        )
        .arg(
            clap::Arg::new("typical")
                .long("typical")
                .default_value("false")
                .about("Set to true if you want to compute a typical trace (session) per user.")
        )
        .arg(
            clap::Arg::new("dependent")
                .long("dependent")
                .default_value("false")
                .about("Set true of the linkage attacks are dependent on each other.")
        )
        .arg(
            clap::Arg::new("reverse")
                .long("reverse")
                .default_value("false")
                .about("Set to true if you want to reverse the sequence for pairwise comparison.")
        )
        .get_matches();

    let config = Config {
        delay_limit: matches
            .value_of("delay_limit")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap(),
        metric: matches
            .value_of("metric")
            .map(String::from)
            .unwrap_or_default(),
        max_trace_len: matches
            .value_of("max_trace_len")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap(),
        fields: matches
            .values_of_lossy("fields")
            .unwrap_or_default()
            .iter()
            .map(|x| DataFields::from_str(x).unwrap())
            .collect(),
        trace_sample_size: matches
            .value_of("trace_sample_size")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap(),
        user_sample_size: matches
            .value_of("user_sample_size")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap(),
        target_trace_sample_size: matches
            .value_of("target_trace_sample_size")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap(),
        max_trace_duration: matches
            .value_of("max_trace_duration")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap(),
        min_trace_len: matches
            .value_of("min_trace_len")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap(),
        min_num_traces: matches
            .value_of("min_num_traces")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap(),
        path: matches
            .value_of("path")
            .map(String::from)
            .unwrap_or_default(),
        seed: matches
            .value_of("seed")
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap(),
        typical: matches
            .value_of("typical")
            .unwrap_or_default()
            .parse::<bool>()
            .unwrap(),
        dependent: matches
            .value_of("dependent")
            .unwrap_or_default()
            .parse::<bool>()
            .unwrap(),
        reverse: matches
            .value_of("reverse")
            .unwrap_or_default()
            .parse::<bool>()
            .unwrap(),
        strategy: matches
            .value_of("strategy")
            .map(String::from)
            .unwrap_or_default(),
        scoring_matrix: matches
            .values_of_lossy("scoring_matrix")
            .unwrap_or_default()
            .iter()
            .map(|x| isize::from_str(x).unwrap())
            .collect(),
        approach: matches
            .value_of("approach")
            .map(String::from)
            .unwrap_or_default(),
        scope: matches
            .value_of("scope")
            .map(String::from)
            .unwrap_or_default(),
    };
    Ok(config)
}
