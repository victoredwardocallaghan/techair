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
pub enum AirbagCmd {
    GetIgnitionCtrlMode,
    GetIgnitionCtrlStatus,
    InitIgnitionCtrl,
    DiagIgnitionCtrl,
    ArmIgnitionCtrl,
    FireAIRBAG, // Umm !?
    ResetIgnitionCtrl,
    DiagGetSquibRes,
    GetCalibSquibRes((Option<f32>, Option<f32>)),
    SetCalibSquibRes,
    GetInflationType(u8),
    SetInflationType(u8),
}

fn as_u16(data: &[u8]) -> u16 {
    return ((data[0] as u16) << 8) | data[1] as u16;
}

fn decode_calibres(data: &[u8]) -> f32 {
    return fixed16_to_double(as_u16(data));
}

impl TryFrom<Vec<u8>> for AirbagCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => AirbagCmd::GetIgnitionCtrlMode,
                    0x01 => AirbagCmd::GetIgnitionCtrlStatus,
                    0x02 => AirbagCmd::InitIgnitionCtrl,
                    0x03 => AirbagCmd::DiagIgnitionCtrl,
                    0x04 => AirbagCmd::ArmIgnitionCtrl,
                    0x05 => {
                        panic!("WTF are you doing?!");
                        //AirbagCmd::FireAIRBAG, // Umm !?
                    },
                    0x06 => AirbagCmd::ResetIgnitionCtrl,
                    0x07 => AirbagCmd::DiagGetSquibRes,
                    0x08 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid res calibration data"));
                        } else {
                            let r0 = decode_calibres(&data);
                            let res1 = if r0 <= 100.0 && r0 >= -100.0 { Some(r0) } else { println!("calibration values returned not valid"); None };
                            let r1 = decode_calibres(&data[2..]);
                            let res2 = if r1 <= 100.0 && r1 >= -100.0 { Some(r1) } else { println!("calibration values returned not valid"); None };
                            AirbagCmd::GetCalibSquibRes((res1, res2))
                        }
                    },
                    0x09 => AirbagCmd::SetCalibSquibRes,
                    0x0a => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid inflation type data"));
                        } else {
                            AirbagCmd::GetInflationType(data[0])
                        }
                    },
                    0x0b => AirbagCmd::SetInflationType(0),
                    _    => return Err(Error::new(ErrorKind::Other, "invalid general cmd")),
                };
            Ok(cmd)
        } else {
                return Err(Error::new(ErrorKind::Other, "no general cmd byte"));
        }
    }
}

impl TechAirEncoder for AirbagCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        println!("write_bytes(): AirbagCmd");
        match self {
            AirbagCmd::GetIgnitionCtrlMode => {
                buf.push(0x00);
            },
            AirbagCmd::GetIgnitionCtrlStatus => {
                buf.push(0x01);
            },
            AirbagCmd::InitIgnitionCtrl => {
                buf.push(0x02);
            },
            AirbagCmd::DiagIgnitionCtrl => {
                buf.push(0x03);
            },
            AirbagCmd::ArmIgnitionCtrl => {
                buf.push(0x04);
            },
            AirbagCmd::FireAIRBAG => { // Umm !?
                panic!("WTF are you doing!?");
                //buf.push(0x05);
            },
            AirbagCmd::ResetIgnitionCtrl => {
                buf.push(0x06);
            },
            AirbagCmd::DiagGetSquibRes => {
                buf.push(0x07);
            },
            AirbagCmd::GetCalibSquibRes(_) => {
                buf.push(0x08);
            },
            AirbagCmd::SetCalibSquibRes => {
                buf.push(0x09);
            },
            AirbagCmd::GetInflationType(_) => {
                buf.push(0x0a);
            },
            AirbagCmd::SetInflationType(it) => {
                buf.push(0x0b);
                buf.push(*it);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second
    }
}
