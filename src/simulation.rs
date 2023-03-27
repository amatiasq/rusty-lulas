pub mod cell;
pub mod vector;

use self::{cell::Cell, vector::Vector};

static mut CELLS: Option<Vec<Cell>> = None;

pub fn start(size: &Vector) {
    let mut cells = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        cells.push(Cell::new(&mut rng, size.x, size.y));
    }

    unsafe {
        CELLS = Some(cells);
    }

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

pub fn update<'a>() -> &'a Vec<Cell> {
    let cells = unsafe { CELLS.as_mut().unwrap() };

    for cell in cells.iter_mut() {
        cell.position.x += cell.velocity.x;
        cell.position.y += cell.velocity.y;
    }

    unsafe { CELLS.as_ref().unwrap() }
}
