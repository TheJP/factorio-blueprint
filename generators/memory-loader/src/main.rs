use std::{fs, io};

use clap::{Arg, App};
use factorio_blueprint::{
    blueprint_string_to_model,
    abstract_model::{Entity, PoleType, Wire, Connector, Side, Blueprint},
    model,
    model_to_blueprint_string
};

const LOADER_BLUEPRINT: &str = "0eNqVk91qwzAMhd9F126Zs6bZDHuOXYwS8qO2gkQOjlwWSt59djxKGV233gRkW+ccf3LOUHceB0csYM5AjeURzMcZRjpw1cU1mQYEAyTYgwKu+li12FCLbtXYviauxDqYFRC3+AlGz+pPgWgkFctthWzeKUAWEsKUZymmkn1fowsWd4UUDHYMvZajf9BbZZvtOlcwgSn0Og9GLTls0olMRRFxtitrPFYnCgqhbU+doPuFxomc+LByiZFOrA4OkeNFGusjUl1cUdkt68zJeIxyOn5S09UtqU2pyDWeZClj9xy5/gCR3RvJLQ4PUvhWLcNeS5fYe3KjlP+m8p6IpEEtwwshh8otIQ28hQbrZfAPSOIJ3SRH4kPSHqZyQV7une1L4iAGRpzH+RHoiXJ4e8tjNVc/h4JgOCZSL3pTvGZFrnP9vH2a5y806h5c";
const CLOCK_BLUEPRINT: &str = "0eNqlk9tOwzAMht/F1xmiZQeIxHNwgVDVg7tZtEmVOhPV1HfHaTZUwdiGuImUOP78239ygKLx2DkyDPoAVFrTg349QE9bkzfhjIcOQQMxtqDA5G3YVVhShW5R2rYgk7N1MCogU+EH6GRUVwGhEOeGzxPS8U0BGiYmjHqmzZAZ3xbopMQlJQo620uqNaG84Bbpcq1gAL3e3K2kSkUOyxhPVZDCzjZZgbt8T5IvSUdqJrFqIvXhtCbXc/ajsz059nLypSneWJQ7LN9DV6dmJ90itMvdJFTDsyRZz53/A/YlIrtB1HnDWe1sm5ERBmh2HsdY0cQWJ+FJWLYO0cyHSVUURK70xNM2DdbNwmKE0NJb0xO5Ps4QJ7vSi76f80uM+ptjNTWM7pfHe2WU/mjNxfd7g8vfONMrpj4Lquu86fEfxsTJBl74Rnr2bRXspe84msdkuXlKN6tklTys78fxE0HyUOw=";

#[derive(Debug)]
struct Arguments {
    input_file: String,
    max_height: u32,
}

fn parse_arguments() -> Option<Arguments> {
    let matches = App::new("JP Factorio Generate Memory Loader")
        .version("0.1.0")
        .arg(Arg::new("input-file")
            .help("Binary file containing the data that should be loaded")
            .required(true))
        .arg(Arg::new("max-height")
            .help("The maximal loader height per column")
            .default_value("100")
            .required(false))
        .get_matches();

    match (matches.value_of("input-file"), matches.value_of_t::<u32>("max-height")) {
        (Some(file), Ok(max_height)) => Some(Arguments { max_height, input_file: file.into() }),
        _ => None,
    }
}

fn main() {
    let args = match parse_arguments() {
        Some(s) => s,
        None => {
            eprintln!("Invalid argument(s). Try --help for more information.");
            return;
        }
    };
    assert!(args.max_height >= 1);

    let data = read_data(&args.input_file).unwrap();
    if data.len() == 0 {
        eprintln!("Cannot generate loader for empty data file.");
        return;
    }

    let blueprint = generate_loader(args.max_height, &data);
    println!("{}", model_to_blueprint_string(blueprint).unwrap());
}

fn read_data(path: &str) -> io::Result<Vec<i32>> {
    let raw_data = fs::read(path)?;
    Ok(raw_data_to_i32(raw_data))
}

fn raw_data_to_i32(mut raw_data: Vec<u8>) -> Vec<i32> {
    let pad_by = (4 - raw_data.len() % 4) % 4;
    raw_data.append(&mut vec![0; pad_by]);
    assert_eq!(0, raw_data.len() % 4);

    let mut data = Vec::new();
    for i in (0..raw_data.len()).step_by(4) {
        let buffer = raw_data[i..i+4].try_into().expect("invalid length");
        let n = i32::from_be_bytes(buffer);
        data.push(n);
    }

    data
}

fn generate_loader(max_height: u32, data: &Vec<i32>) -> Blueprint {
    let mut blueprint = blueprint_string_to_model(LOADER_BLUEPRINT).unwrap();
    assert_eq!(2, blueprint.entities.len());

    let loader_ids: Vec<usize> = blueprint.entities.iter().map(|e| e.id()).collect();

    let (base_x, base_y) = blueprint.entities.iter().find_map(|e| match e {
        Entity::ConstantCombinator {
            position: model::Position { x, y },
            ..
        } => Some((*x, *y)),
        _ => None,
    }).unwrap();
    let (pole_base_x, pole_base_y) = (base_x + 3f32, base_y + 1f32);

    let mut y = 1;
    let mut x = 0;
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut pole_rows = Vec::new();
    let mut pole_row = Vec::new();
    for (i, &data_entry) in data.iter().enumerate() {
        let new_ids = blueprint.clone_entities(&loader_ids).unwrap();

        for &id in &new_ids {
            let entity = &mut blueprint.entities[id];

            let position = entity.position_mut();
            position.y += y as f32;
            position.x += (x as f32) * 4f32;

            match entity {
                Entity::ConstantCombinator { condition, .. } => {
                    assert_eq!(1, condition.len());
                    condition[0].count = data_entry;
                }
                Entity::DeciderCombinator { id, condition, .. } => {
                    condition.constant = Some((i + 1) as i32);
                    row.push(*id);
                }
                _ => {}
            }
        }

        // Add spaced out electric poles.
        // The reach of Medium Electric Poles is 7.
        if (y - 1) % 7 == 0 || y + 1 >= max_height {
            let electric_pole = Entity::ElectricPole {
                id: blueprint.entities.len(),
                position: model::Position {
                    x: pole_base_x + (x as f32) * 4f32,
                    y: pole_base_y + (y as f32) - 1f32,
                },

                pole_type: PoleType::Medium,
                connections: Vec::new(),
                neighbours: Vec::new(),
            };

            pole_row.push(electric_pole.id());
            blueprint.entities.push(electric_pole);
        }

        y += 1;
        if y >= max_height {
            y = 1;
            x += 1;

            rows.push(row);
            row = Vec::new();
            pole_rows.push(pole_row);
            pole_row = Vec::new();
        }
    }

    rows.push(row);
    pole_rows.push(pole_row);

    // Connect wires
    let mut connect_deciders = |id1: usize, id2: usize| {
        blueprint.connect_wire(id1, id2, Wire::Red).unwrap();
        blueprint.connect_wire_with_side(
            Connector { id: id1, side: Side::Two },
            Connector { id: id2, side: Side::Two },
            Wire::Green,
        ).unwrap();
    };

    let mut top_deciders = Vec::new();
    for row in &rows {
        let mut last_decider = row[0];
        top_deciders.push(last_decider);

        for &decider in &row[1..] {
            connect_deciders(last_decider, decider);
            last_decider = decider;
        }
    }

    let mut last_top_decider = top_deciders[0];
    for &decider in &top_deciders[1..] {
        connect_deciders(last_top_decider, decider);
        last_top_decider = decider;
    }

    // Connect electric poles
    let mut last_top_pole = pole_rows[0][0];
    for row in &pole_rows {
        let top_pole = row[0];
        let mut last_pole = top_pole;
        for &pole in &row[1..] {
            blueprint.connect_electric_poles(last_pole, pole).unwrap();
            last_pole = pole;
        }

        if last_top_pole != top_pole {
            blueprint.connect_electric_poles(last_top_pole, top_pole).unwrap();
            last_top_pole = top_pole;
        }
    }

    // Add clock at the top
    let clock = blueprint_string_to_model(CLOCK_BLUEPRINT).unwrap();
    assert_eq!(2, clock.entities.len());
    for mut entity in clock.entities {
        match entity {
            Entity::ConstantCombinator { .. } => {
                entity.update_id(0);
                let position = entity.position_mut();
                position.y = base_y;
                position.x = base_x;

                blueprint.entities[0] = entity;
            }
            Entity::DeciderCombinator { .. } => {
                entity.update_id(1);
                let position = entity.position_mut();
                position.y = base_y;
                position.x = base_x + 1.5f32;

                blueprint.entities[1] = entity;
            }
            _ => unreachable!()
        }
    }

    blueprint.connect_wire(0, 1, Wire::Green).unwrap();
    blueprint.connect_wire_with_side(
        Connector { id: 1, side: Side::Two },
        Connector { id: rows[0][0], side: Side::One },
        Wire::Red,
    ).unwrap();

    blueprint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_u8_vector_to_i32_vector() {
        assert_eq!(vec![5, 512, 7], raw_data_to_i32(vec![
            0, 0, 0, 5,
            0, 0, 2, 0,
            0, 0, 0, 7,
        ]));

        assert_eq!(vec![-1, i32::MAX, i32::MIN], raw_data_to_i32(vec![
            0xff, 0xff, 0xff, 0xff,
            0b0111_1111, 0xff, 0xff, 0xff,
            0b1000_0000, // 0, 0, 0,
        ]));
    }
}

