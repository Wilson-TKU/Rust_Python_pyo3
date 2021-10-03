// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;
// use pyo3::types::PyModule;
// ----------use upper three use or this:
use pyo3::{prelude::*, types::{IntoPyDict, PyModule}};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
            let activators = PyModule::from_code(py, r#"
def relu(x):
    return max(0.0, x)
def hello_world(a, b):
    print("hello_world:")
    return (a+b)
                "#, "activators.py", "activators")?;

            let test: f64 = activators.getattr("hello_world")?.call1((5, 6))?.extract()?;
            println!("{}", test);

            Ok(())
        })
}