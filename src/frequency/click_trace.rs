use crate::frequency::maths;
use crate::utils;

use indexmap::IndexSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FreqClickTrace {
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
}

#[derive(Debug, Clone)]
pub struct VectFreqClickTrace<T> {
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
}

// pub fn gen_typical_vect_click_trace(
//     click_traces: &Vec<FreqClickTrace>,
//     speed_set: &IndexSet<String>,
//     heading_set: &IndexSet<String>,
//     street_set: &IndexSet<String>,
//     postcode_set: &IndexSet<String>,
//     state_set: &IndexSet<String>,
// ) -> VectFreqClickTrace<f64> {
//     let mut speed_vec = maths::zeros_f64(speed_set.len());
//     let mut heading_vec = maths::zeros_f64(heading_set.len());
//     let mut street_vec = maths::zeros_f64(state_set.len());
//     let mut postcode_vec = maths::zeros_f64(postcode_set.len());
//     let mut state_vec = maths::zeros_f64(state_set.len());
//     let mut hour_vec = maths::zeros_f64(24);
//     let mut day_vec = maths::zeros_f64(7);

//     for click_trace in click_traces.into_iter() {
//         let vect_click_trace = vectorize_click_trace(
//             click_trace,
//             speed_set,
//             heading_set,
//             street_set,
//             postcode_set,
//             state_set,
//         );
//         speed_vec = maths::add(speed_vec, &vect_click_trace.speed);
//         heading_vec = maths::add(heading_vec, &vect_click_trace.heading);
//         street_vec = maths::add(street_vec, &vect_click_trace.street);
//         postcode_vec = maths::add(postcode_vec, &vect_click_trace.postcode);
//         state_vec = maths::add(state_vec, &vect_click_trace.state);
//         day_vec = maths::add(day_vec, &vect_click_trace.day);
//         hour_vec = maths::add(hour_vec, &vect_click_trace.hour);
//     }

//     let speed_len = speed_vec.len() as f64;
//     speed_vec.iter_mut().for_each(|a| *a /= speed_len);
//     let heading_len = heading_vec.len() as f64;
//     heading_vec.iter_mut().for_each(|a| *a /= heading_len);
//     let street_len = street_vec.len() as f64;
//     street_vec.iter_mut().for_each(|a| *a /= street_len);
//     let postcode_len = postcode_vec.len() as f64;
//     postcode_vec.iter_mut().for_each(|a| *a /= postcode_len);
//     let state_len = state_vec.len() as f64;
//     state_vec.iter_mut().for_each(|a| *a /= state_len);
//     let hour_len = hour_vec.len() as f64;
//     hour_vec.iter_mut().for_each(|a| *a /= hour_len);
//     let day_len = day_vec.len() as f64;
//     day_vec.iter_mut().for_each(|a| *a /= day_len);

//     let typical_vect_click_trace = VectFreqClickTrace {
//         speed: speed_vec,
//         heading: heading_vec,
//         street: street_vec,
//         postcode: postcode_vec,
//         state: state_vec,
//         day: day_vec,
//         hour: hour_vec,
//     };
//     typical_vect_click_trace
// }

// Transform each histogram (as a hashmap) in a click trace into a vector to speed up further computations
pub fn vectorize_click_trace(
    click_trace: &FreqClickTrace,
    speed_set: &IndexSet<String>,
    heading_set: &IndexSet<String>,
    street_set: &IndexSet<String>,
    postcode_set: &IndexSet<String>,
    state_set: &IndexSet<String>,
    highway_set: &IndexSet<String>,
    hamlet_set: &IndexSet<String>,
    suburb_set: &IndexSet<String>,
    village_set: &IndexSet<String>,
) -> VectFreqClickTrace<u32> {
    let vectorized_click_trace = VectFreqClickTrace {
        speed: utils::gen_vector_from_freq_map(&click_trace.speed, speed_set),
        heading: utils::gen_vector_from_freq_map(&click_trace.heading, heading_set),
        street: utils::gen_vector_from_freq_map(&click_trace.street, street_set),
        postcode: utils::gen_vector_from_freq_map(&click_trace.postcode, postcode_set),
        state: utils::gen_vector_from_freq_map(&click_trace.state, state_set),
        highway: utils::gen_vector_from_freq_map(&click_trace.highway, highway_set),
        hamlet: utils::gen_vector_from_freq_map(&click_trace.hamlet, hamlet_set),
        suburb: utils::gen_vector_from_freq_map(&click_trace.suburb, suburb_set),
        village: utils::gen_vector_from_freq_map(&click_trace.village, village_set),
        day: click_trace.hour.clone(),
        hour: click_trace.day.clone(),
    };
    vectorized_click_trace
}
