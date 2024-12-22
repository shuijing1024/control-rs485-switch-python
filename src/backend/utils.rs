use lib_switch_operate::prelude::AnyHowResult;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::PyResult;

pub(crate) trait MapAnyHowResultToPyResult<T> {
    fn map_to_py_result(self) -> PyResult<T>;
}

impl<T> MapAnyHowResultToPyResult<T> for AnyHowResult<T> {
    fn map_to_py_result(self) -> PyResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                let message = e.to_string().trim_start_matches("Error: ").to_string();
                Err(PyRuntimeError::new_err(message))
            }
        }
    }
}
