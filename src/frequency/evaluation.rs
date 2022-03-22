use crate::cli;
use crate::frequency::{
    mobility_trace,
    mobility_trace::{FreqMobilityTrace, VectFreqMobilityTrace},
    metrics,
    metrics::DistanceMetric,
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

pub fn eval(
    config: &cli::Config,
    client_to_freq_map: &BTreeMap<u32, Vec<FreqMobilityTrace>>,
    client_to_target_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_test_idx_map: &HashMap<u32, usize>
) {
    let result_list: Vec<(bool, bool, bool)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx_list)| {
            eval_step(
                config,
                client,
                &target_idx_list,
                &client_to_freq_map,
                &client_to_sample_idx_map,
                &client_to_test_idx_map
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
    utils::write_to_file(config, top_1, top_1_std, top_10, top_10_std, top_10_percent, top_10_percent_std).expect("Error writing to evaluation file.");
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx_list: &Vec<usize>,
    client_to_freq_map: &BTreeMap<u32, Vec<FreqMobilityTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    client_to_test_idx_map: &HashMap<u32, usize>
) -> (bool, bool, bool) {

    let metric = DistanceMetric::from_str(&config.metric).unwrap();
    let mut result_map: HashMap<u32, OrderedFloat<f64>> = HashMap::new();
    let mut result_tuples: Vec<(u32, OrderedFloat<f64>)> = Vec::with_capacity(client_to_freq_map.len());

    for target_idx in target_idx_list.into_iter() {
        let target_mobility_trace = client_to_freq_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();

        for (client, mobility_traces) in client_to_freq_map.into_iter() {
            
            let samples_idx = client_to_sample_idx_map.get(client).unwrap();
            let sampled_mobility_traces: Vec<FreqMobilityTrace> = samples_idx
                .into_iter()
                .map(|idx| mobility_traces.get(*idx).unwrap().clone())
                .collect();

            let speed_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Speed,
            );
            let heading_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Heading,
            );
            let street_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Street,
            );
            let postcode_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Postcode,
            );
            let state_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::State,
            );
            let highway_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Highway,
            );
            let hamlet_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Hamlet,
            );
            let suburb_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Suburb,
            );
            let village_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::Village,
            );
            let location_code_set = get_unique_set(
                target_mobility_trace,
                &sampled_mobility_traces,
                &DataFields::LocationCode,
            );

            let vect_target_mobility_trace = mobility_trace::vectorize_mobility_trace(
                target_mobility_trace,
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

            if config.typical && !config.dependent  {
                
                let vect_typ_ref_mobility_trace = mobility_trace::gen_typical_vect_mobility_trace(
                    &sampled_mobility_traces,
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
                    &vect_target_mobility_trace,
                    &vect_typ_ref_mobility_trace,
                );
                result_tuples.push((client.clone(), OrderedFloat(dist)));
            
            } else if !config.typical && !config.dependent {

                for sample_mobility_trace in sampled_mobility_traces.into_iter() {
                    let vect_ref_mobility_trace = mobility_trace::vectorize_mobility_trace(
                        &sample_mobility_trace,
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
                    let dist =
                        compute_dist(&config.fields, &metric, &vect_target_mobility_trace, &vect_ref_mobility_trace);
                    result_tuples.push((client.clone(), OrderedFloat(dist)));
                }

            } else {

                let test_idx: usize = client_to_test_idx_map.get(client).unwrap().clone();
                let mobility_trace: FreqMobilityTrace = mobility_traces.get(test_idx).unwrap().clone();
                let vect_ref_mobility_trace = mobility_trace::vectorize_mobility_trace(
                    &mobility_trace,
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
                let dist =
                    compute_dist(&config.fields, &metric, &vect_target_mobility_trace, &vect_ref_mobility_trace);
                *result_map.entry(client.clone()).or_insert(OrderedFloat(0.0)) += OrderedFloat(dist);
            }
        }
    }

    if config.dependent {
        result_tuples = result_map.into_iter().collect();
    }

    result_tuples.sort_unstable_by_key(|k| k.1);
    let cutoff: usize = (0.1 * client_to_freq_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(client_target, &result_tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(client_target, &result_tuples[..10]);
    let is_top_1: bool = client_target.clone() == result_tuples[0].0;
    (
        is_top_1,
        is_top_10,
        is_top_10_percent,
    )
    }

// Calculate the distance between the target and the reference click trace
fn compute_dist<T, U>(
    fields: &Vec<DataFields>,
    metric: &DistanceMetric,
    target_mobility_trace: &VectFreqMobilityTrace<T>,
    ref_mobility_trace: &VectFreqMobilityTrace<U>,
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
                target_mobility_trace.speed.clone(),
                ref_mobility_trace.speed.clone(),
            ),
            DataFields::Heading => (
                target_mobility_trace.heading.clone(),
                ref_mobility_trace.heading.clone(),
            ),
            DataFields::Street => (
                target_mobility_trace.street.clone(),
                ref_mobility_trace.street.clone(),
            ),
            DataFields::Postcode => (
                target_mobility_trace.postcode.clone(),
                ref_mobility_trace.postcode.clone(),
            ),
            DataFields::State => (
                target_mobility_trace.state.clone(),
                ref_mobility_trace.state.clone(),
            ),
            DataFields::Highway => (
                target_mobility_trace.highway.clone(),
                ref_mobility_trace.highway.clone(),
            ),
            DataFields::Hamlet => (
                target_mobility_trace.hamlet.clone(),
                ref_mobility_trace.hamlet.clone(),
            ),
            DataFields::Suburb => (
                target_mobility_trace.suburb.clone(),
                ref_mobility_trace.suburb.clone(),
            ),
            DataFields::Village => (
                target_mobility_trace.village.clone(),
                ref_mobility_trace.village.clone(),
            ),
            DataFields::Day => (target_mobility_trace.day.clone(), ref_mobility_trace.day.clone()),
            DataFields::Hour => (
                target_mobility_trace.hour.clone(),
                ref_mobility_trace.hour.clone(),
            ),
            DataFields::LocationCode =>      (           
                target_mobility_trace.location_code.clone(),
                ref_mobility_trace.location_code.clone(),
            ),
        };

        let dist = match metric {
            DistanceMetric::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetric::Manhattan => metrics::manhattan_dist(target_vector, ref_vector),
            DistanceMetric::Cosine => metrics::consine_dist(target_vector, ref_vector),
            DistanceMetric::NonIntersection => metrics::non_intersection_dist(target_vector, ref_vector),
            DistanceMetric::Bhattacharyya => metrics::bhattacharyya_dist(target_vector, ref_vector),
            DistanceMetric::KullbrackLeibler => metrics::kullbrack_leibler_dist(target_vector, ref_vector),
            DistanceMetric::TotalVariation => metrics::total_variation_dist(target_vector, ref_vector),
            DistanceMetric::JeffriesMatusita => metrics::jeffries_dist(target_vector, ref_vector),
            DistanceMetric::ChiSquared => metrics::chi_squared_dist(target_vector, ref_vector),
        };
        total_dist.push(dist);
    }

    // Compute the final score by averaging the indivdual scores
    let avg_dist = total_dist.iter().sum::<f64>() / total_dist.len() as f64;
    avg_dist
}

pub fn get_unique_set(
    target_mobility_trace: &FreqMobilityTrace,
    sampled_mobility_traces: &Vec<FreqMobilityTrace>,
    field: &DataFields,
) -> IndexSet<String> {
    let mut vector: Vec<String> = match field {
        DataFields::Speed => target_mobility_trace.speed.keys().cloned().collect(),
        DataFields::Heading => target_mobility_trace.heading.keys().cloned().collect(),
        DataFields::Street => target_mobility_trace.street.keys().cloned().collect(),
        DataFields::Postcode => target_mobility_trace.postcode.keys().cloned().collect(),
        DataFields::State => target_mobility_trace.state.keys().cloned().collect(),
        DataFields::Highway => target_mobility_trace.highway.keys().cloned().collect(),
        DataFields::Hamlet => target_mobility_trace.hamlet.keys().cloned().collect(),
        DataFields::Suburb => target_mobility_trace.suburb.keys().cloned().collect(),
        DataFields::Village => target_mobility_trace.village.keys().cloned().collect(),
        DataFields::LocationCode => target_mobility_trace.location_code.keys().cloned().collect(),
        _ => panic!("Error: unknown data field supplied: {}", field),
    };

    for mobility_trace in sampled_mobility_traces.into_iter() {
        match field {
            DataFields::Speed => vector.extend(mobility_trace.speed.keys().cloned()),
            DataFields::Heading => vector.extend(mobility_trace.heading.keys().cloned()),
            DataFields::Street => vector.extend(mobility_trace.street.keys().cloned()),
            DataFields::Postcode => vector.extend(mobility_trace.postcode.keys().cloned()),
            DataFields::State => vector.extend(mobility_trace.state.keys().cloned()),
            DataFields::Highway => vector.extend(mobility_trace.highway.keys().cloned()),
            DataFields::Hamlet => vector.extend(mobility_trace.hamlet.keys().cloned()),
            DataFields::Suburb => vector.extend(mobility_trace.suburb.keys().cloned()),
            DataFields::Village => vector.extend(mobility_trace.village.keys().cloned()),
            DataFields::LocationCode => vector.extend(mobility_trace.location_code.keys().cloned()),
            _ => panic!("Error: unknown data field supplied: {}", field),
        }
    }
    let set: IndexSet<String> = IndexSet::from_iter(vector);
    set
}
