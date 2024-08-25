use std::collections::VecDeque;
use std::thread::sleep;
use pyo3::prelude::*;
use pyo3::types::PyList;

//
// #[derive(FromPyObject)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[pyclass(eq, eq_int)]
enum CellState {
    Dead = 0,
    Alive = 1,
}

#[pyclass]
#[derive(Copy, Clone)]
struct Cell {
    pos_x: u32,
    pos_y: u32,
    state: CellState,
}

#[pymethods]
impl Cell {
    fn __repr__(&self) -> String {
        (self.state as i8).to_string()
    }
}

#[pyclass]
struct Field {
    _field: Vec<Vec<Cell>>,
    _size: (i32, i32),
}

impl Field {
    /// get state of specified cell
    #[inline(always)]
    fn _get_state<'py>(&self, x: i32, y: i32) -> CellState {
        let x_size = self._size.0.clone();
        let y_size = self._size.1.clone();
        self._field[(x.rem_euclid(x_size)) as usize][y.rem_euclid(y_size) as usize].state.clone()
    }

    /// updates state of specified cell
    #[inline(always)]
    fn _set_state<'py>(&mut self, x: i32, y: i32, state: CellState) {
        let x_size = self._size.0.clone();
        let y_size = self._size.1.clone();
        self._field[(x.rem_euclid(x_size)) as usize][y.rem_euclid(y_size) as usize].state = state
    }

    /// get alive neighbors count of specified cell
    fn _neighbors(&self, x: i32, y: i32) -> i8 {
        let offsets: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];
        let mut alive_counter: i8 = 0;
        for offset in offsets.iter() {
            let state = self._get_state(x + offset.0, y + offset.1);
            if state as i8 == 1 {
                alive_counter += 1;
            }
        }

        return alive_counter;
    }

    /// game logic update
    fn _update<'py>(&mut self) {
        for i in 0..self._size.0 {
            for j in 0..self._size.1 {
                let alive_neighbors = self._neighbors(i, j);
                let cell_state = self._get_state(i, j);
                if cell_state == CellState::Dead {
                    if alive_neighbors == 3 {
                        self._set_state(i, j, CellState::Alive);
                    }
                } else if cell_state == CellState::Alive {
                    if 2 <= alive_neighbors && alive_neighbors <= 3 {
                        self._set_state(i, j, CellState::Alive);
                    } else {
                        self._set_state(i, j, CellState::Dead);
                    }
                }
            }
        }
    }
}


#[pyclass]
struct FieldIterator {
    field: Vec<Vec<Cell>>,
    range: VecDeque<(usize, usize)>,
}


impl FieldIterator {
    pub fn new(field: &Field) -> Self {
        let mut range: VecDeque<(usize, usize)> = VecDeque::with_capacity((field._size.0 * field._size.1) as usize);
        let mut field_copy: Vec<Vec<Cell>> = Vec::new();
        for i in 0..field._size.0 {
            for j in 0..field._size.1 {
                range.push_back((i as usize, j as usize));
            }
            field_copy.push(field._field[i as usize].clone())
        }

        return Self {
            field: field_copy,
            range,
        };
    }
}


impl Iterator for FieldIterator {
    type Item = (usize, usize, CellState);

    fn next(&mut self) -> Option<Self::Item> {
        match self.range.pop_front() {
            None => { None }
            Some(val) => { Option::Some((val.0.clone(), val.1.clone(), self.field[val.0.clone()][val.1.clone()].state)) }
        }
    }
}

#[pymethods]
impl FieldIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(usize, usize, CellState)> {
        slf.next()
    }
}


#[pymethods]
impl Field {
    #[new]
    #[pyo3(signature = (size, init_state = None))]
    fn new(size: (usize, usize), init_state: Option<Vec<Vec<Cell>>>) -> Self {
        fn _is_alive_on_start() -> CellState {
            if rand::random::<f32>() <= 0.1 {
                return CellState::Alive;
            }
            return CellState::Dead;
        }

        match init_state {
            None => {
                let mut _field: Vec<Vec<Cell>> = Vec::with_capacity(size.0);
                for i in 0..size.0 {
                    let mut row: Vec<Cell> = Vec::with_capacity(size.1);
                    for j in 0..size.1 {
                        row.push(Cell { pos_x: i as u32, pos_y: j as u32, state: _is_alive_on_start() })
                    }
                    _field.push(row);
                }
                Field { _field, _size: (size.0 as i32, size.1 as i32) }
            }
            Some(state) => {
                Field { _field: state, _size: (size.0 as i32, size.1 as i32) }
            }
        }
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<FieldIterator>> {
        Py::new(slf.py(), FieldIterator::new(&slf))
    }

    fn iterate(slf: PyRef<'_, Self>) -> PyResult<Py<FieldIterator>> {
        return Field::__iter__(slf);
    }

    fn copy(slf: PyRef<'_, Self>) -> Field {
        let mut _field: Vec<Vec<Cell>> = Vec::with_capacity(slf._size.0 as usize);
        for row in &slf._field {
            _field.push(row.clone());
        }
        Field {
            _size: slf._size,
            _field,
        }
    }

    /// game logic update
    fn update(&mut self) {
        self._update()
    }

    /// get state of specified cell
    fn get_state(&self, x: i32, y: i32) -> CellState {
        return self._get_state(x, y);
    }

    /// updates state of specified cell
    fn set_state(&mut self, x: i32, y: i32, state: CellState) {
        return self._set_state(x, y, state);
    }
}


/// renders field content on display
#[pyfunction]
fn render<'py>(
    screen: &Bound<'py, PyAny>,
    background: &Bound<'py, PyAny>,
    ellipse_func: &Bound<'py, PyAny>,
    field: &Field,
    size: (u32, u32),
    window_size: (u32, u32),
) {
    let full_cell_size: (usize, usize) = (window_size.1.div_euclid(size.0) as usize, window_size.0.div_euclid(size.1) as usize);
    const BORDER_SIZE: (usize, usize) = (2, 2);

    let cell_size: (usize, usize) = (full_cell_size.0 - BORDER_SIZE.0, full_cell_size.1 - BORDER_SIZE.1);

    screen.call_method("blit", (background, (0,0)), None).unwrap();
    for (x, y, cell_state) in FieldIterator::new(field) {
        let inner_rect = (
            x * cell_size.0 + (x * BORDER_SIZE.0), y * cell_size.1 + (y * BORDER_SIZE.1),
            cell_size.0, cell_size.1
        );
        if cell_state == CellState::Alive {
            ellipse_func.call((screen, (255, 255, 255), inner_rect), None).unwrap();
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn live_game(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Cell>()?;
    m.add_class::<CellState>()?;
    m.add_class::<Field>()?;
    m.add_class::<FieldIterator>()?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}
