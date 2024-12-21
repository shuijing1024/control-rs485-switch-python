use crate::config::*;
use crate::params::*;
use crate::utils::*;

use std::env;
use std::io::{Read, Write};
use std::path::Path;
use std::string::ToString;

use anyhow::Context;
use chrono::{DateTime, Local};
use tokio_modbus::client::Context as modbusContext;
use tokio_modbus::prelude::{rtu, Client, Reader, Writer};
use tokio_modbus::Result as ModbusResult;
use tokio_modbus::Slave;
use tokio_serial::SerialPortType::UsbPort;
use tokio_serial::{SerialPort, SerialPortInfo, SerialStream};

pub struct SwitchController {
    modbus_context: modbusContext,
    timeout: u64,
}

impl SwitchController {
    pub async fn operate_switch(&mut self, operation_state: SwitchState) -> AnyHowResult<()> {
        let operation_state: WriteSwitchState = operation_state.into();
        let action = self
            .modbus_context
            .write_single_register(WRITE_SWITCH_STATE_ADDRESS, operation_state.into());
        Self::modbus_action_with_timeout(action, "无法打开开关", self.timeout).await
    }

    pub async fn get_switch_state(&mut self) -> AnyHowResult<SwitchState> {
        let action = self
            .modbus_context
            .read_holding_registers(READ_SWITCH_STATE_ADDRESS, 2);
        let register_data_list =
            Self::modbus_action_with_timeout(action, "无法读取开关状态", self.timeout).await?;

        if register_data_list.len() != 2 {
            return Err(anyhow::anyhow!("开关状态数据获取失败"));
        }

        let register_data = (register_data_list[1] & 0x00ff) as u8;

        let state = ReadSwitchState::try_from(register_data).context("无法解析开关状态")?;

        Ok(SwitchState::from(state))
    }

    pub async fn set_baud_rate(&mut self, baud_rate: u32) -> AnyHowResult<()> {
        let baud_rate_register_data_high = ((baud_rate & 0xffff0000) >> 16) as u16;
        let baud_rate_register_data_low = (baud_rate & 0x0000ffff) as u16;
        let baud_rate_register_data = [baud_rate_register_data_high, baud_rate_register_data_low];

        let action = self
            .modbus_context
            .write_multiple_registers(WRITE_BAUD_RATE_ADDRESS, &baud_rate_register_data);

        Self::modbus_action_with_timeout(action, "无法更改波特率", 1000).await
    }

    pub async fn disconnect(&mut self) -> AnyHowResult<()> {
        self.modbus_context
            .disconnect()
            .await
            .context("无法断开连接")
    }
}

impl SwitchController {
    pub fn new(modbus_config: ModbusConfig) -> AnyHowResult<Self> {
        let serial_builder = tokio_serial::new(&modbus_config.port_name, modbus_config.baud_rate)
            .timeout(std::time::Duration::from_millis(
                DEFAULT_SERIAL_TIMEOUT_MILLIS,
            ));

        let serial_port = SerialStream::open(&serial_builder)
            .context(format!("无法打开串口: {}", modbus_config.port_name))?;

        let slave = Slave(modbus_config.slave_id);

        let modbus_context = rtu::attach_slave(serial_port, slave);

        Ok(Self {
            modbus_context,
            timeout: modbus_config.timeout,
        })
    }

    pub fn get_usb_serial_port_list() -> AnyHowResult<Vec<USBSerialPortInfo>> {
        let serial_list = tokio_serial::available_ports().context("无法获得串口列表")?;

        let mut usb_serial_port_list = Vec::new();

        for port in serial_list {
            if let SerialPortInfo {
                port_name,
                port_type: UsbPort(usb_port_info),
            } = port
            {
                let usb_serial_label = if let Some(product_label) = usb_port_info.product {
                    if product_label.contains(&port_name) {
                        product_label
                    } else {
                        format!("({}) - {}", &port_name, product_label)
                    }
                } else {
                    port_name.clone()
                };

                let info = USBSerialPortInfo {
                    value: port_name,
                    label: usb_serial_label,
                };

                usb_serial_port_list.push(info);
            }
        }

        Ok(usb_serial_port_list)
    }

    pub async fn get_app_config() -> AnyHowResult<ModbusConfig> {
        let config_dir = get_config_dir();
        let app_config_file_path = Path::new(&config_dir)
            .join(APP_CONFIG_DIR)
            .join(APP_CONTROLLER_CONFIG_FILE_NAME);

        if !app_config_file_path.exists() {
            return Ok(ModbusConfig::default());
        }

        let config_str = tokio::fs::read_to_string(app_config_file_path)
            .await
            .context("无法读取配置文件")?;

        let config: ModbusConfig = serde_json::from_str(&config_str).context("无法解析配置文件")?;

        Ok(config)
    }

    pub async fn set_app_config(modbus_config: ModbusConfig) -> AnyHowResult<()> {
        let config_dir = get_config_dir();
        let app_config_dir = Path::new(&config_dir).join(APP_CONFIG_DIR);

        if !app_config_dir.exists() {
            tokio::fs::create_dir(&app_config_dir)
                .await
                .context("无法创建配置目录")?;
        }

        let app_config_file_path = app_config_dir.join(APP_CONTROLLER_CONFIG_FILE_NAME);
        let config_str =
            serde_json::to_string_pretty(&modbus_config).context("无法序列化配置文件")?;

        tokio::fs::write(app_config_file_path, config_str)
            .await
            .context("无法写入配置文件")?;

        Ok(())
    }

    pub async fn custom_init(modbus_config: ModbusConfig) -> AnyHowResult<u8> {
        let mut serial_port = tokio_serial::new(&modbus_config.port_name, modbus_config.baud_rate)
            .timeout(std::time::Duration::from_millis(
                DEFAULT_SERIAL_TIMEOUT_MILLIS,
            ))
            .open()
            .context(format!("无法打开串口: {}", modbus_config.port_name))?;

        let mut read_byte_list = Self::get_switch_id(&mut serial_port, 3)?;

        if read_byte_list.len() == 1 {
            return Ok(read_byte_list[0]);
        }

        if read_byte_list.len() != 10 {
            return Err(anyhow::anyhow!("接收到错误ID"));
        }

        read_byte_list.extend_from_slice(&[0x06, 0x09, 0x00]);

        serial_port
            .write_all(&read_byte_list)
            .context("无法写入更改协议命令")?;

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let read_byte_list = Self::get_switch_id(&mut serial_port, 3)?;

        if read_byte_list.len() == 1 {
            Ok(read_byte_list[0])
        } else {
            Err(anyhow::anyhow!("接收到错误ID"))
        }
    }
}

impl SwitchController {
    async fn modbus_action_with_timeout<F, T>(
        action: F,
        description_message: &str,
        time_out_millis: u64,
    ) -> AnyHowResult<T>
    where
        F: std::future::Future<Output = ModbusResult<T>>,
    {
        tokio::select! {
             result = action => {
                if let Ok(Ok(need_message))=result{
                    Ok(need_message)
                }else{
                    Err(anyhow::anyhow!("{}",description_message))
                }
            },
            _ = tokio::time::sleep(std::time::Duration::from_millis(time_out_millis))=>{
                let now: DateTime<Local> = Local::now();
                let now_string =  now.format("%Y-%m-%d %H:%M:%S%.3f");
                Err(anyhow::anyhow!("{}\n当前时间为:{}","modbus操作超时",now_string))},
        }
    }

    fn get_switch_id(
        serial_port: &mut Box<dyn SerialPort>,
        read_time: u8,
    ) -> AnyHowResult<Vec<u8>> {
        let mut read_buffer: [u8; 10] = [0; 10];
        let mut read_byte_list = Vec::with_capacity(14);

        serial_port
            .write_all(&SWITCH_ID_GET_COMMAND)
            .context("无法写入读取ID命令")?;

        let read_start = Local::now().timestamp();

        while (Local::now().timestamp() - read_start) < read_time as i64 {
            let read_result = serial_port.read(&mut read_buffer);

            if let Ok(read_byte_number) = read_result {
                read_byte_list.extend_from_slice(&read_buffer[0..read_byte_number]);
            }
        }

        Ok(read_byte_list)
    }
}

#[cfg(target_os = "windows")]
fn get_config_dir() -> String {
    env::var("APPDATA").unwrap_or("./".to_string())
}

#[cfg(target_os = "linux")]
fn get_config_dir() -> String {
    format!("{}/.config", env::var("HOME").unwrap_or("./".to_string()))
}

#[cfg(target_os = "macos")]
fn get_config_dir() -> String {
    format!(
        "{}/Library/Application Support",
        env::var("HOME").unwrap_or("./".to_string())
    )
}
