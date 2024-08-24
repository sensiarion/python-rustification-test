use pyo3::prelude::*;
use pyo3::types::PyList;
//
// #[derive(FromPyObject)]
// enum CellState {
//     Dead = 0,
//     Alive = 1,
// }
//
// #[derive(FromPyObject)]
// struct Cell {
//     pos_x: u32,
//     pos_y: u32,
//     state: CellState,
// }


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// getting field value
#[pyfunction]
#[inline]
fn get_from_field<'py>(x: i32, y: i32, field: &Bound<'py, PyList>, size: (i32, i32)) -> PyResult<Bound<'py, PyAny>> {
    Ok(field.get_item((x.rem_euclid(size.0)) as usize).unwrap().get_item((y.rem_euclid(size.1)) as usize).unwrap())
}

#[pyfunction]
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
        if (state == 1) {
            alive_counter += 1;
        }
    }


    return alive_counter;
}

// fn neighbors(array: Vec<Vec<&Cell>>, x: i32, y: i32) -> usize {
//     let offsets: [(i8, i8); 8] = [
//         (-1, -1), (-1, 0), (-1, 1),
//         (0, -1), (0, 1),
//         (1, -1), (1, 0), (1, 1),
//     ];
//     let alive_counter: u8 = 0;
//     for offset in offsets:
//         getter.
//     if self._get(x + offset[0], y + offset[1]).state == CellState.alive:
//         alive_counter += 1
//
//     return alive_counter;
// }

/// A Python module implemented in Rust.
#[pymodule]
fn live_game(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_field, m)?)?;
    m.add_function(wrap_pyfunction!(neighbors, m)?)?;
    Ok(())
}
