use pyo3::prelude::*;
use pyo3::types::PyList;
//
// #[derive(FromPyObject)]
enum CellState {
    Dead = 0,
    Alive = 1,
}
//
// #[derive(FromPyObject)]
// struct Cell {
//     pos_x: u32,
//     pos_y: u32,
//     state: CellState,
// }


/// getting field value
#[pyfunction]
#[inline(always)]
fn get_from_field<'py>(x: i32, y: i32, field: &Bound<'py, PyList>, size: (i32, i32)) -> PyResult<Bound<'py, PyAny>> {
    Ok(field.get_item((x.rem_euclid(size.0)) as usize).unwrap().get_item((y.rem_euclid(size.1)) as usize).unwrap())
}

/// found count of alive neihbors on specified cell position
#[pyfunction]
#[inline(always)]
fn neighbors<'py>(x: i32, y: i32, field: &Bound<'py, PyList>, size: (i32, i32)) -> i8 {
    let offsets: [(i8, i8); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    let mut alive_counter: i8 = 0;
    for offset in offsets.iter() {
        let field = get_from_field(x + offset.0 as i32, y + offset.1 as i32, field, size).unwrap();
        let state: i8 = field.getattr("state").unwrap().extract().unwrap();
        if state == 1 {
            alive_counter += 1;
        }
    }


    return alive_counter;
}

//
// for x, y, cell in self.iterate():
//     alive_neighbours = self._field._neighbors(x, y)
//     if cell.state == CellState.dead:
//         if alive_neighbours == 3:
//             cell.state = CellState.alive
//     elif cell.state == CellState.alive:
//         if 2 <= alive_neighbours <= 3:
//             cell.state = CellState.alive
//         else:
//             cell.state = CellState.dead

#[pyfunction]
fn update<'py>(field: &Bound<'py, PyList>, size: (i32, i32)) {
    for i in 0..size.0 {
        for j in 0..size.1 {
            let alive_neighbors = neighbors(i, j, field, size);
            let cell = get_from_field(i, j, field, size).unwrap();
            let cell_state: i8 = cell.getattr("state").unwrap().extract().unwrap();
            if cell_state == CellState::Dead as i8 {
                if alive_neighbors == 3 {
                    cell.setattr("state", CellState::Alive as i8).unwrap();
                }
            } else if cell_state == CellState::Alive as i8 {
                if 2 <= alive_neighbors && alive_neighbors <= 3 {
                    cell.setattr("state", CellState::Alive as i8).unwrap();
                } else {
                    cell.setattr("state", CellState::Dead as i8).unwrap();
                }
            }
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn live_game(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_from_field, m)?)?;
    m.add_function(wrap_pyfunction!(neighbors, m)?)?;
    m.add_function(wrap_pyfunction!(update, m)?)?;
    Ok(())
}
