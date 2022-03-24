use crate::parse::DataFields;
use crate::sequence::trace::{reverse_trace, SeqTrace};
use crate::utils;
use crate::{cli, sequence};

use ordered_float::OrderedFloat;
use rayon::prelude::*;
use seal::pair::{AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
};

/// Runs the evaluation by conducting a specified number of linkage attacks that are
/// independent from each other. The traces are compared using the sequence alignment-based approach.
/// 
/// Due to the independence, the linkage attacks can be performed in parallel. 
pub fn eval(
    config: &cli::Config,
    user_to_seq_map: &BTreeMap<u32, Vec<SeqTrace>>,
    user_to_target_idx_map: &HashMap<u32, Vec<usize>>,
    user_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    user_to_test_idx_map: &HashMap<u32, usize>,
) {
    let result_list: Vec<(bool, bool, bool)> = user_to_target_idx_map
        .par_iter()
        .map(|(user_target, target_idx_list)| {
            eval_step(
                config,
                user_target,
                &target_idx_list,
                &user_to_seq_map,
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
    user_to_seq_map: &BTreeMap<u32, Vec<SeqTrace>>,
    user_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    user_to_test_idx_map: &HashMap<u32, usize>,
) -> (bool, bool, bool) {
    let mut result_map: HashMap<u32, OrderedFloat<f64>> = HashMap::new();
    let mut result_tuples: Vec<(u32, OrderedFloat<f64>)> =
        Vec::with_capacity(user_to_seq_map.len());

    for target_idx in target_idx_list.into_iter() {
        let target_trace = user_to_seq_map
            .get(user_target)
            .unwrap()
            .get(*target_idx)
            .unwrap();
        let reverse_target_trace = reverse_trace(target_trace);

        for (user, traces) in user_to_seq_map.into_iter() {
            let samples_idx = user_to_sample_idx_map.get(user).unwrap();
            let sampled_traces: Vec<SeqTrace> = samples_idx
                .into_iter()
                .map(|idx| traces.get(*idx).unwrap().clone())
                .collect();

            if config.typical && !config.dependent {
                let typical_ref_trace =
                    sequence::trace::gen_typical_trace(&sampled_traces);

                let mut score = compute_alignment_scores(
                    &config.fields,
                    &config.strategy,
                    &config.scope,
                    &config.scoring_matrix,
                    &target_trace,
                    &typical_ref_trace,
                );

                if config.reverse {
                    let score_reverse = compute_alignment_scores(
                        &config.fields,
                        &config.strategy,
                        &config.scope,
                        &config.scoring_matrix,
                        &reverse_target_trace,
                        &typical_ref_trace,
                    );
                    if score < score_reverse {
                        score = score_reverse;
                    }
                }
                result_tuples.push((user.clone(), OrderedFloat(score)));
            } else if !config.typical && !config.dependent {
                for ref_trace in sampled_traces.into_iter() {
                    let mut score = compute_alignment_scores(
                        &config.fields,
                        &config.strategy,
                        &config.scope,
                        &config.scoring_matrix,
                        &target_trace,
                        &ref_trace,
                    );

                    if config.reverse {
                        let score_reverse = compute_alignment_scores(
                            &config.fields,
                            &config.strategy,
                            &config.scope,
                            &config.scoring_matrix,
                            &reverse_target_trace,
                            &ref_trace,
                        );
                        if score < score_reverse {
                            score = score_reverse;
                        }
                    }
                    result_tuples.push((user.clone(), OrderedFloat(score)));
                }
            } else {
                let test_idx: usize = user_to_test_idx_map.get(user).unwrap().clone();
                let ref_trace: SeqTrace =
                    traces.get(test_idx).unwrap().clone();
                let score = compute_alignment_scores(
                    &config.fields,
                    &config.strategy,
                    &config.scope,
                    &config.scoring_matrix,
                    &target_trace,
                    &ref_trace,
                );
                *result_map
                    .entry(user.clone())
                    .or_insert(OrderedFloat(0.0)) += OrderedFloat(score);
            }
        }
    }
    if config.dependent {
        result_tuples = result_map.into_iter().collect();
    }
    result_tuples.sort_unstable_by_key(|k| Reverse(k.1));
    let cutoff: usize = (0.1 * user_to_seq_map.len() as f64) as usize;
    let is_top_10_percent = utils::is_target_in_top_k(user_target, &result_tuples[..cutoff]);
    let is_top_10: bool = utils::is_target_in_top_k(user_target, &result_tuples[..10]);
    let is_top_1 = user_target.clone() == result_tuples[0].0;
    (is_top_1, is_top_10, is_top_10_percent)
}

/// Calculates the alignment score between the target and the reference trace.
fn compute_alignment_scores(
    fields: &Vec<DataFields>,
    strategy: &str,
    scope: &str,
    scoring_matrix: &[isize],
    target_trace: &SeqTrace,
    ref_trace: &SeqTrace,
) -> f64 {
    let mut align_scores = Vec::<f64>::with_capacity(fields.len());
    let mut unnormalized_align_scores = Vec::<f64>::with_capacity(fields.len());

    for field in fields.into_iter() {
        let score = match field {
            DataFields::Speed => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.speed.clone(),
                ref_trace.speed.clone(),
            ),
            DataFields::Heading => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.heading.clone(),
                ref_trace.heading.clone(),
            ),
            DataFields::Street => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.street.clone(),
                ref_trace.street.clone(),
            ),
            DataFields::Postcode => compute_similarity_score(
                target_trace.postcode.clone(),
                ref_trace.postcode.clone(),
            ),
            DataFields::State => compute_similarity_score(
                target_trace.state.clone(),
                ref_trace.state.clone(),
            ),
            DataFields::Day => compute_similarity_score(
                target_trace.day.clone(),
                ref_trace.day.clone(),
            ),
            DataFields::Hour => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.hour.clone(),
                ref_trace.hour.clone(),
            ),
            DataFields::Highway => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.highway.clone(),
                ref_trace.highway.clone(),
            ),
            DataFields::Hamlet => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.hamlet.clone(),
                ref_trace.hamlet.clone(),
            ),
            DataFields::Suburb => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.suburb.clone(),
                ref_trace.suburb.clone(),
            ),
            DataFields::Village => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.village.clone(),
                ref_trace.village.clone(),
            ),
            DataFields::LocationCode => compute_sequence_alignment(
                strategy,
                scope,
                scoring_matrix,
                target_trace.location_code.clone(),
                ref_trace.location_code.clone(),
            ),
        };

        match field {
            DataFields::Speed => unnormalized_align_scores.push(score),
            DataFields::Heading => unnormalized_align_scores.push(score),
            DataFields::Street => unnormalized_align_scores.push(score),
            DataFields::Postcode => unnormalized_align_scores.push(score),
            DataFields::State => unnormalized_align_scores.push(score),
            DataFields::Day => align_scores.push(score),
            DataFields::Hour => unnormalized_align_scores.push(score),
            DataFields::Highway => unnormalized_align_scores.push(score),
            DataFields::Hamlet => unnormalized_align_scores.push(score),
            DataFields::Suburb => unnormalized_align_scores.push(score),
            DataFields::Village => unnormalized_align_scores.push(score),
            DataFields::LocationCode => unnormalized_align_scores.push(score),
        }
    }

    // Normalize scores
    utils::normalize_vector(&mut unnormalized_align_scores);
    align_scores.append(&mut unnormalized_align_scores);

    // Compute the final score by averaging the indivdual scores
    let avg_score = align_scores.iter().sum::<f64>() / align_scores.len() as f64;
    avg_score
}

fn compute_sequence_alignment(
    strategy: &str,
    scope: &str,
    scoring_matrix: &[isize],
    target_trace: Vec<u32>,
    ref_trace: Vec<u32>,
) -> f64 {
    let set: AlignmentSet<InMemoryAlignmentMatrix> = match strategy {
        "nw" => {
            let strategy = NeedlemanWunsch::new(
                scoring_matrix[0],
                scoring_matrix[1],
                scoring_matrix[2],
                scoring_matrix[3],
            );
            AlignmentSet::new(target_trace.len(), ref_trace.len(), strategy, |x, y| {
                target_trace[x] == ref_trace[y]
            })
            .unwrap()
        }
        "sw" => {
            let strategy = SmithWaterman::new(
                scoring_matrix[0],
                scoring_matrix[1],
                scoring_matrix[2],
                scoring_matrix[3],
            );
            AlignmentSet::new(target_trace.len(), ref_trace.len(), strategy, |x, y| {
                target_trace[x] == ref_trace[y]
            })
            .unwrap()
        }
        _ => panic!("Error: unknown strategy name supplied: {}", strategy),
    };

    let score = match scope {
        "global" => set.global_score() as f64,
        "local" => set.local_score() as f64,
        _ => panic!("Error: unknown scope name supplied: {}", scope),
    };
    score
}

fn compute_similarity_score<T: std::cmp::PartialEq>(target_trace_val: T, ref_trace_val: T) -> f64 {
    let score;
    if target_trace_val == ref_trace_val {
        score = 1.0;
    } else {
        score = 0.0;
    }
    score
}
