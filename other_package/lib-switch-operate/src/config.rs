pub(crate) const DEFAULT_SERIAL_TIMEOUT_MILLIS: u64 = 1_000;
pub(crate) const DEFAULT_MODBUS_TIMEOUT_MILLIS: u64 = 6_000;
pub(crate) const READ_SWITCH_STATE_ADDRESS: u16 = 0x0026;
pub(crate) const WRITE_SWITCH_STATE_ADDRESS: u16 = 0x0101;
pub(crate) const WRITE_BAUD_RATE_ADDRESS: u16 = 0x0150;

pub(crate) const SWITCH_ID_GET_COMMAND: [u8; 2] = [0x03, 0xff];

pub(crate) const APP_CONFIG_DIR: &str = "control_rs485_switch";
pub(crate) const APP_CONTROLLER_CONFIG_FILE_NAME: &str = "switch_config.json";
