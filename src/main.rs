// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;
// use pyo3::types::PyModule;
// ----------use upper three use or this:
use pyo3::{prelude::*, types::{PyModule}};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    execute_python().map_err(|err| println!("{:?}", err)).ok();
}

fn execute_python() -> PyResult<()> {
    Python::with_gil(|py| {
        let mut file = File::open("/home/wilson_yeh/wilson_workspace/success_pyo3/src/test.py").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        let activators = PyModule::from_code(py, &contents, "test.py", "test")?;

        let test: f64 = activators.getattr("sum")?.call1((5, 6))?.extract()?;
        println!("{}", test);

        Ok(())
    })
}