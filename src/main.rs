mod data;
mod nn;
mod util;

fn main() {
    let file = include_str!("../input");
    let data = data::get_data(file);

    let network = nn::NeuralNetwork::random(vec![3, 27]);

    data.iter().take(10).for_each(|d: &data::Data| {
        // each datapoint gets processed here
        let labels = d.get_labels();

        let result = network.compute(d.features.clone());

        println!("\ngot    | expected ");

        result
            .iter()
            .zip(labels.iter())
            .for_each(|(got, expected)| println!("{:.3}  |  {:.3}", got, expected))
    });
}
