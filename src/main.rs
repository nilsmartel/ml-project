mod data;

fn main() {
    let file = include_str!("../input");
    let data = data::get_data(file);

    dbg!(data);
}