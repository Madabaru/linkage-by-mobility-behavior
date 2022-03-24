use crate::cli::Config;
use crate::frequency::{maths, trace::FreqTrace};
use crate::sequence::trace::SeqTrace;

use chrono::{prelude::DateTime, Datelike, Timelike, Utc};
use indexmap::IndexSet;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    fmt::Display,
    str::FromStr,
    string::ParseError,
    time::{Duration, UNIX_EPOCH},
};

#[derive(Debug, Deserialize)]
pub struct Record {
    pub user_id: String,
    pub timestamp: f64,
    pub heading: String,
    pub speed: String,
    pub street: String,
    pub postcode: String,
    pub state: String,
    pub highway: String,
    pub hamlet: String,
    pub suburb: String,
    pub village: String,
    pub location_code: String,
}

#[derive(PartialEq, Debug)]
pub enum DataFields {
    Speed,
    Day,
    Hour,
    Heading,
    Street,
    Postcode,
    State,
    Highway,
    Hamlet,
    Suburb,
    Village,
    LocationCode,
}

impl Display for DataFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for DataFields {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "speed" => Ok(Self::Speed),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "heading" => Ok(Self::Heading),
            "street" => Ok(Self::Street),
            "postcode" => Ok(Self::Postcode),
            "state" => Ok(Self::State),
            "highway" => Ok(Self::Highway),
            "hamlet" => Ok(Self::Hamlet),
            "suburb" => Ok(Self::Suburb),
            "village" => Ok(Self::Village),
            "location_code" => Ok(Self::LocationCode),
            x => panic!("Error: Wrong data field supplied: {:?}", x),
        }
    }
}

/// Parses the raw data into a convenient tree map for the histogram-based approach.
pub fn parse_to_frequency(
    config: &Config,
) -> Result<BTreeMap<u32, Vec<FreqTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut trace_len: usize = 0;
    let mut user_id: u32 = 0;

    let mut user_to_freq_map: BTreeMap<u32, Vec<FreqTrace>> = BTreeMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.user_id && !prev_client.is_empty() {
            // Check last mobility trace added to previous client
            let prev_traces_list = user_to_freq_map.get_mut(&user_id).unwrap();
            if !prev_traces_list.is_empty() {
                if trace_len < config.min_trace_len {
                    prev_traces_list.pop();
                }
            }
            user_id += 1;
        }

        if !user_to_freq_map.contains_key(&user_id) {
            user_to_freq_map.insert(user_id, Vec::with_capacity(10));
        }

        let traces_list = user_to_freq_map.get_mut(&user_id).unwrap();

        if traces_list.is_empty()
            || trace_len >= config.max_trace_len
            || record.timestamp - prev_time >= config.delay_limit
        {
            if !traces_list.is_empty() {
                if trace_len < config.min_trace_len
                    || traces_list.last().unwrap().end_time
                        - traces_list.last().unwrap().start_time
                        > config.max_trace_duration
                {
                    traces_list.pop();
                }
            }

            let trace = FreqTrace {
                speed: HashMap::new(),
                heading: HashMap::new(),
                street: HashMap::new(),
                postcode: HashMap::new(),
                highway: HashMap::new(),
                hamlet: HashMap::new(),
                suburb: HashMap::new(),
                village: HashMap::new(),
                state: HashMap::new(),
                location_code: HashMap::new(),
                hour: maths::zeros_u32(24),
                day: maths::zeros_u32(7),
                start_time: record.timestamp,
                end_time: record.timestamp,
            };
            traces_list.push(trace);
            trace_len = 0;
        }

        let current_trace = traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);

        // Convert from u32 to usize
        let hour_index: usize = usize::try_from(datetime.hour()).unwrap();
        let day_index: usize = usize::try_from(datetime.weekday().num_days_from_monday()).unwrap();

        current_trace.hour[hour_index] += 1;
        current_trace.day[day_index] += 1;
        current_trace.end_time = record.timestamp;

        *current_trace
            .speed
            .entry(record.speed.clone())
            .or_insert(0) += 1;
        *current_trace
            .postcode
            .entry(record.postcode.clone())
            .or_insert(0) += 1;
        *current_trace
            .heading
            .entry(record.heading.clone())
            .or_insert(0) += 1;
        *current_trace
            .street
            .entry(record.street.clone())
            .or_insert(0) += 1;
        *current_trace
            .state
            .entry(record.state.clone())
            .or_insert(0) += 1;
        *current_trace
            .highway
            .entry(record.highway.clone())
            .or_insert(0) += 1;
        *current_trace
            .hamlet
            .entry(record.hamlet.clone())
            .or_insert(0) += 1;
        *current_trace
            .suburb
            .entry(record.suburb.clone())
            .or_insert(0) += 1;
        *current_trace
            .village
            .entry(record.village.clone())
            .or_insert(0) += 1;
        *current_trace
            .location_code
            .entry(record.location_code.clone())
            .or_insert(0) += 1;

        prev_time = record.timestamp;
        prev_client = record.user_id;
        trace_len += 1;
    }

    // Remove any client with less than the minimum number of click traces
    log::info!(
        "Numer of users before filtering: {:?}",
        user_to_freq_map.keys().len()
    );
    user_to_freq_map.retain(|_, value| value.len() >= config.min_num_traces);
    log::info!(
        "Number of users after filtering: {:?}",
        user_to_freq_map.keys().len()
    );

    let total_num_traces: usize =
        user_to_freq_map.iter().map(|(_, val)| val.len()).sum();
    log::info!(
        "Total number of mobility traces: {:?}",
        total_num_traces
    );
    Ok(user_to_freq_map)
}

/// Parses the raw data into a convenient tree map for the sequence aligment-based approach.
pub fn parse_to_sequence(
    config: &Config,
) -> Result<BTreeMap<u32, Vec<SeqTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut trace_len: usize = 0;
    let mut user_id: u32 = 0;

    let mut user_to_seq_map: BTreeMap<u32, Vec<SeqTrace>> = BTreeMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    let mut street_set: IndexSet<String> = IndexSet::new();
    let mut postcode_set: IndexSet<String> = IndexSet::new();
    let mut state_set: IndexSet<String> = IndexSet::new();
    let mut speed_set: IndexSet<String> = IndexSet::new();
    let mut heading_set: IndexSet<String> = IndexSet::new();
    let mut highway_set: IndexSet<String> = IndexSet::new();
    let mut hamlet_set: IndexSet<String> = IndexSet::new();
    let mut suburb_set: IndexSet<String> = IndexSet::new();
    let mut village_set: IndexSet<String> = IndexSet::new();
    let mut location_code_set: IndexSet<String> = IndexSet::new();

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.user_id && !prev_client.is_empty() {
            let prev_traces_list = user_to_seq_map.get_mut(&user_id).unwrap();
            if !prev_traces_list.is_empty() {
                if trace_len < config.min_trace_len {
                    prev_traces_list.pop();
                }
            }
            user_id += 1;
        }

        if !user_to_seq_map.contains_key(&user_id) {
            user_to_seq_map.insert(user_id, Vec::with_capacity(10));
        }

        let traces_list = user_to_seq_map.get_mut(&user_id).unwrap();

        if traces_list.is_empty()
            || trace_len >= config.max_trace_len
            || record.timestamp - prev_time >= config.delay_limit
        {
            if !traces_list.is_empty() {
                if trace_len < config.min_trace_len
                    || traces_list.last().unwrap().end_time
                        - traces_list.last().unwrap().start_time
                        > config.max_trace_duration
                {
                    traces_list.pop();
                }
            }

            let trace = SeqTrace {
                street: Vec::with_capacity(10),
                hour: Vec::with_capacity(10),
                day: 0,
                start_time: record.timestamp,
                end_time: record.timestamp,
                speed: Vec::with_capacity(10),
                heading: Vec::with_capacity(10),
                postcode: Vec::with_capacity(10),
                state: Vec::with_capacity(10),
                highway: Vec::with_capacity(10),
                hamlet: Vec::with_capacity(10),
                suburb: Vec::with_capacity(10),
                village: Vec::with_capacity(10),
                location_code: Vec::with_capacity(10),
            };
            traces_list.push(trace);
            trace_len = 0;
        }

        let current_trace = traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);

        street_set.insert(record.street.clone());
        postcode_set.insert(record.postcode.clone());
        state_set.insert(record.state.clone());
        heading_set.insert(record.heading.clone());
        speed_set.insert(record.speed.clone());
        highway_set.insert(record.highway.clone());
        hamlet_set.insert(record.hamlet.clone());
        suburb_set.insert(record.suburb.clone());
        village_set.insert(record.village.clone());
        location_code_set.insert(record.location_code.clone());

        current_trace.hour.push(datetime.hour());
        current_trace.day = datetime.weekday().num_days_from_monday();
        current_trace.end_time = record.timestamp;

        current_trace
            .street
            .push(u32::try_from(street_set.get_full(&record.street).unwrap().0).unwrap());
        current_trace
            .postcode
            .push(u32::try_from(postcode_set.get_full(&record.postcode).unwrap().0).unwrap());
        current_trace
            .state
            .push(u32::try_from(state_set.get_full(&record.state).unwrap().0).unwrap());
        current_trace
            .heading
            .push(u32::try_from(heading_set.get_full(&record.heading).unwrap().0).unwrap());
        current_trace
            .speed
            .push(u32::try_from(speed_set.get_full(&record.speed).unwrap().0).unwrap());
        current_trace
            .highway
            .push(u32::try_from(highway_set.get_full(&record.highway).unwrap().0).unwrap());
        current_trace
            .hamlet
            .push(u32::try_from(hamlet_set.get_full(&record.hamlet).unwrap().0).unwrap());
        current_trace
            .suburb
            .push(u32::try_from(suburb_set.get_full(&record.suburb).unwrap().0).unwrap());
        current_trace
            .village
            .push(u32::try_from(village_set.get_full(&record.village).unwrap().0).unwrap());
        current_trace.location_code.push(
            u32::try_from(location_code_set.get_full(&record.location_code).unwrap().0).unwrap(),
        );

        prev_time = record.timestamp;
        prev_client = record.user_id;
        trace_len += 1;
    }

    // Remove any client with less than the minimum number of click traces
    log::info!(
        "Number of users before filtering: {:?}",
        user_to_seq_map.keys().len()
    );
    user_to_seq_map.retain(|_, value| value.len() >= config.min_num_traces);
    log::info!(
        "Number of users after filtering: {:?}",
        user_to_seq_map.keys().len()
    );

    let total_num_traces: usize = user_to_seq_map.iter().map(|(_, val)| val.len()).sum();
    log::info!(
        "Total number of traces: {:?}",
        total_num_traces
    );
    Ok(user_to_seq_map)
}
