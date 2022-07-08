use rand::prelude::SliceRandom;
use crate::{Green, Grey, Red, Yellow, Parameters};

pub fn create_block_result(params: &mut Parameters) -> String {
    let colours = get_colours(params);

    let mut result = "⠀\n".to_owned();
    result.push_str(&params.name);
    result.push_str("\n\n");

    if is_row_layout(params) {
        result = create_row_result(params, &colours, &mut result);
    } else if is_column_layout(params) {
        result = create_column_result(params, &colours, &mut result);
    } else {
        result = create_tabular_result(params, colours, &mut result);
    }

    result.push_str("\n\nhttps://github.com/metz-dev/bestle");
    result
}

fn is_row_layout(params: &mut Parameters) -> bool {
    params.height == 1
}

fn is_column_layout(params: &mut Parameters) -> bool {
    // no idea if that is even a thing
    params.width == 1
}

fn create_row_result(params: &mut Parameters, colours: &Vec<Colours>, result: &mut String) -> String {
    return create_single_line_result(params, colours, result, false);
}

fn create_column_result(params: &mut Parameters, colours: &Vec<Colours>, result: &mut String) -> String {
    return create_single_line_result(params, colours, result, true);
}

fn create_single_line_result(params: &mut Parameters, colours: &Vec<Colours>, result: &mut String, is_column: bool) -> String {
    // no early winning
    let not_green: Vec<Colours> = colours.iter().filter(|c| match c {
        Green => false,
        _ => true
    }).cloned().collect();

    if is_column {
        for _ in 1..params.height {
            result.push_str(random_block(&not_green));
            result.push_str("\n");
        }
    } else {
        for _ in 1..params.width {
            result.push_str(random_block(&not_green));
        }
    }

    if params.winning {
        result.push_str(Green.block());
    } else {
        result.push_str(random_block(&colours));
    }

    result.to_owned()
}

fn create_tabular_result(params: &mut Parameters, colours: Vec<Colours>, result: &mut String) -> String {
    for _ in 1..params.height {
        for _ in 1..=params.width {
            result.push_str(random_block(&colours));
        }
        result.push_str("\n");
    }

    if params.winning {
        for _ in 1..=params.width {
            result.push_str(Green.block());
        }
    } else {
        for _ in 1..=params.width {
            result.push_str(random_block(&colours));
        }
    }

    result.to_owned()
}

fn random_block(colours: &Vec<Colours>) -> &str {
    colours.choose(&mut rand::thread_rng()).unwrap().block()
}

fn get_colours(params: &mut Parameters) -> Vec<Colours> {
    let mut colours = Vec::new();
    colours.push(Green);
    if params.red {
        colours.push(Red);
    }
    if params.yellow {
        colours.push(Yellow);
    }
    if params.grey {
        colours.push(Grey);
    }
    colours
}

#[derive(PartialEq, Clone)]
pub enum Colours {
    Green,
    Red,
    Yellow,
    Grey
}

impl Colours {
    fn block(&self) -> &'static str {
        match self {
            Green => "🟩",
            Red => "🟥",
            Grey => "⬛",
            Yellow => "🟨"
        }
    }
}
