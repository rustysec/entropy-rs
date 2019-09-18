use super::Entropy;

/// Implementation of Shannon entropy
pub struct Shannon {
    frequency: Vec<f64>,
    probabilities: Vec<f64>,
    entropy: Option<f64>,
    data_len: usize,
}

impl Shannon {
    pub fn new() -> Self {
        Self {
            frequency: vec![0.0; 256],
            probabilities: vec![0.0; 256],
            entropy: None,
            data_len: 0,
        }
    }

    /// simple method to immediately return entropy for a blob of data
    ///
    /// # Examples
    ///
    /// ```
    /// use entropy_rs::Shannon;
    /// let data = vec![0,1,2,3,4,5];
    /// assert_eq!(2.584962500721156, Shannon::quick(&data));
    ///
    /// ```
    ///
    pub fn quick<T: AsRef<[u8]>>(data: T) -> f64 {
        let mut quick = Self::new();
        quick.input(data);
        quick.calculate()
    }
}

impl Entropy for Shannon {
    fn input<T: AsRef<[u8]>>(&mut self, data: T) {
        self.data_len += data.as_ref().len();
        data.as_ref().into_iter().for_each(|v| {
            self.frequency[*v as usize] += 1.0_f64;
        });
    }

    fn calculate(&mut self) -> f64 {
        if let Some(entropy) = self.entropy {
            entropy
        } else {
            let mut entropy = 0.0;
            for i in 0..256 {
                if self.frequency[i] != 0.0 {
                    self.probabilities[i] = self.frequency[i] / self.data_len as f64;
                    entropy += self.probabilities[i] * self.probabilities[i].log(2.0_f64);
                }
            }
            entropy *= -1.0;
            self.entropy = Some(entropy);
            entropy
        }
    }

    fn reset(&mut self) {
        *self = Shannon::new();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shannon_test_quick() {
        use crate::Shannon;
        assert_eq!(Shannon::quick(vec![0, 1, 2, 3, 4, 5]) as f32, 2.5849626_f32);
    }

}
