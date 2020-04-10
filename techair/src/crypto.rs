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

extern crate crypto;

use std::str;
//use std::error::Error;
//use std::io::prelude::*;
use std::io::{Read, Write};
use std::fs::File;

use crypto::{aes, buffer, blockmodes, symmetriccipher};
use crypto::buffer::{ReadBuffer, WriteBuffer};

// needed for CRC16 in math
use crate::cmd::prelude::*;


fn pic32_rom_addresses(bootloader_ver: u8) -> (u32, u32) {
    // num4 is end address of rom.
    // 487063551 = 0x1d07ffff 
    //   -> PIC32MX340F512H,
    //      PIC32MX360F512L,
    //      PIC32MX440F512H,
    //      PIC32MX460F512L.
    // 486801407 = 0x1d03ffff
    //   ->  PIC32MX340F256H,
    //       PIC32MX360F256L,
    //       PIC32MX440F256H,
    //       PIC32MX460F256L.
    // 486563840 = 0x1d006000 <- oh! rom : ORIGIN = . for pic32 lol !!!
    let num4 = if bootloader_ver > 3 {
        487063551
    } else { 486801407 };
    (0x1d006000, num4)
}

pub fn crc16_fw_data(fwf: FwFile, bl_ver: u8) -> Result<u16, std::io::Error> {
    let addr_space = pic32_rom_addresses(bl_ver);
    let ihex_data = parse_ihex(fwf.data.as_slice(), addr_space)?;
//    println!(" > DEBUG {:#02x?}", ihex_data);
    let ihex_crc16 = CRC16::calculate(&ihex_data).as_u16();
    Ok(ihex_crc16)
}

// decodes Intel hex format
fn parse_ihex(ihex: &[u8], addr_space: (u32, u32)) -> Result<Vec::<u8>, std::io::Error> {
    let (s, e) = addr_space;
    let mut hexdata_buf = vec![0xff; (e - s + 1) as usize];
    let mut bad_dseg = false;
        let mut num1 = 0; // low byte
        let mut num2 = 0; // high byte, make up base add.
    let mut dseg = vec![0x00; 60];
    // strip off '\r\n'.
    let data: Vec::<&u8> = ihex.into_iter().filter(|&&b| (b != b'\r') && (b != b'\n')).collect();
    // A ascii ':' is the 'start code'.
    for seg in data.split(|&&b| b == b':') {
        if seg.len() == 0 {
            continue; // null segment on the zero'th line, skip it.
        }
        if seg.len() <= 10 {
            continue; // winblows doesn't bother processing these lines.
        }
//        println!("seg={:?}", seg);
        let record = decode_hexline(seg).unwrap(); // assume line is ok :/
//        println!("record={:?}", record);
        let bcount = record[0]; // num5
//        println!("bcount={}", bcount);
        let mut crc_bsum: u32 = 0;
        for i in 0..bcount+5 {
            crc_bsum += record[i as usize] as u32;
        }
        if crc_bsum & 0xff != 0 {
            println!("bcount={}, crc=0x{:02x}", bcount, (crc_bsum &0xff) as u8);
            panic!("invalid crc in hex file");
        }
        //if record[(bcount+4) as usize] & 0xff == 0xff {
        //    println!("EOF record found! breaking..");
        //    break;
        //}
        for i in 0..bcount {
            let idx = 4+i as usize;
//            println!("idx={}", idx);
//            println!("record=0x{:x}", record[idx]);
            dseg[i as usize] = record[idx]; // dseg := numArray
        }
        let rtype = record[3]; // num6
        if rtype == 0 { // DATA.
            // num8 = base_addr
//            println!("record[1,2]=0x{:x}, 0x{:x}", record[1], record[2]);
            let base_addr: u32 = ((( ((record[1] as i32) << 8) & 0xff00 as i32 | (record[2] as i32) & 0xff as i32) & 0xffff as i32) as u32) + num1 + num2;
            // DEVCFG3-0: origin: 0x1fc02ff0, kseg1, len=16bytes
            if base_addr > s && base_addr < e {
                // num9 = rel_addr
                let rel_addr: u32 = base_addr - s;
                if rel_addr <= e - s - (bcount as u32) {
                    for i in 0..bcount {
                        println!("base_addr=0x{:x}: dseg[..]=0x{:x}", base_addr, dseg[i as usize]);
                        hexdata_buf[(i as u32 + rel_addr) as usize] = dseg[i as usize];
                    }
                    continue; // we are done with this dseg.
                }
                // num3 is bad_dseg_flag
                bad_dseg = true; // bad dseg, invalid rel_addr, don't bother cal crc16
            }
        } else if  rtype == 2 {
            //2 => { // Extended Segment Address.
            // 65280     =  0x0000ff00
            // 16711680  =  0x00ff0000
            // -16777216 = ~0x00ffffff
            let x = (dseg[0] as i32) << 16 & 0x00ff0000 | (dseg[1] as i32) << 8 & 0x0000ff00;
            num1 = x as u32;
            num2 = 0;
        } else if rtype == 4 {
            //4 => { // Extended Linear Address.
            let y = (dseg[0] as i32) << 24 & !0x00ffffff | (dseg[1] as i32) << 16 & 0x00ff0000;
            num1 = 0; num2 = y as u32;
        } else {
            num1 = 0; num2 = 0;
        }
    }
    if bad_dseg == true {
        panic!("bad deseg");
    }
    Ok(hexdata_buf)
}

fn decode_hexline(data: &[&u8]) -> Option<Vec::<u8>> {
    if data.len() % 2 != 0 {
        return None;
    }
    let x = data.into_iter().map(|&&b| decode_asciihex(b)).collect::<Vec::<u8>>();
    Some(x.chunks(2).map(|a| (a[0] << 4) + a[1]).collect())
}

fn decode_asciihex(b: u8) -> u8 {
    char::from(b).to_digit(16).unwrap() as u8
}

pub fn idk() -> Result<FwFile, std::io::Error> {
    // LOLz, wtf is the point? This was always gonna happen..
    // look at the long history.. and just OSS the software from the begining! Christ!
    let key: [u8; 16] = [1,2,3,4,5,6,7,8,9,16,17,18,19,20,21,22];
    let iv: [u8; 16] = [1,2,3,4,5,6,7,8,9,16,17,18,19,20,21,22];

    let mut ifile = File::open("fw_en.bin")?; //{
//        Err(e) => panic!("could not open file because: {}", e.description()),
//        Ok(f) => f,
//    };
 //   let mut reader = BufReader::new(ifile);
 //   assert!(reader.buffer().is_empty());
//    let idata = reader.fill_buf().unwrap();
//    let data = match decrypt(idata, &key, &iv) {
    let mut data = Vec::<u8>::new();
    match decrypt(&mut ifile, &mut data, &key, &iv) {
        Err(_) => panic!("could not decrypt"),
        Ok(_) => println!("xx"),
    }

    let mut ofile = File::create("fw_de.bin")?;// {
//        Err(e) => panic!("could not create file because: {}", e.description()),
//        Ok(f) => f,
//    };
    ofile.write_all(&data)?;// {
//        Err(e) => panic!("could not write because: {}", e.description()),
//        Ok(_)  => println!("wrote out file"),
//    }

    let fw_file = decode_fw_file(data.as_slice());

    if let Some(fw) = fw_file {
        println!("decoded header as: {:#?}", fw.header);
        return Ok(fw);
    }
        
    panic!("error");
}

#[derive(Debug)]
pub struct FwFile {
    header: Header,
    data: Vec::<u8>,
}

impl FwFile {
    pub fn data(&self) -> Vec::<u8> {
        self.data.to_vec()
    }
    pub fn header(&self) {
        println!("decoded header as: {:#?}", self.header);
    }
}

fn decode_fw_file(data: &[u8]) -> Option<FwFile> {
    // parse header to validate it
    let mut buf = Vec::<u8>::new();
    let mut pos: usize = 0;
    for b in data.iter() {
        if *b == 35 && pos == 0 {
            println!("found header start byte.. pos={} continuing", pos);
            pos = pos+1;
            continue;
        }
        if *b == 35 && pos != 0 {
            println!("found header end byte.. pos={} breaking", pos);
            break;
        }
        buf.push(*b);
        pos = pos+1;
    }
    if buf.len() < 2 {
        println!("no valid header found!");
        return None;
    }
    let hbuf: Vec<&str> = str::from_utf8(&buf).unwrap().split(|c| c == ';').collect();
    let header = decode_fw_image_header(hbuf.as_slice()).unwrap();

    // Just for interest, extract out the data?
    let v = data[pos..].as_ref();
    //let v: Vec<&str> = str::from_utf8(data).unwrap().split(|c| c == ';').collect();
    if v.len() > 2 {
       // let data = v[3].to_string();
       // if data.as_bytes()[0] != 35 {
       //     println!("no valid data start byte");
       //     return None;
       // }
        // dump hex data portion out to fw.hex
        let mut hfile = File::create("fw.hex").unwrap();
        hfile.write(v[1..].as_ref()).unwrap(); // skip the first byte == '#'
        //hfile.write(data[1..].as_ref()); // skip the first byte == '#'
        //hfile.write(data[1..].replace("\r", "").as_ref()); // skip the first byte == '#'
        //for s in data.replace("\r", "").bytes() { //.split(|x| x == '\n' ) {
            //println!("{}", s);
        //    hfile.write(s);//.as_bytes());
        //}
    //println!("v = {:x?}", v);
        return Some(FwFile{
            header: header,
            data: v[1..].to_vec(),
        });
    }
    None
}

#[derive(Debug)]
struct Header {
    image:   String,
    version: f32,
    flags:   String,
}

fn validate_fw_image_header(s: &str) -> bool {
    if s == "ACU Firmware" || s == "*ACU Firmware" || s == "+ACU Firmware" { true } else { false }
}

// fw_info == split(';'),
//   - str(fw_info[0]) => header
//   - u32(fw_info[2]) => flags
//   - f32(fw_info[1])*100.0 => version
fn decode_fw_image_header(v: &[&str]) -> Option<Header> {
    //let v: Vec<&[u8]> = data.split(|b| *b == b';').collect();
    //if v.len() > 2 {
    //    let v: Vec<&str> = str::from_utf8(data).unwrap().split(|c| c == ';').collect();
        let flags = v[2].parse::<u32>().unwrap();
        let f = decode_fw_image_flags(flags).unwrap();
        let s = v[0].to_string();

        if !validate_fw_image_header(&s) {
            println!("unknown image type");
            return None;
        }
        return Some(Header{
            image: s,
            version: v[1].parse::<f32>().unwrap(),
            flags: f,
        });
    //}
//    None
}

// if (version >= 3.05) {
//    flags &1  != 0 => "Street-Mode" // FWOperatingMode (byte)170
//    flags &2  != 0 => "Racing 5S"   // FWOperatingMode (byte)221
//    flags &32 != 0 => "Race"        // FWOperatingMode (byte)204
//    flags &64 != 0 => "Racing 3S"   // FWOperatingMode (byte)187
//
 // conditions:
 //    if buf[0] == "ACU Firmware"
 //    if buf[1] >= 3.05f (check fw ver)
 //    v := buf[2]; (the flags to decode here..)
fn decode_fw_image_flags(flags: u32) -> Option<String> {
    if flags & 1 != 0 { // FWOperatingMode = (byte)170
        return Some("Street-Mode".to_string());
    } else if flags & 2 != 0 { // FWOperatingMode = (byte)221
        return Some("ABP 5Sensor-Mode".to_string());
    } else if flags & 32 != 0 { // FWOperatingMode = (byte)187
        return Some("Race-Mode".to_string());
    } else if flags & 64 != 0 { // FWOperatingMode = (byte)204
        return Some("Race vest Street-Mode".to_string());
    };
//    // &4 - SD-Card present
//    // &8 - GPS-Module present
//    // &16 - Open-Loop-Detection disabled
    None
}

fn decrypt<R: Read, W: Write>(mut idata: R, mut odata: W, key: &[u8], iv: &[u8])
    -> Result<(), symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut buffer = Vec::new();
    idata.read_to_end(&mut buffer).unwrap();

    let mut obuf = [0u8; 128];
    let mut write_buf = buffer::RefWriteBuffer::new(&mut obuf);
    let mut read_buf = buffer::RefReadBuffer::new(&mut buffer);//&mut ibuf);
    loop {
        let ret = decryptor.decrypt(&mut read_buf, &mut write_buf, true)?;
        odata.write(write_buf.take_read_buffer().take_remaining()).unwrap();
        match ret {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => { }
        }
    }

    Ok(())
}
