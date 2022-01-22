use factorio_blueprint::{blueprint_string_to_model, Result};

const BLUEPRINT_MEMORY_CELL: &str = "0eNrNVsFu2zAM/RceB7uIZHvdDOwndtlhKAzHZlsClmTQUrAg8L+Psrc0SZvASYF1lwQSxcdHvgfCO1h3AXsm66HcATXODlD+3MFAT7bu4p3f9gglkEcDCdjaxFOLDbXIaePMmmztHcOYANkWf0GpxocE0HryhDPadNhWNpg1sjy4hJNA7wZJdTZWF7hU5wls5T/Td4VUaYmxmeM6AWHs2XXVGp/rDUm+JP1BrSTWTkhDvH0kHnz1qrENsQ9ys+c0v0hXsaM4D1/H4aziwfQ1TyRL+CYJLvg+XAH5xIh2hu23wi5YXz2yMxVZwYHSc8BxrmrnFifiKv7MuQfDpHYaZEPcBPLTUY/JUTg/DquTcHGa/SC1dSzG2J6W0qdYovESTvJwHA8K//WAvtID+iM88P3YA+qDPfCGLK8ccE6W7KK62RmVsj19gy0Fk2InpJiatHcdvqXT6q44UmphH3ppH/oqT59zX77vq2byzwa9tLRsCamFBnwBfp8H94YZMOJUL3ZMRXzXo7hx4gGfbrfje7aOuuishYIUN64D9S/XwY//fh3kN6yD5VtcwKbPgPLgqyGBDfIwj/6Lyu+/6vtCFSr7vBrH390R3ds=";

fn main() -> Result<()> {
    let _memory_cell = blueprint_string_to_model(BLUEPRINT_MEMORY_CELL)?;
    Ok(())
}
