pub mod cell;
pub mod vector;

use self::{cell::Cell, vector::Vector};

pub fn start(size: &Vector) -> Vec<Cell> {
    let mut rng = rand::thread_rng();
    let mut cells = Vec::new();

    for _ in 0..100 {
        cells.push(Cell::new(&mut rng, size.x, size.y));
    }

    cells

    // for _ in 0..100 {
    //     for cell in cells.iter_mut() {
    //         cell.position.x += cell.velocity.x;
    //         cell.position.y += cell.velocity.y;
    //     }
    // }

    // for cell in cells.iter() {
    //     println!("Cell at ({}, {})", cell.position.x, cell.position.y);
    // }
}

pub fn update(cells: &mut Vec<Cell>) {
    for cell in cells.iter_mut() {
        cell.position.x += cell.velocity.x;
        cell.position.y += cell.velocity.y;
    }
}
