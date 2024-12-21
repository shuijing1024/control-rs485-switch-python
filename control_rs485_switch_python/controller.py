from dataclasses import dataclass
from enum import Enum

from .control_rs485_switch_python import PythonSwitchController,PythonSwitchState

@dataclass(frozen=True)
class USBSerialPortInfo:
    port_name:str
    port_label:str

@dataclass(frozen=True)
class ModbusConfig:
    port_name:str
    baud_rate:int=4800
    slave_id:int=1
    timeout:int=6000

class SwitchState(Enum):
    Close=0
    Open=1
    Lock=2
    Unlock=3


class SwitchController:
    def __init__(self,config:ModbusConfig):
        self.__controller=None
        self.__controller=PythonSwitchController(config)

    @staticmethod
    def get_usb_serial_port_list()->list[USBSerialPortInfo]:
        serialList=PythonSwitchController.get_usb_serial_port_list()
        return [USBSerialPortInfo(port_name=serial.port_name,port_label=serial.port_label) for serial in serialList]

    def custom_init(self,config:ModbusConfig)->int:
        return self.__controller.custom_init(config)

    def get_switch_state(self)->SwitchState:
        switch_state=self.__controller.get_switch_state()
        if switch_state==PythonSwitchState.Close:
            return SwitchState.Close
        elif switch_state==PythonSwitchState.Open:
            return SwitchState.Open
        elif switch_state==PythonSwitchState.Lock:
            return SwitchState.Lock
        elif switch_state==PythonSwitchState.Unlock:
            return SwitchState.Unlock
        else:
            raise RuntimeError("未知开关状态")

    def open_switch(self):
        switch_state=PythonSwitchState.Open
        self.__controller.operate_switch(switch_state)

    def close_switch(self):
        switch_state=PythonSwitchState.Close
        self.__controller.operate_switch(switch_state)

    def set_baud_rate(self, baud_rate:int):
        self.__controller.set_baud_rate(baud_rate)


    def __del__(self):
        if self.__controller is not None:
            self.__controller.disconnect()