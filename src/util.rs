use super::nn::Float;
pub struct RandomIter<'a, T> {
    source: &'a [T],
}

impl<'a, T> RandomIter<'a, T> {
    pub fn new(source: &'a [T]) -> Self {
        RandomIter { source }
    }
}

impl<'a, T> Iterator for RandomIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.source.len() == 0 {
            return None;
        }
        let index = (rand::random::<f64>() * self.source.len() as f64) as usize;

        Some(&self.source[index])
    }
}
//
// Numeric derivation
pub fn derive(f: impl Fn(Vec<Float>) -> Float) -> impl Fn(Vec<Float>) -> Vec<Float> {
    // small distance, not optimal probably
    const H: Float = 0.00001;

    move |weights: Vec<Float>| {
        let base = f(weights.clone());

        // TODO PERFORMANCE do this in parallel
        (0..weights.len())
            .map(|weight_index| {
                let mut w = weights.clone();
                w[weight_index] += H;

                (base - f(w)) / H
            })
            .collect()
    }
}

pub fn add_vec(a: &[Float], b: &[Float]) -> Vec<Float> {
    a.iter().zip(b.iter()).map(|(a, b)| a + b).collect()
}
