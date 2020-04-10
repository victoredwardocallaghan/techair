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

// TODO: make parameteric over a Writer
//use std::io::prelude::*;
//use std::io::Write;
pub trait TechAirEncoder {
//pub trait TechAirEncoder<W: Write> {
    fn write_bytes(&self, buf: &mut Vec<u8>);
//    fn write_bytes(&self, uart: &mut W) -> std::io::Result<()>;
}

impl TechAirEncoder for Vec<u8> {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_slice());
    }
}
