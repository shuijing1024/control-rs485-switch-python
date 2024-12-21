use crate::config::DEFAULT_MODBUS_TIMEOUT_MILLIS;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct USBSerialPortInfo {
    pub value: String,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub enum SwitchState {
    Open,
    Close,
    Lock,
    Unlock,
}

#[derive(Serialize, Deserialize)]
pub struct ModbusConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub slave_id: u8,
    pub timeout: u64,
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub(super) enum ReadSwitchState {
    Close = 0, // 分闸
    Open = 1,  // 合闸
    Lock = 2,  // 上锁
}

#[derive(IntoPrimitive)]
#[repr(u16)]
pub(super) enum WriteSwitchState {
    Open = 0x01,  // 合闸（也就是开关拨到“1”）
    Close = 0x02, // 分闸（开关拨到“0”）
    Lock = 0x03,
    Unlock = 0x04,
}

impl From<ReadSwitchState> for SwitchState {
    fn from(state: ReadSwitchState) -> Self {
        match state {
            ReadSwitchState::Close => SwitchState::Close,
            ReadSwitchState::Open => SwitchState::Open,
            ReadSwitchState::Lock => SwitchState::Lock,
        }
    }
}

impl Into<WriteSwitchState> for SwitchState {
    fn into(self) -> WriteSwitchState {
        match self {
            SwitchState::Open => WriteSwitchState::Open,
            SwitchState::Close => WriteSwitchState::Close,
            SwitchState::Lock => WriteSwitchState::Lock,
            SwitchState::Unlock => WriteSwitchState::Unlock,
        }
    }
}

impl Default for ModbusConfig {
    fn default() -> Self {
        Self {
            port_name: "COM7".to_string(),
            baud_rate: 4800,
            slave_id: 1,
            timeout: DEFAULT_MODBUS_TIMEOUT_MILLIS,
        }
    }
}
