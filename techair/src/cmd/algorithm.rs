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
pub enum AlgorithmCmd {
    InitAlgorithm,
    DoSingleSampleCalc,
    GetSingleSampleCalcState,
    GetSingleSampleCalcResult,
    GetAlogrithmThresholds(Option<Vec::<u8>>),
    SetAlogrithmThresholds,
    SetAlogrithmDefaultThresholds(Option<u8>),
}

impl TryFrom<Vec<u8>> for AlgorithmCmd {
    type Error = io::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let data = v[1..].to_vec();
        if let Some(subcmd) = v.first() {
            let cmd =
                match subcmd {
                    0x00 => AlgorithmCmd::InitAlgorithm,
                    0x01 => AlgorithmCmd::DoSingleSampleCalc,
                    0x02 => AlgorithmCmd::GetSingleSampleCalcState,
                    0x03 => AlgorithmCmd::GetSingleSampleCalcResult,
                    0x04 => {
                        if data.len() < 1 { // ???
                            return Err(Error::new(ErrorKind::Other, "invalid algorithm threshold data"));
                        } else {
                            println!("data = {:?}", data);
                            AlgorithmCmd::GetAlogrithmThresholds(Some(data))
                        }
                    },
                    0x05 => AlgorithmCmd::SetAlogrithmThresholds,
                    0x06 => { // requires uiSoftwareVersion >= 279 else write algorithm default THs in FW not impl
                        if data.len() < 1 {
                            return Err(Error::new(ErrorKind::Other, "invalid algorithm threshold data"));
                        } else {
                            println!("data = {:?}", data);
                            AlgorithmCmd::SetAlogrithmDefaultThresholds(Some(data[0]))
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

impl TechAirEncoder for AlgorithmCmd {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        println!("write_bytes(): AlgorithmCmd");
        match self {
            AlgorithmCmd::InitAlgorithm => {
                buf.push(0x00);
            },
            AlgorithmCmd::DoSingleSampleCalc => {
                buf.push(0x01);
            },
            AlgorithmCmd::GetSingleSampleCalcState => {
                buf.push(0x02);
            },
            AlgorithmCmd::GetSingleSampleCalcResult => {
                buf.push(0x03);
            },
            AlgorithmCmd::GetAlogrithmThresholds(_) => {
                buf.push(0x04);
            },
            AlgorithmCmd::SetAlogrithmThresholds => {
                buf.push(0x05);
            },
            AlgorithmCmd::SetAlogrithmDefaultThresholds(_) => {
                buf.push(0x06);
            },
        }
	let crc = CRC16::calculate(buf.as_slice()).as_u16();
        println!("cal crc {:#04x?}", crc);
        buf.push((crc & 0xff) as u8); // LSB first
        buf.push((crc >>   8) as u8); // MSB second
    }
}
