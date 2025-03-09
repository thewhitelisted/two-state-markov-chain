// two state markov chain simulation
// 1. white -> white or white -> black
// 2. black -> white or black -> black

// imports
use rand::thread_rng;
use rand::seq::SliceRandom;

// computes the steady state of a markov chain
// shows long run behaviour of the chain
fn steady_state(transition_matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let alpha: f64 = transition_matrix[0][0];
    let beta: f64 = transition_matrix[1][1];

    let x: f64 = (1.0 - beta) / (2.0 - (alpha + beta));
    let y: f64 = (1.0 - alpha) / (2.0 - (alpha + beta));

    return vec![x, y];
}

// calculates the dot product of two vectors
fn dot_product(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let mut result = 0.0;
    for i in 0..v1.len() {
        result += v1[i] * v2[i];
    }
    return result;
}

// predicts the next state given the current state for a number of draws. if it reaches steady state it will return 'W'
fn predict_next_state(transition_matrix: &Vec<Vec<f64>>, current_state: char, num_draws: usize) -> Vec<char> {
    // initial state vector
    let mut v0: Vec<f64> = vec![0.0, 0.0];
    if current_state == 'W' {
        v0[0] = 1.0;
    } else {
        v0[1] = 1.0;
    }

    let mut futures: Vec<Vec<f64>> = vec![vec![0.0, 0.0]; num_draws];
    futures[0] = v0;
    let mut results: Vec<char> = Vec::new();

    for i in 1..num_draws {
        futures[i][0] = dot_product(&futures[i-1], &transition_matrix[0]);
        futures[i][1] = dot_product(&futures[i-1], &transition_matrix[1]);
    }


    for i in 0..num_draws {
        if futures[i][0] < futures[i][1] {
            results.push('B');
        } else {
            results.push('W');
        }
    }

    return results;
}

// draws a ball randomly from a jar
fn draw_ball(jar: &mut Vec<char>) -> char {
    jar.shuffle(&mut thread_rng());
    return jar[0];
}

// creates a sequence that follows our rules
fn derive_sequence(jar_w: &mut Vec<char>, jar_b: &mut Vec<char>, num_draws: usize) -> Vec<char> {
    let mut sequence: Vec<char> = Vec::new();

    // draws start from W, can start from either W or B
    sequence.push('W');
    let mut a = draw_ball(jar_w);
    sequence.push(a);

    // drawing sequence
    for _ in 1..num_draws {
        if a == 'W' {
            a = draw_ball(jar_w);
        } else {
            a = draw_ball(jar_b);
        }
        sequence.push(a);
    }

    return sequence;
}

// checks if a jump from a jar has occured, returns the jump state and the colour of the jar
fn is_jump(prev: char, curr: char) -> Option<(bool, char)> {
    if prev == 'W' && curr == 'B' {
        return Some((true, 'W'));
    } else if prev == 'B' && curr == 'W' {
        return Some((true, 'B'));
    } 
    return None;
}

// calculates the transition probability matrix from a sequence
fn seq_to_transition_probability(seq: Vec<char>) -> Vec<Vec<f64>> {
    let mut w_wb = 0.0;
    let mut draws_from_white = 0.0;
    let mut b_bw = 0.0;
    let mut draws_from_black = 0.0;

    let mut result: Vec<Vec<f64>>= Vec::new();
    // start at the second element because the first one is predetermined.
    let mut prev: char = seq[1];

    // iterate and count the number of jumps and draws from each jar
    for i in 2..seq.len() {
        let curr: char = seq[i];

        if let Some((jump, colour)) = is_jump(prev, curr) {
            if jump {
                if colour == 'W' {
                    w_wb += 1.0;
                } else {
                    b_bw += 1.0;
                }
            }
        }
        if prev == 'W'{
            draws_from_white += 1.0;
        } else {
            draws_from_black += 1.0;
        }
        prev = curr;
    }

    // calculate the transition probabilities
    let wb: f64 = w_wb / draws_from_white as f64;
    let bw: f64 = b_bw / draws_from_black as f64;
    let ww: f64 = 1.0 - wb;
    let bb: f64 = 1.0 - bw;

    result.push(vec![ww, wb]);
    result.push(vec![bw, bb]);

    return result;
}

fn main() {
    // proportion of white and black balls in the jars
    let mut jar_w: Vec<char> = "WWBBBBBBBB".chars().collect();
    let mut jar_b: Vec<char> = "WWWWWWBBBB".chars().collect();
    let seq: Vec<char> = derive_sequence(&mut jar_w, &mut jar_b, 100);
    let transition_matrix: Vec<Vec<f64>> = seq_to_transition_probability(seq.clone());
    let actual_matrix: Vec<Vec<f64>> = vec![vec![0.17, 0.83], vec![0.56, 0.44]];

    // print transition matrix with 2 decimal places
    println!("Transition Matrix:");
    for row in transition_matrix.iter() {
        println!("{:.2} | {:.2}", row[0], row[1]);
    }
    println!("Sequence: {:?}", seq);

    // prediction, reaches steady state after 40 draws
    let predictions: Vec<char> = predict_next_state(&actual_matrix, 'W', 41);
    println!("Predictions: {:?}", predictions);

    // steady state
    let steady: Vec<f64> = steady_state(&actual_matrix);
    println!("Steady State: {:?}", steady);
}
