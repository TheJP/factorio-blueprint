use clap::{App, Arg};
use factorio_blueprint::{
    abstract_model::{Entity, Wire, Blueprint, utility},
    blueprint_string_to_model,
    model::{DeciderCondition, Signal},
    model_to_blueprint_string, Result,
};

const BLUEPRINT_MEMORY_CELL: &str = "0eNrNVsFu2zAM/RceB7uIZHvdDOwndtlhKAzHZlsClmTQUrAg8L+Psrc0SZvASYF1lwQSxcdHvgfCO1h3AXsm66HcATXODlD+3MFAT7bu4p3f9gglkEcDCdjaxFOLDbXIaePMmmztHcOYANkWf0GpxocE0HryhDPadNhWNpg1sjy4hJNA7wZJdTZWF7hU5wls5T/Td4VUaYmxmeM6AWHs2XXVGp/rDUm+JP1BrSTWTkhDvH0kHnz1qrENsQ9ys+c0v0hXsaM4D1/H4aziwfQ1TyRL+CYJLvg+XAH5xIh2hu23wi5YXz2yMxVZwYHSc8BxrmrnFifiKv7MuQfDpHYaZEPcBPLTUY/JUTg/DquTcHGa/SC1dSzG2J6W0qdYovESTvJwHA8K//WAvtID+iM88P3YA+qDPfCGLK8ccE6W7KK62RmVsj19gy0Fk2InpJiatHcdvqXT6q44UmphH3ppH/oqT59zX77vq2byzwa9tLRsCamFBnwBfp8H94YZMOJUL3ZMRXzXo7hx4gGfbrfje7aOuuishYIUN64D9S/XwY//fh3kN6yD5VtcwKbPgPLgqyGBDfIwj/6Lyu+/6vtCFSr7vBrH390R3ds=";

#[derive(Debug)]
struct MemorySize {
    width: u32,
    height: u32,
}

fn parse_arguments() -> Option<MemorySize> {
    let matches = App::new("JP Factorio Generate Memory")
        .version("0.1.0")
        .arg(Arg::new("width")
            .help("How many columns of memory are created")
            .required(true))
        .arg(Arg::new("height")
            .help("How many memory cells are created per column (i.e. how many rows of memory are created)")
            .required(true))
        .get_matches();

    match (matches.value_of_t::<u32>("width"), matches.value_of_t::<u32>("height")) {
        (Ok(width), Ok(height)) => Some(MemorySize { width, height }),
        _ => None,
    }
}

fn connect_all(blueprint: &mut Blueprint, id1: usize, id2: usize) -> utility::Result<()> {
    blueprint.connect_electric_poles(id1, id2)?;
    blueprint.connect_wire(id1, id2, Wire::Green)?;
    blueprint.connect_wire(id1, id2, Wire::Red)?;
    Ok(())
}

fn find_electric_pole(blueprint: &mut Blueprint, ids: &Vec<usize>) -> Option<usize> {
    for &id in ids {
        match blueprint.entities[id] {
            Entity::ElectricPole { .. } => {
                return Some(id)
            },
            _ => {}
        }
    }
    None
}

fn main() -> Result<()> {
    let size = match parse_arguments() {
        Some(s) => s,
        None => {
            eprintln!("Invalid argument(s). Try --help for more information.");
            return Ok(());
        }
    };

    let mut blueprint = blueprint_string_to_model(BLUEPRINT_MEMORY_CELL)?;
    let memory_cell_ids: Vec<usize> = blueprint.entities.iter().map(|e| e.id()).collect();

    let electric_pole_id = blueprint.entities.iter()
        .find_map(|e| {
            if let Entity::ElectricPole { id, .. } = e { Some(*id) }
            else { None }
        }).unwrap();

    let mut next_address = 2;
    // Update read and write address
    let mut assign_next_address = |blueprint: &mut Blueprint, ids: &Vec<usize>| {
        for &id in ids {
            if let Entity::DeciderCombinator { condition, .. } = &mut blueprint.entities[id] {
                match condition {
                    DeciderCondition {
                        constant: Some(ref mut value),
                        first_signal: Some(Signal { ref name, .. }),
                        ..
                    } if name == "signal-R" || name == "signal-W" => {
                        *value = next_address;
                    }
                    _ => {}
                }
            }
        }

        next_address += 1;
    };

    let mut connections = Vec::new();
    let mut last_top_row_electric_pole = electric_pole_id;

    for x in 0..size.width {
        let row_top_ids;
        if x == 0 {
            row_top_ids = memory_cell_ids.clone();
        } else {
            row_top_ids = blueprint.clone_entities(&memory_cell_ids).unwrap();
            assign_next_address(&mut blueprint, &row_top_ids);

            let electric_pole = find_electric_pole(&mut blueprint, &row_top_ids).unwrap();
            connections.push((last_top_row_electric_pole, electric_pole));
            last_top_row_electric_pole = electric_pole;

            // Translate entites to the right
            for &id in &row_top_ids {
                let position = blueprint.entities[id].position_mut();
                position.x += (x as f32) * 5f32;

            }
        }

        let mut last_electric_pole = last_top_row_electric_pole;

        for y in 1..size.height {
            let new_ids = blueprint.clone_entities(&row_top_ids).unwrap();
            assign_next_address(&mut blueprint, &new_ids);

            let electric_pole = find_electric_pole(&mut blueprint, &new_ids).unwrap();
            connections.push((last_electric_pole, electric_pole));
            last_electric_pole = electric_pole;

            // Translate entites down
            for &id in &new_ids {
                let position = blueprint.entities[id].position_mut();
                position.y += (y as f32) * 2f32;
            }
        }
    }

    for (from, to) in connections {
        connect_all(&mut blueprint, from, to).unwrap();
    }

    let blueprint = model_to_blueprint_string(blueprint)?;
    println!("{}", blueprint);

    Ok(())
}
