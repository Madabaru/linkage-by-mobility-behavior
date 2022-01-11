use crate::cli::Config;
use crate::frequency::{
    click_trace::FreqClickTrace, 
    maths
};
use crate::sequence::click_trace::SeqClickTrace;

use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    str::FromStr,
    string::ParseError,
    time::{Duration, UNIX_EPOCH},
    fmt::Display,
};
use chrono::{prelude::DateTime, Datelike, Timelike, Utc};
use indexmap::IndexSet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub client_id: String,
    pub timestamp: f64,
    pub heading: String,
    pub speed: String,
    pub street: String,
    pub postcode: String,
    pub state: String,
    pub highway: String,
    pub hamlet: String, 
    pub suburb: String, 
    pub village: String
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
    Village

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
            x => panic!("Error: Wrong data field supplied: {:?}", x),
        }
    }
}

pub fn parse_to_frequency(
    config: &Config,
) -> Result<BTreeMap<u32, Vec<FreqClickTrace>>, Box<dyn Error>> {
    
    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut click_trace_len: usize = 0;
    let mut client_id: u32 = 0;

    let mut client_to_freq_map: BTreeMap<u32, Vec<FreqClickTrace>> = BTreeMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.client_id && !prev_client.is_empty() {
            client_id += 1;
        }

        if !client_to_freq_map.contains_key(&client_id) {
            client_to_freq_map.insert(client_id, Vec::with_capacity(10));
        }

        let click_traces_list = client_to_freq_map.get_mut(&client_id).unwrap();

        if click_traces_list.is_empty()
            || click_trace_len >= config.max_click_trace_len
            || record.timestamp - prev_time >= config.delay_limit
        {
            if !click_traces_list.is_empty() {
                if click_trace_len < config.min_click_trace_len
                    || click_traces_list.last().unwrap().end_time
                        - click_traces_list.last().unwrap().start_time
                        > config.max_click_trace_duration
                {
                    click_traces_list.pop();
                } else if click_traces_list.last().unwrap().street.keys().len() <= 2 && (click_traces_list.last().unwrap().street.contains_key("A 5") || click_traces_list.last().unwrap().street.contains_key("A 67")) {
                    println!("Hi");
                    click_traces_list.pop();
                }
            }

            let click_trace = FreqClickTrace {
                speed: HashMap::new(),
                heading: HashMap::new(),
                street: HashMap::new(),
                postcode: HashMap::new(),
                highway: HashMap::new(),
                hamlet: HashMap::new(),
                suburb: HashMap::new(),
                village: HashMap::new(),
                state: HashMap::new(),
                hour: maths::zeros_u32(24),
                day: maths::zeros_u32(7),
                start_time: record.timestamp,
                end_time: record.timestamp,
            };
            click_traces_list.push(click_trace);
            click_trace_len = 0;
        }

        let current_click_trace = click_traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);

        // Convert from u32 to usize
        let hour_index: usize = usize::try_from(datetime.hour()).unwrap();
        let day_index: usize = usize::try_from(datetime.weekday().num_days_from_monday()).unwrap();

        current_click_trace.hour[hour_index] += 1;
        current_click_trace.day[day_index] += 1;
        current_click_trace.end_time = record.timestamp;

        *current_click_trace
            .speed
            .entry(record.speed.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .postcode
            .entry(record.postcode.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .heading
            .entry(record.heading.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .postcode
            .entry(record.postcode.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .state
            .entry(record.state.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .highway
            .entry(record.state.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .hamlet
            .entry(record.state.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .suburb
            .entry(record.state.clone())
            .or_insert(0) += 1;
        *current_click_trace
            .village
            .entry(record.state.clone())
            .or_insert(0) += 1;


        prev_time = record.timestamp;
        prev_client = record.client_id;
        click_trace_len += 1;
    }

    // Remove any client with less than the minimum number of click traces
    log::info!("Numer of clients before filtering: {:?}", client_to_freq_map.keys().len());
    client_to_freq_map.retain(|_, value| value.len() >= config.min_num_click_traces);
    log::info!("Number of clients after filtering: {:?}", client_to_freq_map.keys().len());
    Ok(client_to_freq_map)
}

pub fn parse_to_sequence(
    config: &Config,
) -> Result<BTreeMap<u32, Vec<SeqClickTrace>>, Box<dyn Error>> {
    let mut prev_time: f64 = 0.0;
    let mut prev_client = String::new();
    let mut click_trace_len: usize = 0;
    let mut client_id: u32 = 0;

    let mut client_to_seq_map: BTreeMap<u32, Vec<SeqClickTrace>> = BTreeMap::new();
    let mut reader = csv::Reader::from_path(&config.path)?;

    let mut street_set: IndexSet<String> = IndexSet::new();
    let mut postcode_set: IndexSet<String> = IndexSet::new();
    let mut state_set: IndexSet<String> = IndexSet::new();
    let mut speed_set: IndexSet<String> = IndexSet::new();
    let mut heading_set: IndexSet<String> = IndexSet::new();

    for result in reader.deserialize() {
        let record: Record = result?;

        if prev_client != record.client_id && !prev_client.is_empty() {
            client_id += 1;
        }

        if !client_to_seq_map.contains_key(&client_id) {
            client_to_seq_map.insert(client_id, Vec::with_capacity(10));
        }

        let click_traces_list = client_to_seq_map.get_mut(&client_id).unwrap();

        if click_traces_list.is_empty()
            || click_trace_len >= config.max_click_trace_len
            || record.timestamp - prev_time >= config.delay_limit
        {
            if !click_traces_list.is_empty() {
                if click_trace_len < config.min_click_trace_len
                    || click_traces_list.last().unwrap().end_time
                        - click_traces_list.last().unwrap().start_time
                        > config.max_click_trace_duration
                {
                    click_traces_list.pop();
                }
            }

            let click_trace = SeqClickTrace {
                street: Vec::with_capacity(10),
                hour: Vec::with_capacity(10),
                day: 0,
                start_time: record.timestamp,
                end_time: record.timestamp,
                speed: Vec::with_capacity(10),
                heading: Vec::with_capacity(10),
                postcode: Vec::with_capacity(10),
                state: Vec::with_capacity(10),
            };
            click_traces_list.push(click_trace);
            click_trace_len = 0;
        }

        let current_click_trace = click_traces_list.last_mut().unwrap();

        // Extract day and hour from unix timestamp
        let date = UNIX_EPOCH + Duration::from_secs_f64(record.timestamp.clone());
        let datetime = DateTime::<Utc>::from(date);

        street_set.insert(record.street.clone());
        postcode_set.insert(record.postcode.clone());
        state_set.insert(record.state.clone());
        heading_set.insert(record.heading.clone());
        speed_set.insert(record.speed.clone());

        current_click_trace.hour.push(datetime.hour());
        current_click_trace.day = datetime.weekday().num_days_from_monday();
        current_click_trace.end_time = record.timestamp;

        current_click_trace
            .street
            .push(u32::try_from(street_set.get_full(&record.street).unwrap().0).unwrap());
        current_click_trace
            .postcode
            .push(u32::try_from(postcode_set.get_full(&record.postcode).unwrap().0).unwrap());
        current_click_trace
            .state
            .push(u32::try_from(state_set.get_full(&record.state).unwrap().0).unwrap());
        current_click_trace
            .heading
            .push(u32::try_from(heading_set.get_full(&record.heading).unwrap().0).unwrap());
        current_click_trace
            .speed
            .push(u32::try_from(speed_set.get_full(&record.speed).unwrap().0).unwrap());

        prev_time = record.timestamp;
        prev_client = record.client_id;
        click_trace_len += 1;
    }

    // Remove any client with less than the minimum number of click traces
    log::info!("Number of clients before filtering: {:?}", client_to_seq_map.keys().len());
    client_to_seq_map.retain(|_, value| value.len() >= config.min_num_click_traces);
    log::info!("Number of clients after filtering: {:?}", client_to_seq_map.keys().len());
    Ok(client_to_seq_map)
}
