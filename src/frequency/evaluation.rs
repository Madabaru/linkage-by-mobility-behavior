use crate::cli;
use crate::frequency::{
    click_trace,
    click_trace::{FreqClickTrace, VectFreqClickTrace},
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
    client_to_freq_map: &BTreeMap<u32, Vec<FreqClickTrace>>,
    client_to_target_idx_map: &HashMap<u32, usize>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) {
    let result_list: Vec<(u32, u32, bool, bool)> = client_to_target_idx_map
        .par_iter()
        .map(|(client, target_idx)| {
            eval_step(
                config,
                client,
                target_idx,
                &client_to_freq_map,
                client_to_sample_idx_map,
            )
        })
        .collect();

    let mut correct_pred = 0;
    let mut top_10_count = 0;
    let mut top_10_percent_count = 0;
    for (pred, target, in_top_10, in_top_10_percent) in result_list.iter() {
        if pred == target {
            correct_pred += 1
        }
        if *in_top_10 {
            top_10_count += 1;
        }
        if *in_top_10_percent {
            top_10_percent_count += 1;
        }
    }

    let accuracy: f64 = correct_pred as f64 / result_list.len() as f64;
    log::info!("Rank 1: {:?}", accuracy);
    let top_10: f64 = top_10_count as f64 / result_list.len() as f64;
    log::info!("Top 10: {:?}", top_10);
    let top_10_percent: f64 = top_10_percent_count as f64 / result_list.len() as f64;
    log::info!("Top 10 Percent: {:?}", top_10_percent);

    // Write result to output file for further processing in python
    utils::write_to_output(result_list);
    // Write metrics to final evaluation file
    utils::write_to_eval(config, top_10, top_10_percent);
}

fn eval_step(
    config: &cli::Config,
    client_target: &u32,
    target_idx: &usize,
    client_to_freq_map: &BTreeMap<u32, Vec<FreqClickTrace>>,
    client_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
) -> (u32, u32, bool, bool) {
    let metric = DistanceMetric::from_str(&config.metric).unwrap();
    let target_click_trace = client_to_freq_map
        .get(client_target)
        .unwrap()
        .get(*target_idx)
        .unwrap();
        
    let mut tuples: Vec<(OrderedFloat<f64>, u32)> = Vec::with_capacity(client_to_freq_map.len());

    for (client, click_traces) in client_to_freq_map.into_iter() {
        let samples_idx = client_to_sample_idx_map.get(client).unwrap();
        let sampled_click_traces: Vec<FreqClickTrace> = samples_idx
            .into_iter()
            .map(|idx| click_traces.get(*idx).unwrap().clone())
            .collect();

        let speed_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Speed,
        );
        let heading_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Heading,
        );
        let street_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Street,
        );
        let postcode_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Postcode,
        );
        let state_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::State,
        );
        let highway_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Highway,
        );
        let hamlet_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Hamlet,
        );
        let suburb_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Suburb,
        );
        let village_set = get_unique_set(
            target_click_trace,
            &sampled_click_traces,
            &DataFields::Village,
        );

        let vectorized_target = click_trace::vectorize_click_trace(
            target_click_trace,
            &speed_set,
            &heading_set,
            &street_set,
            &postcode_set,
            &state_set,
            &highway_set, 
            &hamlet_set, 
            &suburb_set,
            &village_set
        );

        if config.typical {
            // let vect_typ_click_trace = click_trace::gen_typical_vect_click_trace(
            //     &sampled_click_traces,
            //     &speed_set,
            //     &heading_set,
            //     &street_set,
            //     &postcode_set,
            //     &state_set,
            // );
            // let dist = compute_dist(
            //     &config.fields,
            //     &metric,
            //     &vectorized_target,
            //     &vect_typ_click_trace,
            // );
            // tuples.push((OrderedFloat(dist), client.clone()));
        } else {
            for sample_click_trace in sampled_click_traces.into_iter() {
                let vectorized_ref = click_trace::vectorize_click_trace(
                    &sample_click_trace,
                    &speed_set,
                    &heading_set,
                    &street_set,
                    &postcode_set,
                    &state_set,
                    &highway_set, 
                    &hamlet_set, 
                    &suburb_set,
                    &village_set
                );
                let dist =
                    compute_dist(&config.fields, &metric, &vectorized_target, &vectorized_ref);
                tuples.push((OrderedFloat(dist), client.clone()));
            }
        }
    }
    tuples.sort_unstable_by_key(|k| k.0);
    let cutoff: usize = (0.1 * client_to_freq_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(client_target, &tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(client_target, &tuples[..10   ]);
    (
        client_target.clone(),
        tuples[0].1,
        is_top_10,
        is_top_10_percent,
    )
}

// Calculate the distance between the target and the reference click trace
fn compute_dist<T, U>(
    fields: &Vec<DataFields>,
    metric: &DistanceMetric,
    target_click_trace: &VectFreqClickTrace<T>,
    ref_click_trace: &VectFreqClickTrace<U>,
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
                target_click_trace.speed.clone(),
                ref_click_trace.speed.clone(),
            ),
            DataFields::Heading => (
                target_click_trace.heading.clone(),
                ref_click_trace.heading.clone(),
            ),
            DataFields::Street => (
                target_click_trace.street.clone(),
                ref_click_trace.street.clone(),
            ),
            DataFields::Postcode => (
                target_click_trace.postcode.clone(),
                ref_click_trace.postcode.clone(),
            ),
            DataFields::State => (
                target_click_trace.state.clone(),
                ref_click_trace.state.clone(),
            ),
            DataFields::Highway => (
                target_click_trace.highway.clone(),
                ref_click_trace.highway.clone(),
            ),
            DataFields::Hamlet => (
                target_click_trace.hamlet.clone(),
                ref_click_trace.hamlet.clone(),
            ),
            DataFields::Suburb => (
                target_click_trace.suburb.clone(),
                ref_click_trace.suburb.clone(),
            ),
            DataFields::Village => (
                target_click_trace.state.clone(),
                ref_click_trace.state.clone(),
            ),
            DataFields::Day => (target_click_trace.day.clone(), ref_click_trace.day.clone()),
            DataFields::Hour => (
                target_click_trace.hour.clone(),
                ref_click_trace.hour.clone(),
            ),
        };

        let dist = match metric {
            DistanceMetric::Euclidean => metrics::euclidean_dist(target_vector, ref_vector),
            DistanceMetric::Manhatten => metrics::manhattan_dist(target_vector, ref_vector),
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
    target_click_trace: &FreqClickTrace,
    sampled_click_traces: &Vec<FreqClickTrace>,
    field: &DataFields,
) -> IndexSet<String> {
    let mut vector: Vec<String> = match field {
        DataFields::Speed => target_click_trace.speed.keys().cloned().collect(),
        DataFields::Heading => target_click_trace.heading.keys().cloned().collect(),
        DataFields::Street => target_click_trace.street.keys().cloned().collect(),
        DataFields::Postcode => target_click_trace.postcode.keys().cloned().collect(),
        DataFields::State => target_click_trace.state.keys().cloned().collect(),
        DataFields::Highway => target_click_trace.highway.keys().cloned().collect(),
        DataFields::Hamlet => target_click_trace.hamlet.keys().cloned().collect(),
        DataFields::Suburb => target_click_trace.suburb.keys().cloned().collect(),
        DataFields::Village => target_click_trace.village.keys().cloned().collect(),
        _ => panic!("Error: unknown data field supplied: {}", field),
    };

    for click_trace in sampled_click_traces.into_iter() {
        match field {
            DataFields::Speed => vector.extend(click_trace.speed.keys().cloned()),
            DataFields::Heading => vector.extend(click_trace.heading.keys().cloned()),
            DataFields::Street => vector.extend(click_trace.street.keys().cloned()),
            DataFields::Postcode => vector.extend(click_trace.postcode.keys().cloned()),
            DataFields::State => vector.extend(click_trace.state.keys().cloned()),
            DataFields::Highway => vector.extend(click_trace.highway.keys().cloned()),
            DataFields::Hamlet => vector.extend(click_trace.hamlet.keys().cloned()),
            DataFields::Suburb => vector.extend(click_trace.suburb.keys().cloned()),
            DataFields::Village => vector.extend(click_trace.village.keys().cloned()),
            _ => panic!("Error: unknown data field supplied: {}", field),
        }
    }
    let set: IndexSet<String> = IndexSet::from_iter(vector);
    set
}
