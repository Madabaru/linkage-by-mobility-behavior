use crate::utils;

#[derive(Debug, Clone)]
pub struct SeqClickTrace {
    pub speed: Vec<u32>,
    pub heading: Vec<u32>,
    pub street: Vec<u32>,
    pub postcode: Vec<u32>,
    pub state: Vec<u32>,
    pub hour: Vec<u32>,
    pub day: u32,
    pub start_time: f64,
    pub end_time: f64,
}

pub fn gen_typical_click_trace(click_traces: &Vec<SeqClickTrace>) -> SeqClickTrace {
    // Get length of typical click trace by majority vote
    let lengths: Vec<usize> = click_traces.iter().map(|cl| cl.speed.len()).collect();
    let typical_length = utils::get_most_freq_element(&lengths);

    // Get typical day
    let days: Vec<u32> = click_traces.iter().map(|cl| cl.day).collect();
    let typical_day = utils::get_most_freq_element(&days);

    // Get typical speed
    let mut typical_speeds: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_speeds.iter_mut().enumerate() {
        let speeds: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.speed.len() > i)
            .map(|cl| cl.speed[i])
            .collect();
        let typical_speed = utils::get_most_freq_element(&speeds);
        *x = typical_speed;
    }

    // Get typical heading
    let mut typical_headings: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_headings.iter_mut().enumerate() {
        let headings: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.heading.len() > i)
            .map(|cl| cl.heading[i])
            .collect();
        let typical_heading = utils::get_most_freq_element(&headings);
        *x = typical_heading;
    }

    // Get typical street
    let mut typical_streets: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_streets.iter_mut().enumerate() {
        let streets: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.street.len() > i)
            .map(|cl| cl.street[i])
            .collect();
        let typical_street = utils::get_most_freq_element(&streets);
        *x = typical_street;
    }

    // Get typical hour
    let mut typical_states: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_states.iter_mut().enumerate() {
        let states: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.state.len() > i)
            .map(|cl| cl.state[i])
            .collect();
        let typical_state = utils::get_most_freq_element(&states);
        *x = typical_state;
    }

    // Get typical postcode
    let mut typical_postcodes: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_postcodes.iter_mut().enumerate() {
        let postcodes: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.postcode.len() > i)
            .map(|cl| cl.postcode[i])
            .collect();
        let typical_postcode = utils::get_most_freq_element(&postcodes);
        *x = typical_postcode;
    }

    // Get typical hour
    let mut typical_hours: Vec<u32> = vec![0; typical_length];
    for (i, x) in typical_hours.iter_mut().enumerate() {
        let hours: Vec<u32> = click_traces
            .iter()
            .filter(|cl| cl.hour.len() > i)
            .map(|cl| cl.hour[i])
            .collect();
        let typical_hour = utils::get_most_freq_element(&hours);
        *x = typical_hour;
    }

    // Create typical click trace from typical values
    let typical_click_trace = SeqClickTrace {
        street: typical_streets,
        postcode: typical_postcodes,
        state: typical_states,
        hour: typical_hours,
        day: typical_day,
        start_time: 0.0,
        end_time: 0.0,
        speed: typical_speeds,
        heading: typical_headings,
    };
    typical_click_trace
}

pub fn reverse_click_trace(click_trace: &SeqClickTrace) -> SeqClickTrace {
    let mut reverse_click_trace = click_trace.clone();
    reverse_click_trace.speed.reverse();
    reverse_click_trace.heading.reverse();
    reverse_click_trace.street.reverse();
    reverse_click_trace.postcode.reverse();
    reverse_click_trace.state.reverse();
    reverse_click_trace
}


