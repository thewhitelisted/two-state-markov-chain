// two state markov chain simulation
// 1. white -> white or white -> black
// 2. black -> white or black -> black

// imports
use rand::thread_rng;
use rand::seq::SliceRandom;

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
    let mut jar_w: Vec<char> = "WWBBBBBBBB".chars().collect();
    let mut jar_b: Vec<char> = "WWWWWWBBBB".chars().collect();
    let seq = derive_sequence(&mut jar_w, &mut jar_b, 100);
    let transition_matrix = seq_to_transition_probability(seq.clone());

    // print transition matrix with 2 decimal places
    println!("Transition Matrix:");
    for row in transition_matrix.iter() {
        println!("{:.2} | {:.2}", row[0], row[1]);
    }
    println!("Sequence: {:?}", seq);
}
