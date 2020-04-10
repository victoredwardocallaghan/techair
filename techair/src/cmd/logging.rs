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


#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum LoggingCmd {
    GetOPHours(Option<String>),
    ClearOPHours,
    GetNumOfErrors(Option<u8>),
    GetErrorEntry(Option<Vec::<u8>>),
    ClearErrorHistory,
    GetNumOfPreCrashLogs(Option<u16>),
    GetPreCrashLogs(Option<Vec::<u8>>),
    ClearPreCrashLog,
    GetNumOfPostCrashLogs(Option<u16>),
    GetPostCrashBulk(Option<u8>),
    ClearPostCrashLog,
    GetPreCrashBulk,
    GetErrorHistory(Option<Vec::<u8>>),
    GetPostCrashLogs(Option<Vec::<u8>>),
    GetBatCount(Option<u16>),
    GetPreCrashENCBulk,
    GetPostCrashENCBulk,
}

impl TryFrom<Vec<u8>> for LoggingCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => {
                        if data.len() < 4 {
                            return Err(Error::new(ErrorKind::Other, "invalid op hours data"));
                        } else {
                            println!("data = {:?}", data);
                            let hours: u16 = ((data[0] as u16) << 8) | data[1] as u16;
                            let s = format!("{:02}:{:02}:{:02}",
                                hours, data[2], data[3]);
                            LoggingCmd::GetOPHours(Some(s))
                        }
                    },
                    0x01 => LoggingCmd::ClearOPHours,
                    0x02 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid no-errors logs data"));
                        } else {
                            println!("data = {:?}", data);
                            LoggingCmd::GetNumOfErrors(Some(data[0]))
                        }
                    },
                    0x03 => {
                        if data.len() < 1 { // ???
                            return Err(Error::new(ErrorKind::Other, "invalid error-entry logs data"));
                        } else {
                            println!("data = {:?}", data);
                            LoggingCmd::GetErrorEntry(Some(data))
                        }
                    },
                    0x04 => LoggingCmd::ClearErrorHistory,
                    0x05 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid pre-crash logs data"));
                        } else {
                            println!("data = {:?}", data);
                            let decode: u16 = ((data[0] as u16) << 8)  + data[1] as u16; // XXX correct decode?
                            LoggingCmd::GetNumOfPreCrashLogs(Some(decode))
                        }
                    },
                    0x06 => {
                        if data.len() < 1 { // ???
                            return Err(Error::new(ErrorKind::Other, "invalid pre-crash logs data"));
                        } else {
                            println!("data = {:?}", data);
                            LoggingCmd::GetPreCrashLogs(Some(data))
                        }
                    },
                    0x07 => LoggingCmd::ClearPreCrashLog,
                    0x08 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid post-crash logs data"));
                        } else {
                            println!("data = {:?}", data);
                            let decode: u16 = ((data[0] as u16) << 8)  + data[1] as u16; // XXX correct decode?
                            LoggingCmd::GetNumOfPostCrashLogs(Some(decode))
                        }
                    },
                    0x09 => {
                        LoggingCmd::GetPostCrashBulk(None)
                    },
                    0x0a => LoggingCmd::ClearPostCrashLog,
                    0x0b => LoggingCmd::GetPreCrashBulk,
                    0x0c => {
                        if data.len() < 1 { // ???
                            return Err(Error::new(ErrorKind::Other, "invalid error history logs data"));
                        } else {
                            LoggingCmd::GetErrorHistory(Some(data.to_vec()))
                        }
                    },
                    0x0d => {
                        if data.len() < 1 { // ???
                            return Err(Error::new(ErrorKind::Other, "invalid post-crash logs data"));
                        } else {
                            println!("data = {:?}", data);
                            LoggingCmd::GetPostCrashLogs(Some(data))
                        }
                    },
                    0x0e => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid bat-count logs data"));
                        } else {
                            println!("data = {:?}", data);
                            let decode: u16 = ((data[0] as u16) << 8)  + data[1] as u16; // XXX correct decode?
                            LoggingCmd::GetBatCount(Some(decode))
                        }
                    },
                    0x0f => LoggingCmd::GetPreCrashENCBulk,
                    0x10 => LoggingCmd::GetPostCrashENCBulk,
                    _    => return Err(Error::new(ErrorKind::Other, "invalid general cmd")),
                };
            Ok(cmd)
        } else {
                return Err(Error::new(ErrorKind::Other, "no general cmd byte"));
        }
    }
}

impl TechAirEncoder for LoggingCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        println!("write_bytes(): LoggingCmd");
        match self {
            LoggingCmd::GetOPHours(_) => {
                buf.push(0x00);
            },
            LoggingCmd::ClearOPHours => {
                buf.push(0x01);
            },
            LoggingCmd::GetNumOfErrors(_) => {
                buf.push(0x02);
            },
            LoggingCmd::GetErrorEntry(_) => {
                buf.push(0x03);
            },
            LoggingCmd::ClearErrorHistory => {
                buf.push(0x04);
            },
            LoggingCmd::GetNumOfPreCrashLogs(_) => {
                buf.push(0x05);
            },
            LoggingCmd::GetPreCrashLogs(_) => {
                buf.push(0x06);
            },
            LoggingCmd::ClearPreCrashLog => {
                buf.push(0x07);
            },
            LoggingCmd::GetNumOfPostCrashLogs(_) => {
                buf.push(0x08);
            },
            LoggingCmd::GetPostCrashBulk(_) => {
                buf.push(0x09);
            },
            LoggingCmd::ClearPostCrashLog => {
                buf.push(0x0a);
            },
            LoggingCmd::GetPreCrashBulk => {
                buf.push(0x0b);
            },
            LoggingCmd::GetErrorHistory(_) => {
                buf.push(0x0c);
            },
            LoggingCmd::GetPostCrashLogs(_) => {
                buf.push(0x0d);
            },
            LoggingCmd::GetBatCount(_) => {
                buf.push(0x0e);
            },
            LoggingCmd::GetPreCrashENCBulk => {
                buf.push(0x0f);
            },
            LoggingCmd::GetPostCrashENCBulk => {
                buf.push(0x10);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second
    }
}
