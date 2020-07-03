pub struct NeuralNetwork(Vec<Vec<Neuron>>);

pub type Layout = Vec<usize>;

fn amount_of_weights(l: &Layout) -> usize {
    l.iter()
        .zip(l[1..].iter())
        .map(|(n1, n2)| (n1 + 1) * n2)
        .sum()
}

impl NeuralNetwork {
    pub fn random(layers: Layout) -> NeuralNetwork {
        let layers = layers
            .iter()
            .zip(layers[1..].iter())
            .map(|(inp, out)| {
                (0usize..*out)
                    .map(|_| Neuron::random(*inp))
                    .collect::<Vec<Neuron>>()
            })
            .collect();

        NeuralNetwork(layers)
    }

    pub fn fromVector(weights: &[Float], layers: Layout) -> NeuralNetwork {
        // assert that weights has the proper length
        assert_eq!(weights.len(), amount_of_weights(&layers));

        let info =
            layers
                .iter()
                .zip(layers[1..].iter())
                .map(|(prev, current): (&usize, &usize)| {
                    // `prev` states the amount of neurons in the previous layer
                    // while `current` is the amount of neurons desired for the current one.

                    // returns the needed amount of weights for this specific layer
                    ((*prev + 1), current)
                });

        let mut acc = 0;
        let mut layers: Vec<Vec<Neuron>> = Vec::new();
        for (needed, amount) in info {
            let mut layer = Vec::new();
            for _ in 0..*amount {
                layer.push(Neuron::fromVector(&weights[acc..acc + needed]));

                acc += needed;
            }

            layers.push(layer);
        }

        NeuralNetwork(layers)
    }

    pub fn toVector(self) -> Vec<Float> {
        self.0.into_iter().fold(Vec::new(), |vec, layer| {
            layer.into_iter().fold(vec, |mut vec, neuron| {
                vec.extend(neuron.toVector());
                vec
            })
        })
    }
}

type Float = f32;

#[derive(Clone, Debug, PartialEq)]
struct Neuron {
    weights: Vec<Float>,
    bias: Float,
}

impl Eq for Neuron {}

impl Neuron {
    fn random(weight_count: usize) -> Neuron {
        let weights = (0..weight_count).map(|_| rand::random::<Float>()).collect();
        let bias = rand::random();

        Neuron { weights, bias }
    }

    fn fromVector(v: &[Float]) -> Neuron {
        let weights = v[..v.len() - 1].to_vec();
        let bias = v[v.len() - 1];

        Neuron { weights, bias }
    }

    fn toVector(mut self) -> Vec<Float> {
        self.weights.push(self.bias);
        self.weights
    }

    /// Compute the tanh of dot product + bias
    fn compute(&self, values: &[Float]) -> Float {
        (self
            .weights
            .iter()
            .zip(values.iter())
            .map(|(a, b)| a * b)
            .sum::<Float>()
            + self.bias)
            .tanh()
    }
}

#[cfg(test)]
mod test {
    use super::Neuron;
    #[test]
    fn test_neuron() {
        let neuron = Neuron::random(3);

        assert_eq!(neuron.clone(), Neuron::fromVector(&neuron.toVector()))
    }
}
