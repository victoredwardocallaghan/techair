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

use std::str;
use std::convert::TryFrom;
use std::io;
use std::io::{Error, ErrorKind};

use crate::cmd::math::*;
use crate::encoder::TechAirEncoder;


#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum GeneralCmd {
    GetCtrlMode(Option<u8>),
    SetCtrlMode,
    GetSoftwareVersion(Option<f32>),
    GetOperatingModus(Option<OpModus>),
    GetSerialNr(Option<String>),
    SetSerialNr,
    GetHardwareVersion(Option<f32>),
    SetHardwareVersion,
    GetCustomerInfo(Option<String>),
    SetCustomerInfo,
    GetServiceDate(Option<String>),
    SetServiceDate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpModus {
    op_mode: Option<OpMode>,
    op_addons: Option<Vec::<OpAddons>>,
}

impl TryFrom<Vec<u8>> for GeneralCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid ctrl mode data"));
                        } else {
                            GeneralCmd::GetCtrlMode(Some(data[0]))
                        }
                    },
                    0x01 => GeneralCmd::SetCtrlMode,
                    0x02 => {
                        if data.len() < 2 {
                            // TODO: Return error instead and change variant inner type from
                            // Option<T> to just T.
                            return Err(Error::new(ErrorKind::Other, "invalid sw version data"));
                            //GeneralCmd::GetSoftwareVersion(None)
                        } else {
                            let ver = ((data[0] as u16) << 8) | data[1] as u16;
                            GeneralCmd::GetSoftwareVersion(Some(fixed16_to_double(ver) * 10.0))
                        }
                    },
                    0x03 => {
                        if data.len() < 2 {
                            GeneralCmd::GetOperatingModus(None)
                        } else {
                            let mode = decode_op_mode(data[0]);
                            let addons = decode_op_addons(data[1]);
                            let modus = OpModus{op_mode: mode, op_addons: addons};
                            GeneralCmd::GetOperatingModus(Some(modus))
                        }
                    },
                    0x04 => {
                        if let Ok(str) = str::from_utf8(&data) {
                            GeneralCmd::GetSerialNr(Some(str.to_string()))
                        } else {
                            GeneralCmd::GetSerialNr(None)
                        }
                    },
                    0x05 => GeneralCmd::SetSerialNr,
                    0x06 => {
                        if data.len() < 2 {
                            GeneralCmd::GetHardwareVersion(None)
                        } else {
                            let ver = ((data[0] as u16) << 8) | data[1] as u16;
                            GeneralCmd::GetHardwareVersion(Some(fixed16_to_double(ver) * 10.0))
                        }
                    },
                    0x07 => GeneralCmd::SetHardwareVersion,
                    0x08 => {
                        // first byte is the len of the data buf which we dont need
                        if let Ok(str) = str::from_utf8(&data[1..]) {
                            GeneralCmd::GetCustomerInfo(Some(str.to_string()))
                        } else {
                            GeneralCmd::GetCustomerInfo(None)
                        }
                    },
                    0x09 => GeneralCmd::SetCustomerInfo,
                    0x0a => {
                        if data.len() < 3 {
                            GeneralCmd::GetServiceDate(None)
                        } else {
                            let s = format!("{:02}/{:02}/20{:02}",
                                data[0], data[1], data[2]);
                            GeneralCmd::GetServiceDate(Some(s))
                        }
                    },
                    0x0b => GeneralCmd::SetServiceDate,
                    _    => return Err(Error::new(ErrorKind::Other, "invalid general cmd")),
                };
            Ok(cmd)
        } else {
                return Err(Error::new(ErrorKind::Other, "no general cmd byte"));
        }
    }
}

// TODO: better way to do this?
impl TechAirEncoder for GeneralCmd {
//impl<W: Write> TechAirEncoder<W> for GeneralCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
//    fn write_bytes(&self, uart: &mut W) -> std::io::Result<()> {
//        let mut buf = Vec::<u8>::new();
        println!("write_bytes(): GeneralCmd");
        match self {
            GeneralCmd::GetCtrlMode(_) => {
                buf.push(0x00);
            },
            GeneralCmd::SetCtrlMode => {
                buf.push(0x01);
            },
//            GeneralCmd::GetSoftwareVersion(d) => {
            GeneralCmd::GetSoftwareVersion(_) => {
                buf.push(0x02);
//                if let Some(v) = d {
//                    v.write_bytes(buf);
//                }
            },
            GeneralCmd::GetOperatingModus(_) => {
                buf.push(0x03);
            },
            GeneralCmd::GetSerialNr(_) => {
                buf.push(0x04);
            },
            GeneralCmd::SetSerialNr => {
                buf.push(0x05);
            },
            GeneralCmd::GetHardwareVersion(_) => {
                buf.push(0x06);
            },
            GeneralCmd::SetHardwareVersion => {
                buf.push(0x07);
            },
            GeneralCmd::GetCustomerInfo(_) => {
                buf.push(0x08);
            },
            GeneralCmd::SetCustomerInfo => {
                buf.push(0x09);
            },
            GeneralCmd::GetServiceDate(_) => {
                buf.push(0x0a);
            },
            GeneralCmd::SetServiceDate => {
                buf.push(0x0b);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second

//        uart.write_all(&buf.as_slice())?;
//        uart.flush()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum OpMode {
    Street,
    Race3S,
    Race,
    RaceVestStreet,
}

fn decode_op_mode(flags: u8) -> Option<OpMode> {
    match flags {
        0xaa => Some(OpMode::Street),
        0xbb => Some(OpMode::Race3S),
        0xdd => Some(OpMode::Race),
        0xcc => Some(OpMode::RaceVestStreet),
        _    => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpAddons {
    OpenLoop,
    SDCard,
    GPS,
}

fn decode_op_addons(flags: u8) -> Option<Vec::<OpAddons>> {
    let mut decode = Vec::<OpAddons>::new();
    if flags &4 == 1 { // &4 - SD-Card present
        decode.push(OpAddons::SDCard);
    }
    if flags &8 == 1 { // &8 - GPS-Module present
        decode.push(OpAddons::GPS);
    }
    if flags &16 == 1 { // &16 - Open-Loop-Detection disabled
        decode.push(OpAddons::OpenLoop);
    }
    if decode.len() > 0 { Some(decode) } else { None }
}
//pub fn decode_extensions(v: u8) -> String {
//    let mut decode = String::new();
//    let a = ["OL", "SD", "GPS"];
//    for (i, s) in a.enumate() {
//        if v & (1 << i) < 0 {
//            decode.append(s);
//        }
//        decode.append(", ");
//    }
//    return "Extensions: " + decode;
//}
