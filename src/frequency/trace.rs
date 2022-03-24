use crate::frequency::maths;
use crate::utils;

use indexmap::IndexSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FreqTrace {
    pub speed: HashMap<String, u32>,
    pub heading: HashMap<String, u32>,
    pub street: HashMap<String, u32>,
    pub postcode: HashMap<String, u32>,
    pub state: HashMap<String, u32>,
    pub highway: HashMap<String, u32>,
    pub hamlet: HashMap<String, u32>,
    pub suburb: HashMap<String, u32>,
    pub village: HashMap<String, u32>,
    pub day: Vec<u32>,
    pub hour: Vec<u32>,
    pub start_time: f64,
    pub end_time: f64,
    pub location_code: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct VectFreqTrace<T> {
    pub speed: Vec<T>,
    pub heading: Vec<T>,
    pub street: Vec<T>,
    pub postcode: Vec<T>,
    pub state: Vec<T>,
    pub highway: Vec<T>,
    pub hamlet: Vec<T>,
    pub suburb: Vec<T>,
    pub village: Vec<T>,
    pub hour: Vec<T>,
    pub day: Vec<T>,
    pub location_code: Vec<T>,
}

/// Generates a typical (vectorized) trace from a given list of traces.
///
/// The distribution of values for each data field is determined by taking the average.
pub fn gen_typical_vect_trace(
    traces: &Vec<FreqTrace>,
    speed_set: &IndexSet<String>,
    heading_set: &IndexSet<String>,
    street_set: &IndexSet<String>,
    postcode_set: &IndexSet<String>,
    state_set: &IndexSet<String>,
    highway_set: &IndexSet<String>,
    hamlet_set: &IndexSet<String>,
    suburb_set: &IndexSet<String>,
    village_set: &IndexSet<String>,
    location_code_set: &IndexSet<String>,
) -> VectFreqTrace<f64> {
    let mut speed_vec = maths::zeros_f64(speed_set.len());
    let mut heading_vec = maths::zeros_f64(heading_set.len());
    let mut street_vec = maths::zeros_f64(street_set.len());
    let mut postcode_vec = maths::zeros_f64(postcode_set.len());
    let mut state_vec = maths::zeros_f64(state_set.len());
    let mut highway_vec = maths::zeros_f64(highway_set.len());
    let mut hamlet_vec = maths::zeros_f64(hamlet_set.len());
    let mut suburb_vec = maths::zeros_f64(suburb_set.len());
    let mut village_vec = maths::zeros_f64(village_set.len());
    let mut location_code_vec = maths::zeros_f64(location_code_set.len());
    let mut hour_vec = maths::zeros_f64(24);
    let mut day_vec = maths::zeros_f64(7);

    for trace in traces.into_iter() {
        let vect_trace = vectorize_trace(
            trace,
            speed_set,
            heading_set,
            street_set,
            postcode_set,
            state_set,
            highway_set,
            hamlet_set,
            suburb_set,
            village_set,
            location_code_set,
        );
        speed_vec = maths::add(speed_vec, &vect_trace.speed);
        heading_vec = maths::add(heading_vec, &vect_trace.heading);
        street_vec = maths::add(street_vec, &vect_trace.street);
        postcode_vec = maths::add(postcode_vec, &vect_trace.postcode);
        state_vec = maths::add(state_vec, &vect_trace.state);
        highway_vec = maths::add(highway_vec, &vect_trace.highway);
        hamlet_vec = maths::add(hamlet_vec, &vect_trace.hamlet);
        suburb_vec = maths::add(suburb_vec, &vect_trace.suburb);
        village_vec = maths::add(village_vec, &vect_trace.village);
        location_code_vec = maths::add(location_code_vec, &vect_trace.location_code);
        day_vec = maths::add(day_vec, &vect_trace.day);
        hour_vec = maths::add(hour_vec, &vect_trace.hour);
    }

    let speed_len = speed_vec.len() as f64;
    speed_vec.iter_mut().for_each(|a| *a /= speed_len);
    let heading_len = heading_vec.len() as f64;
    heading_vec.iter_mut().for_each(|a| *a /= heading_len);
    let street_len = street_vec.len() as f64;
    street_vec.iter_mut().for_each(|a| *a /= street_len);
    let postcode_len = postcode_vec.len() as f64;
    postcode_vec.iter_mut().for_each(|a| *a /= postcode_len);
    let state_len = state_vec.len() as f64;
    state_vec.iter_mut().for_each(|a| *a /= state_len);
    let highway_len = highway_vec.len() as f64;
    highway_vec.iter_mut().for_each(|a| *a /= highway_len);
    let hamlet_len = hamlet_vec.len() as f64;
    hamlet_vec.iter_mut().for_each(|a| *a /= hamlet_len);
    let suburb_len = suburb_vec.len() as f64;
    suburb_vec.iter_mut().for_each(|a| *a /= suburb_len);
    let village_len = village_vec.len() as f64;
    village_vec.iter_mut().for_each(|a| *a /= village_len);
    let location_code_len = location_code_vec.len() as f64;
    location_code_vec
        .iter_mut()
        .for_each(|a| *a /= location_code_len);
    let hour_len = hour_vec.len() as f64;
    hour_vec.iter_mut().for_each(|a| *a /= hour_len);
    let day_len = day_vec.len() as f64;
    day_vec.iter_mut().for_each(|a| *a /= day_len);

    let typical_vect_trace = VectFreqTrace {
        speed: speed_vec,
        heading: heading_vec,
        street: street_vec,
        postcode: postcode_vec,
        state: state_vec,
        day: day_vec,
        hour: hour_vec,
        highway: highway_vec,
        hamlet: hamlet_vec,
        suburb: suburb_vec,
        village: village_vec,
        location_code: location_code_vec,
    };
    typical_vect_trace
}

/// Transforms each histogram (stored in a hash map) that corresponds to a trace into a fixed-size vector.
///
/// This tranformation to a fixed size vector greatly improves performance during the evaluation phase.
pub fn vectorize_trace(
    trace: &FreqTrace,
    speed_set: &IndexSet<String>,
    heading_set: &IndexSet<String>,
    street_set: &IndexSet<String>,
    postcode_set: &IndexSet<String>,
    state_set: &IndexSet<String>,
    highway_set: &IndexSet<String>,
    hamlet_set: &IndexSet<String>,
    suburb_set: &IndexSet<String>,
    village_set: &IndexSet<String>,
    location_code_set: &IndexSet<String>,
) -> VectFreqTrace<u32> {
    let vectorized_trace = VectFreqTrace {
        speed: utils::gen_vector_from_freq_map(&trace.speed, speed_set),
        heading: utils::gen_vector_from_freq_map(&trace.heading, heading_set),
        street: utils::gen_vector_from_freq_map(&trace.street, street_set),
        postcode: utils::gen_vector_from_freq_map(&trace.postcode, postcode_set),
        state: utils::gen_vector_from_freq_map(&trace.state, state_set),
        highway: utils::gen_vector_from_freq_map(&trace.highway, highway_set),
        hamlet: utils::gen_vector_from_freq_map(&trace.hamlet, hamlet_set),
        suburb: utils::gen_vector_from_freq_map(&trace.suburb, suburb_set),
        village: utils::gen_vector_from_freq_map(&trace.village, village_set),
        location_code: utils::gen_vector_from_freq_map(&trace.location_code, location_code_set),
        day: trace.hour.clone(),
        hour: trace.day.clone(),
    };
    vectorized_trace
}
