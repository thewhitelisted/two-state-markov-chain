fn is_jump(prev: char, curr: char) -> Option<(bool, char)> {
    if prev == 'W' && curr == 'B' {
        return Some((true, 'W'));
    } else if prev == 'B' && curr == 'W' {
        return Some((true, 'B'));
    } 
    return None;
}

fn seq_to_transition_probability(seq: Vec<char>) -> Vec<Vec<f64>> {
    let mut w_wb = 0.0;
    let mut draws_from_white = 0.0;
    let mut b_bw = 0.0;
    let mut draws_from_black = 0.0;

    let mut result: Vec<Vec<f64>>= Vec::new();
    let mut prev: char = seq[1];

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

    println!("{:?}", w_wb);
    println!("{:?}", draws_from_white);
    println!("{:?}", b_bw);
    println!("{:?}", draws_from_black);

    let wb: f64 = w_wb / draws_from_white as f64;
    let bw: f64 = b_bw / draws_from_black as f64;
    let ww: f64 = 1.0 - wb;
    let bb: f64 = 1.0 - bw;

    result.push(vec![ww, wb]);
    result.push(vec![bw, bb]);

    return result;
}

fn main() {
    let seq: Vec<char> = "WBWBWBWBBBBBWWBWB".chars().collect();
    let result = seq_to_transition_probability(seq);

    for i in 0..result.len() {
        println!("{:?}", result[i]);
    }
}
