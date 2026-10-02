use pyo3::prelude::*;

mod grid;

#[pymodule]
mod _core {
    use pyo3::prelude::*;

    #[pyfunction]
    fn hello_from_bin() -> String {
        "Hello from rustlife!".to_string()
    }
}
