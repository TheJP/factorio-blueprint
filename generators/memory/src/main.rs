use factorio_blueprint::{
    abstract_model::{Entity, Wire},
    blueprint_string_to_model,
    model::{DeciderCondition, Signal},
    model_to_blueprint_string, Result,
};

const BLUEPRINT_MEMORY_CELL: &str = "0eNrNVsFu2zAM/RceB7uIZHvdDOwndtlhKAzHZlsClmTQUrAg8L+Psrc0SZvASYF1lwQSxcdHvgfCO1h3AXsm66HcATXODlD+3MFAT7bu4p3f9gglkEcDCdjaxFOLDbXIaePMmmztHcOYANkWf0GpxocE0HryhDPadNhWNpg1sjy4hJNA7wZJdTZWF7hU5wls5T/Td4VUaYmxmeM6AWHs2XXVGp/rDUm+JP1BrSTWTkhDvH0kHnz1qrENsQ9ys+c0v0hXsaM4D1/H4aziwfQ1TyRL+CYJLvg+XAH5xIh2hu23wi5YXz2yMxVZwYHSc8BxrmrnFifiKv7MuQfDpHYaZEPcBPLTUY/JUTg/DquTcHGa/SC1dSzG2J6W0qdYovESTvJwHA8K//WAvtID+iM88P3YA+qDPfCGLK8ccE6W7KK62RmVsj19gy0Fk2InpJiatHcdvqXT6q44UmphH3ppH/oqT59zX77vq2byzwa9tLRsCamFBnwBfp8H94YZMOJUL3ZMRXzXo7hx4gGfbrfje7aOuuishYIUN64D9S/XwY//fh3kN6yD5VtcwKbPgPLgqyGBDfIwj/6Lyu+/6vtCFSr7vBrH390R3ds=";

fn main() -> Result<()> {
    let mut blueprint = blueprint_string_to_model(BLUEPRINT_MEMORY_CELL)?;
    let memory_cell_ids: Vec<usize> = blueprint.entities.iter().map(|e| e.id()).collect();

    let mut last_electric_pole = blueprint.entities.iter()
        .find_map(|e| {
            if let Entity::ElectricPole { id, .. } = e { Some(*id) }
            else { None }
        }).unwrap();

    for i in 1..=9 {
        let new_ids = blueprint.clone_entities(&memory_cell_ids).unwrap();

        for &id in &new_ids {
            // Translate entites down
            let position = blueprint.entities[id].position_mut();
            position.y += (i as f32) * 2f32;

            match &mut blueprint.entities[id] {
                // Update read and write address
                Entity::DeciderCombinator { condition, .. } =>
                    match condition {
                        DeciderCondition {
                            constant: Some(ref mut address),
                            first_signal: Some(Signal { ref name, .. }),
                            ..
                        } if name == "signal-R" || name == "signal-W" =>
                            *address = i + 1,
                        _ => {}
                },

                // Connect and wire electric poles
                Entity::ElectricPole { id, .. } => {
                    let id = *id;
                    blueprint.connect_electric_poles(last_electric_pole, id).unwrap();
                    blueprint.connect_wire(last_electric_pole, id, Wire::Green).unwrap();
                    blueprint.connect_wire(last_electric_pole, id, Wire::Red).unwrap();
                    last_electric_pole = id;
                },
                _ => {}
            }
        }

    }

    let blueprint = model_to_blueprint_string(blueprint)?;
    println!("{}", blueprint);

    Ok(())
}
