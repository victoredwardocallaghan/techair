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
pub enum SensorCmd {
    EnableSensorReading(Option<u8>),
    GetSensorReadingEnables(u8),
    GetRightHandAccel(Option<(f32,f32,f32)>),
    GetLeftHandAccel(Option<(f32,f32,f32)>),
    GetRightFootAccel(Option<(f32,f32,f32)>),
    GetLeftFootAccel(Option<(f32,f32,f32)>),
    GetBodyAccel(Option<(f32,f32,f32)>),
    GetGyroscope(Option<(f32,f32,f32)>),
    GetSWVRH((f32, f32)),
    GetSWVLH((f32, f32)),
    GetSWVRF((f32, f32)),
    GetSWVLF((f32, f32)),
}

fn decode_rev(data: Vec<u8>) -> (f32, f32) {
    let s = ((data[0] as u16) << 8) | data[1] as u16;
    let h = ((data[2] as u16) << 8) | data[3] as u16;
    let sw = fixed16_to_double(s) * 10.0;
    let hw = fixed16_to_double(h) * 10.0;
    return  (sw, hw);
}

fn decode_xyz(data: Vec<u8>) -> (u16, u16, u16) {
    let x = ((data[0] as u16) << 8) | data[1] as u16;
    let y = ((data[2] as u16) << 8) | data[3] as u16;
    let z = ((data[4] as u16) << 8) | data[5] as u16;
    (x, y, z)
}

fn decode_accel(data: Vec<u8>) -> (f32, f32, f32) {
    let (x, y, z) = decode_xyz(data);
    (calculate_accel(x), calculate_accel(y), calculate_accel(z))
}

fn decode_gyro(data: Vec<u8>) -> (f32, f32, f32) {
    let (x, y, z) = decode_xyz(data);
    (calculate_gyro(x), calculate_gyro(y), calculate_gyro(z))
}

impl TryFrom<Vec<u8>> for SensorCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => {
                        SensorCmd::EnableSensorReading(None)
                    },
                    0x01 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor en data"));
                        } else {
                            SensorCmd::GetSensorReadingEnables(data[0])
                        }
                    },
                    0x02 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor data"));
                        } else {
                            let s = decode_accel(data);
                            SensorCmd::GetRightHandAccel(Some(s))
                        }
                    },
                    0x03 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor data"));
                        } else {
                            let s = decode_accel(data);
                            SensorCmd::GetLeftHandAccel(Some(s))
                        }
                    },
                    0x04 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor data"));
                        } else {
                            let s = decode_accel(data);
                            SensorCmd::GetRightFootAccel(Some(s))
                        }
                    },
                    0x05 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor data"));
                        } else {
                            let s = decode_accel(data);
                            SensorCmd::GetLeftFootAccel(Some(s))
                        }
                    },
                    0x06 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor data"));
                        } else {
                            let s = decode_accel(data);
                            SensorCmd::GetBodyAccel(Some(s))
                        }
                    },
                    0x07 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid sensor data"));
                        } else {
                            let s = decode_gyro(data);
                            SensorCmd::GetGyroscope(Some(s))
                        }
                    },
                    0x08 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid rev data"));
                        } else {
                            SensorCmd::GetSWVRH(decode_rev(data))
                        }
                    },
                    0x09 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid rev data"));
                        } else {
                            SensorCmd::GetSWVLH(decode_rev(data))
                        }
                    },
                    0x0a => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid rev data"));
                        } else {
                            SensorCmd::GetSWVRF(decode_rev(data))
                        }
                    },
                    0x0b => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid rev data"));
                        } else {
                            SensorCmd::GetSWVLF(decode_rev(data))
                        }
                    },
                    _    => return Err(Error::new(ErrorKind::Other, "invalid general cmd")),
                };
            Ok(cmd)
        } else {
                return Err(Error::new(ErrorKind::Other, "no general cmd byte"));
        }
    }
}

impl TechAirEncoder for SensorCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        println!("write_bytes(): SensorCmd");
        match self {
            SensorCmd::EnableSensorReading(mask) => {
                buf.push(0x00);
                if let Some(m) = mask {
                    buf.push(*m);
                }
            },
            SensorCmd::GetSensorReadingEnables(_) => {
                buf.push(0x01);
            },
            SensorCmd::GetRightHandAccel(_) => {
                buf.push(0x02);
            },
            SensorCmd::GetLeftHandAccel(_) => {
                buf.push(0x03);
            },
            SensorCmd::GetRightFootAccel(_) => {
                buf.push(0x04);
            },
            SensorCmd::GetLeftFootAccel(_) => {
                buf.push(0x05);
            },
            SensorCmd::GetBodyAccel(_) => {
                buf.push(0x06);
            },
            SensorCmd::GetGyroscope(_) => {
                buf.push(0x07);
            },
            SensorCmd::GetSWVRH(_) => {
                buf.push(0x08);
            },
            SensorCmd::GetSWVLH(_) => {
                buf.push(0x09);
            },
            SensorCmd::GetSWVRF(_) => {
                buf.push(0x0a);
            },
            SensorCmd::GetSWVLF(_) => {
                buf.push(0x0b);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second
    }
}
