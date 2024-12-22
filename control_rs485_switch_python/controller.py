from dataclasses import dataclass
from enum import Enum

from .control_rs485_switch_python import PythonSwitchController,PythonSwitchState

@dataclass(frozen=True)
class USBSerialPortInfo:
    """
    USB串口信息。
    """

    port_name:str   # 串口名
    port_label:str  # 串口标签，例如Windows上会显示CH340。如果没有的话就和串口名一致。

@dataclass(frozen=True)
class ModbusConfig:
    """
    Modbus配置
    """

    port_name:str   #串口名
    baud_rate:int=4800  # 波特率
    slave_id:int=1  # 从机设备ID
    timeout:int=6000    # Modbus操作超时时间，单位为毫秒

class SwitchState(Enum):
    """
    开关状态
    """

    Close=0 # 分闸
    Open=1  # 合闸
    Lock=2  # 锁定
    Unlock=3    # 解锁。在读取开关情况下，不会返回此状态。


class SwitchController:
    """
    这是控制开关操作的Python类。\n
    为了简化操作，请不要将该控制类实例于多线程中使用，可能会出现未知情况。\n
    当建立类实例时即为连接上通信串口。\n
    类中所有方法都有可能返回RuntimeError，其中参数是字符串，描述了错误原因。
    """

    def __init__(self, config: ModbusConfig):
        """
        初始化。会打开通信串口。
        :param config: Modbus配置
        """

        self.__controller=None
        self.__controller=PythonSwitchController(config)

    @staticmethod
    def get_usb_serial_port_list()->list[USBSerialPortInfo]:
        """
        获得USB串口列表
        :return: USB串口列表
        """

        serialList=PythonSwitchController.get_usb_serial_port_list()
        return [USBSerialPortInfo(port_name=serial.port_name,port_label=serial.port_label) for serial in serialList]

    def custom_init(self,config:ModbusConfig)->int:
        """
        初始化。\n
        假如开关不是Modbus协议，切换为Modbus协议，并返回Modbus协议下的设备ID
        :return: 设备ID
        """

        return self.__controller.custom_init(config)

    def get_switch_state(self)->SwitchState:
        """
        获得开关状态
        :return: 开关状态
        """

        switch_state= self.__controller.get_switch_state
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

    def open_switch(self)->None:
        """
        合闸
        """

        switch_state=PythonSwitchState.Open
        self.__controller.operate_switch(switch_state)

    def close_switch(self)->None:
        """
        分闸
        """

        switch_state=PythonSwitchState.Close
        self.__controller.operate_switch(switch_state)

    def set_baud_rate(self, baud_rate:int)->None:
        """
        设置波特率。就目前来看，波特率最高能有115200，最低4800。刚到手时默认115200。
        :param baud_rate: 要设置的波特率
        """

        self.__controller.set_baud_rate(baud_rate)


    def __del__(self):
        """
        在此控制器删除时进行一些清理操作
        """

        if self.__controller is not None:
            self.__controller.disconnect()