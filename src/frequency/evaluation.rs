use crate::cli;
use crate::frequency::{
    metrics,
    metrics::DistanceMetric,
    trace,
    trace::{FreqTrace, VectFreqTrace},
};
use crate::parse::DataFields;
use crate::utils;

use indexmap::IndexSet;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    iter::FromIterator,
    str::FromStr,
};

/// Runs the evaluation by conducting a specified number of linkage attacks that are
/// independent from each other. The traces are compared using the histogram-based approach.
/// 
/// Due to the independence, the linkage attacks can be performed in parallel. 
pub fn eval(
    config: &cli::Config,
    user_to_freq_map: &BTreeMap<u32, Vec<FreqTrace>>,
    user_to_target_idx_map: &HashMap<u32, Vec<usize>>,
    user_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    user_to_test_idx_map: &HashMap<u32, usize>,
) {
    let result_list: Vec<(bool, bool, bool)> = user_to_target_idx_map
        .par_iter()
        .map(|(user, target_idx_list)| {
            eval_step(
                config,
                user,
                &target_idx_list,
                &user_to_freq_map,
                &user_to_sample_idx_map,
                &user_to_test_idx_map,
            )
        })
        .collect();

    let mut top_1_list: Vec<f64> = Vec::with_capacity(result_list.len());
    let mut top_10_list: Vec<f64> = Vec::with_capacity(result_list.len());
    let mut top_10_percent_list: Vec<f64> = Vec::with_capacity(result_list.len());
    for (in_top_1, in_top_10, in_top_10_percent) in result_list.iter() {
        if *in_top_1 {
            top_1_list.push(1.0);
        } else {
            top_1_list.push(0.0);
        }
        if *in_top_10 {
            top_10_list.push(1.0);
        } else {
            top_10_list.push(0.0);
        }
        if *in_top_10_percent {
            top_10_percent_list.push(1.0);
        } else {
            top_10_percent_list.push(0.0);
        }
    }

    let top_1: f64 = utils::mean(&top_1_list);
    log::info!("Rank 1: {:?}", top_1);
    let top_10: f64 = utils::mean(&top_10_list);
    log::info!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = utils::mean(&top_10_percent_list);
    log::info!("Top 10 Percent: {:?}", top_10_percent);

    let top_1_std = utils::std_deviation(&top_1_list);
    let top_10_std = utils::std_deviation(&top_10_list);
    let top_10_percent_std = utils::std_deviation(&top_10_percent_list);

    // Write metrics to final evaluation file
    utils::write_to_file(
        config,
        top_1,
        top_1_std,
        top_10,
        top_10_std,
        top_10_percent,
        top_10_percent_std,
    )
    .expect("Error writing to evaluation file.");
}

/// Performs a single independent linkage attack.
fn eval_step(
    config: &cli::Config,
    user_target: &u32,
    target_idx_list: &Vec<usize>,
    user_to_freq_map: &BTreeMap<u32, Vec<FreqTrace>>,
    user_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    user_to_test_idx_map: &HashMap<u32, usize>,
) -> (bool, bool, bool) {
    let metric = DistanceMetric::from_str(&config.metric).unwrap();
    let mut result_map: HashMap<u32, OrderedFloat<f64>> = HashMap::new();
    let mut result_tuples: Vec<(u32, OrderedFloat<f64>)> =
        Vec::with_capacity(user_to_freq_map.len());

    for target_idx in target_idx_list.into_iter() {
        let target_trace = user_to_freq_map
            .get(user_target)
            .unwrap()
            .get(*target_idx)
            .unwrap();

        for (user, traces) in user_to_freq_map.into_iter() {
            let samples_idx = user_to_sample_idx_map.get(user).unwrap();
            let sampled_traces: Vec<FreqTrace> = samples_idx
                .into_iter()
                .map(|idx| traces.get(*idx).unwrap().clone())
                .collect();

            let speed_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Speed,
            );
            let heading_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Heading,
            );
            let street_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Street,
            );
            let postcode_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Postcode,
            );
            let state_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::State,
            );
            let highway_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Highway,
            );
            let hamlet_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Hamlet,
            );
            let suburb_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Suburb,
            );
            let village_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::Village,
            );
            let location_code_set = get_unique_set(
                target_trace,
                &sampled_traces,
                &DataFields::LocationCode,
            );

            let vect_target_trace = trace::vectorize_trace(
                target_trace,
                &speed_set,
                &heading_set,
                &street_set,
                &postcode_set,
                &state_set,
                &highway_set,
                &hamlet_set,
                &suburb_set,
                &village_set,
                &location_code_set,
            );

            if config.typical && !config.dependent {
                let vect_typ_ref_trace = trace::gen_typical_vect_trace(
                    &sampled_traces,
                    &speed_set,
                    &heading_set,
                    &street_set,
                    &postcode_set,
                    &state_set,
                    &highway_set,
                    &hamlet_set,
                    &suburb_set,
                    &village_set,
                    &location_code_set,
                );
                let dist = compute_dist(
                    &config.fields,
                    &metric,
                    &vect_target_trace,
                    &vect_typ_ref_trace,
                );
                result_tuples.push((user.clone(), OrderedFloat(dist)));
            } else if !config.typical && !config.dependent {
                for sample_trace in sampled_traces.into_iter() {
                    let vect_ref_trace = trace::vectorize_trace(
                        &sample_trace,
                        &speed_set,
                        &heading_set,
                        &street_set,
                        &postcode_set,
                        &state_set,
                        &highway_set,
                        &hamlet_set,
                        &suburb_set,
                        &village_set,
                        &location_code_set,
                    );
                    let dist = compute_dist(
                        &config.fields,
                        &metric,
                        &vect_target_trace,
                        &vect_ref_trace,
                    );
                    result_tuples.push((user.clone(), OrderedFloat(dist)));
                }
            } else {
                let test_idx: usize = user_to_test_idx_map.get(user).unwrap().clone();
                let trace: FreqTrace =
                    traces.get(test_idx).unwrap().clone();
                let vect_ref_trace = trace::vectorize_trace(
                    &trace,
                    &speed_set,
                    &heading_set,
                    &street_set,
                    &postcode_set,
                    &state_set,
                    &highway_set,
                    &hamlet_set,
                    &suburb_set,
                    &village_set,
                    &location_code_set,
                );
                let dist = compute_dist(
                    &config.fields,
                    &metric,
                    &vect_target_trace,
                    &vect_ref_trace,
                );
                *result_map
                    .entry(user.clone())
                    .or_insert(OrderedFloat(0.0)) += OrderedFloat(dist);
            }
        }
    }

    if config.dependent {
        result_tuples = result_map.into_iter().collect();
    }

    result_tuples.sort_unstable_by_key(|k| k.1);
    let cutoff: usize = (0.1 * user_to_freq_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(user_target, &result_tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(user_target, &result_tuples[..10]);
    let is_top_1: bool = user_target.clone() == result_tuples[0].0;
    (is_top_1, is_top_10, is_top_10_percent)
}

/// Calculates the distance between the target and the reference trace.
fn compute_dist<T, U>(
    fields: &Vec<DataFields>,
    metric: &DistanceMetric,
    target_trace: &VectFreqTrace<T>,
    ref_trace: &VectFreqTrace<U>,
) -> f64
where
    T: Clone
        + std::cmp::PartialEq
        + std::fmt::Debug
        + num_traits::ToPrimitive
        + std::cmp::PartialOrd
        + num_traits::Zero,
    U: Clone
        + std::cmp::PartialEq
        + std::fmt::Debug
        + num_traits::ToPrimitive
        + std::cmp::PartialOrd
        + num_traits::Zero,
{
    // Vector to store distance scores for each data field to be considered
    let mut total_dist = Vec::<f64>::with_capacity(fields.len());

    // Iterate over all data fields that are considered
    for field in fields.into_iter() {
        let (target_vector, ref_vector) = match field {
            DataFields::Speed => (
                target_trace.speed.clone(),
                ref_trace.speed.clone(),
            ),
            DataFields::Heading => (
                target_trace.heading.clone(),
                ref_trace.heading.clone(),
            ),
            DataFields::Street => (
                target_trace.street.clone(),
                ref_trace.street.clone(),
            ),
            DataFields::Postcode => (
                target_trace.postcode.clone(),
                ref_trace.postcode.clone(),
            ),
            DataFields::State => (
                target_trace.state.clone(),
                ref_trace.state.clone(),
            ),
            DataFields::Highway => (
                target_trace.highway.clone(),
                ref_trace.highway.clone(),
            ),
            DataFields::Hamlet => (
                target_trace.hamlet.clone(),
                ref_trace.hamlet.clone(),
            ),
            DataFields::Suburb => (
                target_trace.suburb.clone(),
                ref_trace.suburb.clone(),
            ),
            DataFields::Village => (
                target_trace.village.clone(),
                ref_trace.village.clone(),
            ),
            DataFields::Day => (
                target_trace.day.clone(),
                ref_trace.day.clone(),
            ),
            DataFields::Hour => (
                target_trace.hour.clone(),
                ref_trace.hour.clone(),
            ),
            DataFields::LocationCode => (
                target_trace.location_code.clone(),
                ref_trace.location_code.clone(),
            ),
        };

        let dist = match metric {
            DistanceMetric::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetric::Manhattan => metrics::manhattan_dist(target_vector, ref_vector),
            DistanceMetric::Cosine => metrics::consine_dist(target_vector, ref_vector),
            DistanceMetric::NonIntersection => {
                metrics::non_intersection_dist(target_vector, ref_vector)
            }
            DistanceMetric::Bhattacharyya => metrics::bhattacharyya_dist(target_vector, ref_vector),
            DistanceMetric::KullbrackLeibler => {
                metrics::kullbrack_leibler_dist(target_vector, ref_vector)
            }
            DistanceMetric::TotalVariation => {
                metrics::total_variation_dist(target_vector, ref_vector)
            }
            DistanceMetric::JeffriesMatusita => metrics::jeffries_dist(target_vector, ref_vector),
            DistanceMetric::ChiSquared => metrics::chi_squared_dist(target_vector, ref_vector),
        };
        total_dist.push(dist);
    }

    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}

/// Retrieves the set of unique values for a given target trace and sampled traces and a specific data field.
pub fn get_unique_set(
    target_trace: &FreqTrace,
    sampled_traces: &Vec<FreqTrace>,
    field: &DataFields,
) -> IndexSet<String> {
    let mut vector: Vec<String> = match field {
        DataFields::Speed => target_trace.speed.keys().cloned().collect(),
        DataFields::Heading => target_trace.heading.keys().cloned().collect(),
        DataFields::Street => target_trace.street.keys().cloned().collect(),
        DataFields::Postcode => target_trace.postcode.keys().cloned().collect(),
        DataFields::State => target_trace.state.keys().cloned().collect(),
        DataFields::Highway => target_trace.highway.keys().cloned().collect(),
        DataFields::Hamlet => target_trace.hamlet.keys().cloned().collect(),
        DataFields::Suburb => target_trace.suburb.keys().cloned().collect(),
        DataFields::Village => target_trace.village.keys().cloned().collect(),
        DataFields::LocationCode => target_trace
            .location_code
            .keys()
            .cloned()
            .collect(),
        _ => panic!("Error: unknown data field supplied: {}", field),
    };

    for trace in sampled_traces.into_iter() {
        match field {
            DataFields::Speed => vector.extend(trace.speed.keys().cloned()),
            DataFields::Heading => vector.extend(trace.heading.keys().cloned()),
            DataFields::Street => vector.extend(trace.street.keys().cloned()),
            DataFields::Postcode => vector.extend(trace.postcode.keys().cloned()),
            DataFields::State => vector.extend(trace.state.keys().cloned()),
            DataFields::Highway => vector.extend(trace.highway.keys().cloned()),
            DataFields::Hamlet => vector.extend(trace.hamlet.keys().cloned()),
            DataFields::Suburb => vector.extend(trace.suburb.keys().cloned()),
            DataFields::Village => vector.extend(trace.village.keys().cloned()),
            DataFields::LocationCode => vector.extend(trace.location_code.keys().cloned()),
            _ => panic!("Error: unknown data field supplied: {}", field),
        }
    }
    let set: IndexSet<String> = IndexSet::from_iter(vector);
    set
}
