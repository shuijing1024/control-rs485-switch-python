pub(crate) use lib_switch_operate::prelude::{
    ModbusConfig as RustModbusConfig, SwitchState as RustSwitchState,
    USBSerialPortInfo as RustUSBSerialPortInfo,
};
use pyo3::prelude::*;

#[derive(IntoPyObject)]
pub(crate) struct PythonUSBSerialPortInfo {
    pub(crate) port_name: String,
    pub(crate) port_label: String,
}

#[derive(FromPyObject, IntoPyObject)]
pub(crate) struct PythonModbusConfig {
    pub(crate) port_name: String,
    pub(crate) baud_rate: u32,
    pub(crate) slave_id: u8,
    pub(crate) timeout: u64,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum PythonSwitchState {
    Open,
    Close,
    Lock,
    Unlock,
}

impl From<RustUSBSerialPortInfo> for PythonUSBSerialPortInfo {
    fn from(info: RustUSBSerialPortInfo) -> Self {
        Self {
            port_name: info.value,
            port_label: info.label,
        }
    }
}

impl From<RustModbusConfig> for PythonModbusConfig {
    fn from(config: RustModbusConfig) -> Self {
        Self {
            port_name: config.port_name,
            baud_rate: config.baud_rate,
            slave_id: config.slave_id,
            timeout: config.timeout,
        }
    }
}

impl From<PythonModbusConfig> for RustModbusConfig {
    fn from(value: PythonModbusConfig) -> Self {
        RustModbusConfig {
            port_name: value.port_name,
            baud_rate: value.baud_rate,
            slave_id: value.slave_id,
            timeout: value.timeout,
        }
    }
}

impl From<RustSwitchState> for PythonSwitchState {
    fn from(state: RustSwitchState) -> Self {
        match state {
            RustSwitchState::Open => PythonSwitchState::Open,
            RustSwitchState::Close => PythonSwitchState::Close,
            RustSwitchState::Lock => PythonSwitchState::Lock,
            RustSwitchState::Unlock => PythonSwitchState::Unlock,
        }
    }
}

impl From<PythonSwitchState> for RustSwitchState {
    fn from(value: PythonSwitchState) -> Self {
        match value {
            PythonSwitchState::Open => RustSwitchState::Open,
            PythonSwitchState::Close => RustSwitchState::Close,
            PythonSwitchState::Lock => RustSwitchState::Lock,
            PythonSwitchState::Unlock => RustSwitchState::Unlock,
        }
    }
}
