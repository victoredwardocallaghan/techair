// Copyright (C) 2020, Edward O'Callaghan.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use std::io;
use std::io::{Error, ErrorKind};
use std::convert::TryFrom;

use crate::cmd::math::*;
use crate::encoder::TechAirEncoder;


#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum MeasureCmd {
    GetLogicVoltage(f32),
    GetPeripheralVoltage(f32),
    GetRightHandVoltage(f32),
    GetLeftHandVoltage(f32),
    GetRightFootVoltage(f32),
    GetLeftFootVoltage(f32),
    GetSquibVoltage(f32),
    GetBatteryVoltage(f32),
    GetChargingState(Option<String>),
    GetZIPSwitchState(bool),
    SetEXTDisplay(u8), // USBSetLEDs()
}

fn as_u16(data: Vec<u8>) -> u16 {
    return ((data[0] as u16) << 8) | data[1] as u16;
}

fn decode_voltage(data: Vec<u8>) -> f32 {
    return fixed16_to_double(as_u16(data)) ;//* 10.0;
}

impl TryFrom<Vec<u8>> for MeasureCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetLogicVoltage(decode_voltage(data))
                        }
                    },
                    0x01 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetPeripheralVoltage(decode_voltage(data))
                        }
                    },
                    0x02 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetRightHandVoltage(decode_voltage(data))
                        }
                    },
                    0x03 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetLeftHandVoltage(decode_voltage(data))
                        }
                    },
                    0x04 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetRightFootVoltage(decode_voltage(data))
                        }
                    },
                    0x05 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetLeftFootVoltage(decode_voltage(data))
                        }
                    },
                    0x06 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetSquibVoltage(decode_voltage(data))
                        }
                    },
                    0x07 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid voltage data"));
                        } else {
                            MeasureCmd::GetBatteryVoltage(decode_voltage(data))
                        }
                    },
                    0x08 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid charging state data"));
                        } else {
                            let state = decode_charging_state(data[0]);
                            MeasureCmd::GetChargingState(Some(state))
                        }
                    }
                    0x09 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid charging state data"));
                        } else {
                            let state = if data[0] > 0 { true } else { false };
                            MeasureCmd::GetZIPSwitchState(state)
                        }
                    },
                    0x0a => {
                        println!("Nothing needed to be returned for a .SET. packet type?");
                        if data.len() > 0 {
                            println!("got data back.. {:?}", data);
                        }
                        MeasureCmd::SetEXTDisplay(0)
                    },
                    _    => return Err(Error::new(ErrorKind::Other, "invalid general cmd")),
                };
            Ok(cmd)
        } else {
                return Err(Error::new(ErrorKind::Other, "no general cmd byte"));
        }
    }
}

impl TechAirEncoder for MeasureCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        println!("write_bytes(): MeasureCmd");
        match self {
            MeasureCmd::GetLogicVoltage(_) => {
                buf.push(0x00);
            },
            MeasureCmd::GetPeripheralVoltage(_) => {
                buf.push(0x01);
            },
            MeasureCmd::GetRightHandVoltage(_) => {
                buf.push(0x02);
            },
            MeasureCmd::GetLeftHandVoltage(_) => {
                buf.push(0x03);
            },
            MeasureCmd::GetRightFootVoltage(_) => {
                buf.push(0x04);
            },
            MeasureCmd::GetLeftFootVoltage(_) => {
                buf.push(0x05);
            },
            MeasureCmd::GetSquibVoltage(_) => {
                buf.push(0x06);
            },
            MeasureCmd::GetBatteryVoltage(_) => {
                buf.push(0x07);
            },
            MeasureCmd::GetChargingState(_) => {
                buf.push(0x08);
            },
            MeasureCmd::GetZIPSwitchState(_) => {
                buf.push(0x09);
            },
            MeasureCmd::SetEXTDisplay(flag) => {
                buf.push(0x0a);
                buf.push(*flag);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second
    }
}

//	MEASURE = 0x3,
//	GET_CHARGING_STATE = 0x8,
//	[0x03, 0x08, <CRC16>(.,.)]
pub fn decode_charging_state(v: u8) -> String {
    let s0 = if 1 & v > 0 {
        "Over or undervoltage present, "
    } else if 2 & v > 0 {
            "USB Power OK, "
    } else { "" };
    let s1 = match v >> 2 {
        0 => "Charge suspend",
        1 => "Fast charge",
        2 => "Charge done",
        3 => "Pre charge",
        _ => "Error in charge information"
    };
    return s0.to_owned() + s1;
}
