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

extern crate serialport;
extern crate byteorder;

//use serialport::prelude::*;
use serialport::{SerialPortType,SerialPortInfo};

use std::convert::TryFrom;
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::time::Duration;
use std::io::prelude::*;

use crate::usbcmd::{UsbCmd};
use crate::encoder::TechAirEncoder;


pub struct TechAir {
    port: String,
    // keep a boxed heap allocation of the trait SerialPort
    uart: Box<dyn serialport::SerialPort>,
}

impl fmt::Debug for TechAir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.port)
    }
}

#[derive(Debug)]
pub struct TechAirError {
    details: String
}

impl TechAirError {
    fn new(msg: &str) -> TechAirError {
        TechAirError{details: msg.to_string()}
    }
}

impl fmt::Display for TechAirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for TechAirError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<serialport::Error> for TechAirError {
    fn from(err: serialport::Error) -> TechAirError {
        TechAirError::new(err.description())
    }
}

impl TechAir {
    pub fn new() -> Result<TechAir, TechAirError> {
        let p = find_techair()?;
    	let s = serialport::SerialPortSettings{
    		baud_rate: 115200,
    		data_bits: serialport::DataBits::Eight,
    		flow_control: serialport::FlowControl::None,
    		parity: serialport::Parity::None,
    		stop_bits: serialport::StopBits::One,
    		timeout: Duration::from_millis(100) // 2000 R, 500 W ?
    	};
        if let Ok(uart) = serialport::open_with_settings(&p, &s) { 
            uart.clear(serialport::ClearBuffer::All)?;
            Ok(TechAir{
                port: p,
                uart: uart,
            })
        } else {
            Err(TechAirError::new("could not open serial uart"))
        }
    }

    pub fn set_timeout(&mut self, time: Duration)
        -> Result<(), serialport::Error> {
//        -> Result<(), serialport::Error> {
        let uart = self.uart.as_mut();
        uart.set_timeout(time)
    }

    pub fn bytes_to_write_left(&mut self) {
        let uart = self.uart.as_mut();
        println!("DEBUG: bytes_to_write_left() = {:?}", uart.bytes_to_write());
    }

    pub fn read(&mut self)
        -> Result<UsbCmd, std::io::Error> {
            let uart = self.uart.as_mut();
            let mut buf: Vec<u8> = Vec::new();
            for byte in uart.bytes() {
                if let Ok(b) = byte {
                    buf.push(b);
                } else {
                    break;
                }
            }
            if buf.len() > 0 {
                println!("processing vec = {:#?}", buf);
                let cmd = UsbCmd::try_from(buf)?;
                return Ok(cmd);
            }
            Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "no bytes returned"))
    }

    pub fn write(&mut self, cmd: UsbCmd)
        -> std::io::Result<()> {
            let uart = self.uart.as_mut();
            let mut buf: Vec<u8> = Vec::new();
            cmd.write_bytes(&mut buf);
            //println!("writing buf = {:#04x?}", buf.as_slice());
// XXX comment out for swupdate work to be NOPed
            uart.clear(serialport::ClearBuffer::Input)?;
            uart.write_all(&buf.as_slice())//?;
            //uart.flush()
//            cmd.write_bytes(&mut uart)
    }
}

fn find_techair() -> Result<String, TechAirError> {
	let vap = serialport::available_ports()?;
        vap.into_iter().filter(filter_sp)
            .map(|p| p.port_name).next()
            .ok_or(TechAirError::new("no techair found!"))
}

fn filter_sp(sp: &SerialPortInfo) -> bool {
    if let SerialPortType::UsbPort(ref u) = sp.port_type {
        if u.manufacturer == Some("ALPINESTARS".to_string()) &&
           u.product == Some("Airbag_Control_Unit".to_string()) {
            println!("Found! 'Alpinestars ACU'.");
            return true;
        }
    }
    false
}
