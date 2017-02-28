pub fn calculate(data: Vec<u8>) -> f64 {
    let mut frequency = vec![0.0_f64; 256];
    let mut probabilities = vec![0.0_f64; 256];
    let mut entropy_sum = 0.0_f64;
    
    unsafe {
        frequency.set_len(256);
        probabilities.set_len(256);
    }

    for i in 0..data.len() {
        frequency[data[i] as usize] = (frequency[data[i] as usize] + 1.0_f64) as f64;
    }

    for i in 0..256 {
        if frequency[i] == 0.0_f64 { continue; }
        probabilities[i] = frequency[i] / ((data.len() as f64));
        entropy_sum += probabilities[i] * probabilities[i].log(2.0_f64);
    }

    return -1.0 * entropy_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test1() {
        let data = vec![0,1,2,3,4,5];
        assert_eq!(calculate(data), 2.5849626_f64);
    }
}
