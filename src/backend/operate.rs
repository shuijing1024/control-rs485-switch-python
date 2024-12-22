use super::params::*;
use super::utils::*;

use lib_switch_operate::prelude::SwitchController as RustSwitchController;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

#[pyclass]
pub(crate) struct PythonSwitchController {
    runtime: Runtime,
    controller: RustSwitchController,
}

unsafe impl Sync for PythonSwitchController {}

#[pymethods]
impl PythonSwitchController {
    #[new]
    fn new(config: PythonModbusConfig) -> PyResult<Self> {
        let runtime = Runtime::new().map_err(|_e| PyRuntimeError::new_err("创建运行环境失败"))?;
        let controller = RustSwitchController::new(config.into()).map_to_py_result()?;

        Ok(Self {
            runtime,
            controller,
        })
    }

    #[staticmethod]
    fn get_usb_serial_port_list() -> PyResult<Vec<PythonUSBSerialPortInfo>> {
        let infos: Vec<_> = RustSwitchController::get_usb_serial_port_list()
            .map_to_py_result()?
            .into_iter()
            .map(PythonUSBSerialPortInfo::from)
            .collect();

        Ok(infos)
    }

    fn custom_init(&self, modbus_config: PythonModbusConfig) -> PyResult<u8> {
        let action = RustSwitchController::custom_init(RustModbusConfig::from(modbus_config));

        self.runtime.block_on(action).map_to_py_result()
    }

    fn get_switch_state(&mut self) -> PyResult<PythonSwitchState> {
        let action = self.controller.get_switch_state();

        let state = self.runtime.block_on(action).map_to_py_result()?;

        Ok(PythonSwitchState::from(state))
    }

    fn operate_switch(&mut self, operation_state: &PythonSwitchState) -> PyResult<()> {
        let custom_state: RustSwitchState = RustSwitchState::from(operation_state.clone());
        let action = self.controller.operate_switch(custom_state);

        self.runtime.block_on(action).map_to_py_result()
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> PyResult<()> {
        let action = self.controller.set_baud_rate(baud_rate);

        self.runtime.block_on(action).map_to_py_result()
    }

    fn disconnect(&mut self) -> PyResult<()> {
        let action = self.controller.disconnect();

        self.runtime.block_on(action).map_to_py_result()
    }
}
