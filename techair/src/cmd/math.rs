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

//use byteorder::{ByteOrder, BigEndian};
use crc16::*;

#[derive(Clone, Copy, Debug)]
pub struct CRC16(u16);

impl CRC16 {
	pub fn new(v: u16) -> CRC16 {
            CRC16(v)
        }
	pub fn calculate(s: &[u8]) -> CRC16 {
		CRC16(State::<MODBUS>::calculate(s))//.to_be_bytes())
	}

	#[inline]
	pub fn as_u16(&self) -> u16 {
		(*self).into()
	}
}

//impl From<CRC16> for [u8;2] {
//	#[inline]
//	fn from(crc: CRC16) -> [u8;2] {
//                (crc.0).to_be_bytes()
//	}
//}

impl From<CRC16> for u16 {
	#[inline]
	fn from(crc: CRC16) -> u16 {
		//BigEndian::read_u16(&crc.0)
                crc.0
	}
}

// 16bit fixed-point format: 13.3 (13 integral bits, 3 fractional bits).
pub fn fixed16_to_double(v: u16) -> f32 {
    const FIXED_POINT_FRACTIONAL_BITS: u32 = 3;
    let b: i32 = 10;
    return (v as f32) / b.pow(FIXED_POINT_FRACTIONAL_BITS) as f32;
}

pub fn calculate_accel(v: u16) -> f32 {
    return fixed16s_to_double(v) * (9984.0 / 625.0);
}

pub fn calculate_gyro(v: u16) -> f32 {
    return fixed16s_to_double(v) * 2279.513043;
}

// 16bit fixed-point (special - used base-2 here and sign bit).
fn fixed16s_to_double(v: u16) -> f32 {
    let s = if 0x8000 & v == 1 { -1.0 } else { 1.0 };
    let d = if s < 0.0 {
        s * ((v & 0x7FFF) ^ 0x7FFF) as f32
    } else { v as f32 };
    return d / (1 << 15) as f32;
}

pub fn calculate_num_pages(hex_file_sz: usize) -> u16 {
    let page_count = if hex_file_sz % 256 == 0 {
        (hex_file_sz / 256) - 1
    } else {
        hex_file_sz / 256
    };
    return page_count as u16;
}

#[cfg(test)]
mod tests {
    #[test]
    fn crc16() {
        let v: u16 = 0xAABB;
        let crc16 = super::CRC16::new(v);
        assert_eq!(crc16.as_u16(), 0xAABB);

        let data = vec![0xff; 5];
        let crc16c = super::CRC16::calculate(&data);
        assert_eq!(crc16c.as_u16(), 32817);
    }
    #[test]
    fn fixed16_to_double() {
        let v = 0xAABB;
        let d = super::fixed16_to_double(v);
        assert_eq!(d, 43.707);
    }
    #[test]
    fn calculate_accel() {
        let v = 0xAABB;
        let a = super::calculate_accel(v);
        assert_eq!(a, 21.307161);
    }
    #[test]
    fn calculate_gyro() {
        let v = 0xAABB;
        let g = super::calculate_gyro(v);
        assert_eq!(g, 3040.4868);
    }
    #[test]
    fn fixed16s_to_double() {
        let v = 0xAABB;
        let d = super::fixed16s_to_double(v);
        assert_eq!(d, 1.3338318);
    }
    #[test]
    fn calculate_num_pages() {
        let hfsz = 0x1080;
        let np = super::calculate_num_pages(hfsz);
        assert_eq!(np, 0x10);
    }
}
