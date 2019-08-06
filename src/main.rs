#[derive(Copy, Clone)]
struct Cell {
    value: u8,
    visible: bool,
}

type Field = [[Cell; 8]; 8];

fn main() {
    let mut field: Field = [[Cell {
        value: 0,
        visible: false,
    }; 8]; 8];
    field[4][3].value = 1;
    render_field(&field);
}

fn render_field(field: &Field) {
    let displayable_field: Vec<Vec<char>> = field
        .iter()
        .map(|line| {
            line.iter()
                .map(|cell| if cell.value == 1 { 'X' } else { '.' })
                .collect()
        })
        .collect();
    for line in displayable_field {
        println!("{:?}", line);
    }
}