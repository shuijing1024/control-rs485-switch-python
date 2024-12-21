mod backend;

use pyo3::prelude::*;

use backend::prelude::*;

#[pymodule]
fn control_rs485_switch_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonSwitchController>()?;
    m.add_class::<PythonSwitchState>()?;
    Ok(())
}
