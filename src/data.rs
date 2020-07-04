use crate::nn::Float;
use nom::IResult;
use std::collections::{HashMap, HashSet};

pub fn get_data(file: &str) -> Vec<Data> {
    let codes: Vec<ColorCode> = {
        use nom::{character::complete::newline, multi::separated_list0};
        separated_list0(newline, parse_color_code)(file).unwrap().1
    };

    let colors = codes.iter().map(|code| code.name).collect::<HashSet<_>>();

    let label_amount = colors.len() as u32;
    let mut map: HashMap<&str, u32> = HashMap::new();
    for (id, value) in colors.iter().enumerate() {
        map.insert(value, id as u32);
    }

    codes
        .into_iter()
        .map(|ColorCode { color, name }| Data {
            features: vec![
                color.0 as Float / 255.0,
                color.1 as Float / 255.0,
                color.2 as Float / 255.0,
            ],
            label_amount,
            label_id: map[name],
        })
        .collect()
}

#[derive(Debug)]
pub struct Data {
    pub features: Vec<Float>,
    label_amount: u32,
    label_id: u32,
}

impl Data {
    pub fn get_label_id(&self) -> u32 {
        self.label_id
    }

    pub fn get_labels(&self) -> Vec<Float> {
        let mut labels = vec![0.0; self.label_amount as usize];
        labels[self.label_id as usize] = 1.0;

        return labels;
    }
}

#[derive(Debug)]
struct ColorCode<'a> {
    color: Color,
    name: &'a str,
}

fn parse_color_code(i: &str) -> IResult<&str, ColorCode> {
    use nom::sequence::pair;

    let (rest, (color, name)) = pair(parse_color, parse_name)(i)?;

    return Ok((rest, ColorCode { color, name }));
}

#[derive(Debug)]
struct Color(u8, u8, u8);

fn parse_name(input: &str) -> IResult<&str, &str> {
    use nom::bytes::complete::take_while;
    use nom::character::complete::char;
    use nom::sequence::preceded;

    preceded(
        char(' '),
        take_while(|c: char| (c >= 'a' && c <= 'z') || c == ' '),
    )(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    use nom::{
        bytes::complete::tag,
        character::complete::char,
        combinator::map,
        sequence::{delimited, preceded, tuple},
    };

    map(
        delimited(
            char('['),
            tuple((
                parse_int,
                preceded(tag(", "), parse_int),
                preceded(tag(", "), parse_int),
            )),
            char(']'),
        ),
        |(r, g, b)| Color(r, g, b),
    )(input)
}

fn parse_int(i: &str) -> IResult<&str, u8> {
    use nom::{character::complete::digit1, combinator::map};

    map(digit1, |s: &str| u8::from_str_radix(s, 10).unwrap())(i)
}
