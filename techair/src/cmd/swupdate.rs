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
pub enum SWUpdateCmd {
    StartBootLoader,
    GetBootLoaderVersion(Option<u8>),
    GetBootLoaderState(Option<SWUpdateBootLoaderStates>),
    WriteFWData(FWData),
    QuitBootLoader,
    CRCCheck(u16),
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum SWUpdateBootLoaderStates {
    WaitFW,
    CheckReadFW,
    EraseFlash,
    FlashFW,
    CalcFlashedCRC,
    WaitVerifyFlashedCRC,
    VerifyFlashedCRCMemory,
}

impl From<u8> for SWUpdateBootLoaderStates {
    fn from(b: u8) -> Self {
        match b {
            0x0 => SWUpdateBootLoaderStates::WaitFW,
            0x1 => SWUpdateBootLoaderStates::CheckReadFW,
            0x2 => SWUpdateBootLoaderStates::EraseFlash,
            0x3 => SWUpdateBootLoaderStates::FlashFW,
            0x4 => SWUpdateBootLoaderStates::CalcFlashedCRC,
            0x5 => SWUpdateBootLoaderStates::WaitVerifyFlashedCRC,
            0x6 => SWUpdateBootLoaderStates::VerifyFlashedCRCMemory,
            _ => panic!("cannot convert"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FWData {
    pub page_count: u16,
    chunk_len: usize, // decryption_len - info_len (num2)
    data: Option<Vec::<u8>>,
    //crc: u16,//[u8;2],
}

impl FWData {
    pub fn new(data: &[u8], page_count: &u16) -> Option<FWData> {
        let chunk_len = data.len();
        if chunk_len > 256 || chunk_len == 0 {
            return None;
        }
        Some(FWData{
            page_count: *page_count,
            chunk_len: chunk_len,
            data: Some(data.to_owned()),
        })
    }
}

impl TryFrom<Vec<u8>> for SWUpdateCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => SWUpdateCmd::StartBootLoader,
                    0x01 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid swupdate data"));
                        } else {
                            SWUpdateCmd::GetBootLoaderVersion(Some(data[0]))
                        }
                    },
                    0x02 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid swupdate data"));
                        } else {
                            let state: SWUpdateBootLoaderStates = data[0].into();
                            SWUpdateCmd::GetBootLoaderState(Some(state))
                        }
                    },
                    0x03 => {
                        if data.len() < 2 {
                            return Err(Error::new(ErrorKind::Other, "invalid swupdate data"));
                        } else {
                            println!("data = {:?}", data);
                            let page_count: u16 = ((data[0] as u16) << 8)  + data[1] as u16;
                            if page_count == 0xFFFF {
                                println!("err xfer hex data failed {:?}", data);
                                return Err(Error::new(ErrorKind::Other, "xfer hex data failed"));
                            }
                            // if returned page_count != num of bytes sent, then fail!.
                            // if returned page_count == 0, then success!
                            let fw_data = FWData{
                                page_count: page_count,
                                chunk_len: 0,
                                data: None,
                            };
                            SWUpdateCmd::WriteFWData(fw_data)
                        }
                    },
                    0x04 => SWUpdateCmd::QuitBootLoader,
                    0x05 => {
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid swupdate data"));
                        } else {
                            // non-zero value indicates a failed xfer
                            SWUpdateCmd::CRCCheck(data[0] as u16)
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

impl TechAirEncoder for SWUpdateCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        println!("write_bytes(): SWUpdateCmd");
        match self {
            SWUpdateCmd::StartBootLoader => {
                buf.push(0x00);
            },
            SWUpdateCmd::GetBootLoaderVersion(_) => {
                buf.push(0x01);
            },
            SWUpdateCmd::GetBootLoaderState(_) => {
                buf.push(0x02);
            },
            SWUpdateCmd::WriteFWData(fw_data) => {
                buf.push(0x03);
                buf.push((fw_data.page_count >> 8) as u8);
                buf.push( fw_data.page_count       as u8);
//                if fw_data.chunk_len < 256 {
                    buf.push((fw_data.chunk_len >> 8) as u8);
                    buf.push( fw_data.chunk_len       as u8);
//                } else {
                // hack to emulate winblows
//                buf.push(0x16 as u8);
//                buf.push(0x00 as u8);
//                    buf.push(0x00 as u8);
//                }
//            println!("DEBUG: chunk len = {}", fw_data.chunk_len);

                // buf[idx=6] = <data segment[0]> || crc16
                // buf[idx=7] = <data segment[1.> || crc16
                if let Some(data) = &fw_data.data {
                    for b in data.iter() {
                        buf.push(*b);
                    }
                }
            },
            SWUpdateCmd::QuitBootLoader => {
                buf.push(0x04);
            },
            SWUpdateCmd::CRCCheck(crc) => {
                buf.push(0x05);
                buf.push((crc >>   8) as u8);
                buf.push((crc & 0xff) as u8);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second
    }
}
