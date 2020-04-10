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
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

use crate::encoder::TechAirEncoder;

use crate::cmd::prelude::*;


//
// NOTES on emum layout are defined here:
//   https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html
//

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum UsbCmd {
    General(GeneralCmd),
    Logging(LoggingCmd),
    Power(PowerCmd),
    Measure(MeasureCmd),
    Sensor(SensorCmd),
    Algorithm(AlgorithmCmd),
    Airbag(AirbagCmd),
//    SDCard,
//    Configure, // 0x10
    SWUpdate(SWUpdateCmd), // 0x20
}

impl UsbCmd {
    pub fn general(self) -> Option<GeneralCmd> {
        match self {
            UsbCmd::General(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn logging(self) -> Option<LoggingCmd> {
        match self {
            UsbCmd::Logging(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn power(self) -> Option<PowerCmd> {
        match self {
            UsbCmd::Power(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn measure(self) -> Option<MeasureCmd> {
        match self {
            UsbCmd::Measure(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn sensor(self) -> Option<SensorCmd> {
        match self {
            UsbCmd::Sensor(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn algorithm(self) -> Option<AlgorithmCmd> {
        match self {
            UsbCmd::Algorithm(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn airbag(self) -> Option<AirbagCmd> {
        match self {
            UsbCmd::Airbag(cmd) => Some(cmd),
            _ => None,
        }
    }
    // ..
    pub fn swupdate(self) -> Option<SWUpdateCmd> {
        match self {
            UsbCmd::SWUpdate(cmd) => Some(cmd),
            _ => None,
        }
    }
}

impl TechAirEncoder for UsbCmd {
//impl<W: Write> TechAirEncoder<W> for UsbCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
//    fn write_bytes(&self, uart: &mut W) -> std::io::Result<()> {
        println!("write_bytes(): UsbCmd");
//        let mut buf = Vec::<u8>::new();
        match self {
            UsbCmd::General(cmd) => {
                buf.push(0x00);
                cmd.write_bytes(buf);
            },
            UsbCmd::Logging(cmd) => {
                buf.push(0x01);
                cmd.write_bytes(buf);
            },
            UsbCmd::Power(cmd) => {
                buf.push(0x02);
                cmd.write_bytes(buf);
            },
            UsbCmd::Measure(cmd) => {
                buf.push(0x03);
                cmd.write_bytes(buf);
            },
            UsbCmd::Sensor(cmd) => {
                buf.push(0x04);
                cmd.write_bytes(buf);
            },
            UsbCmd::Algorithm(cmd) => {
                buf.push(0x05);
                cmd.write_bytes(buf);
            },
            UsbCmd::Airbag(cmd) => {
                buf.push(0x06);
                cmd.write_bytes(buf);
            },
            //..
            UsbCmd::SWUpdate(cmd) => {
                buf.push(0x20);
                cmd.write_bytes(buf);
            },
        }
        println!("writing (usbcmd) buf = {:#04x?}", buf.as_slice());
//        uart.write_all(&buf.as_slice())?;
//        uart.flush()
    }
}

impl TryFrom<Vec<u8>> for UsbCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        validate_crc16(&v)?;
        let subdata = v[1..(v.len() - 2)].to_vec();
        let cmd =
            match v.first() {
                Some(0x00) => {
                    let gcmd = GeneralCmd::try_from(subdata)?;
                    UsbCmd::General(gcmd)
                },
                Some(0x01) => {
                    let gcmd = LoggingCmd::try_from(subdata)?;
                    UsbCmd::Logging(gcmd)
                },
                Some(0x02) => {
                    let gcmd = PowerCmd::try_from(subdata)?;
                    UsbCmd::Power(gcmd)
                },
                Some(0x03) => {
                    let gcmd = MeasureCmd::try_from(subdata)?;
                    UsbCmd::Measure(gcmd)
                },
                Some(0x04) => {
                    let gcmd = SensorCmd::try_from(subdata)?;
                    UsbCmd::Sensor(gcmd)
                },
                Some(0x05) => {
                    let gcmd = AlgorithmCmd::try_from(subdata)?;
                    UsbCmd::Algorithm(gcmd)
                },
                Some(0x06) => {
                    let gcmd = AirbagCmd::try_from(subdata)?;
                    UsbCmd::Airbag(gcmd)
                },
                // ..
                //Some(0x09) => {
                Some(0x20) => {
                    let gcmd = SWUpdateCmd::try_from(subdata)?;
                    UsbCmd::SWUpdate(gcmd)
                },
                _ => return Err(Error::new(ErrorKind::Other, "invalid usb cmd")),
            };
        Ok(cmd)
    }
}

fn validate_crc16(v: &[u8]) -> Result<(), io::Error> {
        let data = &v[0..(v.len() - 2)]; // strip crc16 off end.
//        println!("data={:#?}", data);
	let crc_cal = CRC16::calculate(&data);
        if let Some(crc_data) = v.get((v.len() - 2)..v.len()) {
            let crc = ((crc_data[1] as u16) << 8) | crc_data[0] as u16;
            if crc != crc_cal.as_u16() {
                return Err(
                    Error::new(ErrorKind::Other,
                        format!("CRC16 mismatch, expected {:?}", crc_cal)
                    )
                );
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "cannot extract CRC16"));
        }

        Ok(())
}
