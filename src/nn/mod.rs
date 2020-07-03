pub struct NeuralNetwork(Vec<Vec<Neuron>>);

pub type Layout = Vec<usize>;

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

    pub fn fromVector(weights: Vec<Float>, layers: Layout) -> NeuralNetwork {}
}

type Float = f32;

struct Neuron {
    weights: Vec<Float>,
    bias: Float,
}

impl Neuron {
    fn random(weight_count: usize) -> Neuron {
        let weights = (0..weight_count).map(|_| rand::random::<Float>()).collect();
        let bias = rand::random();

        Neuron { weights, bias }
    }
}
