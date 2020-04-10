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

use crate::crypto::FwFile;
use crate::serial::TechAir;
use crate::usbcmd::UsbCmd;
use crate::cmd::prelude::*;

use std::io::{Error, ErrorKind};
use std::time::Duration;

/////////
/// General.
/// //.

pub fn get_sw_version() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    // gets sw version
    if ta.write(UsbCmd::General(GeneralCmd::GetSoftwareVersion(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        println!("packet = {:#?}", packet);
        if let Some(GeneralCmd::GetSoftwareVersion(ver)) = packet.general() {
            return ver.map(|v| v.to_string());
        }
    }
    None
}
pub fn get_operating_modus() -> Option<OpModus> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    // gets sw version
    if ta.write(UsbCmd::General(GeneralCmd::GetOperatingModus(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        println!("packet = {:#?}", packet);
        if let Some(GeneralCmd::GetOperatingModus(modus)) = packet.general() {
            return modus;
        }
    }
    None
}
pub fn get_serial_nr() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    // gets serial number
    if ta.write(UsbCmd::General(GeneralCmd::GetSerialNr(None))).is_err() {
        println!("write went bad!");
        return None;
    }
    println!("reading..");
    if let Ok(packet) = ta.read() {
        println!("read!");
        if let Some(GeneralCmd::GetSerialNr(ver)) = packet.general() {
            return ver;
        }
    }
    None
}
pub fn get_customer_info() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    // gets customer info
    if ta.write(UsbCmd::General(GeneralCmd::GetCustomerInfo(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(GeneralCmd::GetCustomerInfo(info)) = packet.general() {
            return info;
        }
    }
    None
}
pub fn get_service_date() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    // gets service date
    if ta.write(UsbCmd::General(GeneralCmd::GetServiceDate(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(GeneralCmd::GetServiceDate(date)) = packet.general() {
            return date;
        }
    }
    None
}


/////////
/// Logging.
/// //.

pub fn get_op_hours() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetOPHours(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetOPHours(hours)) = packet.logging() {
            return hours;
        }
    }
    None
}
pub fn clear_op_hours() -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    ta.write(UsbCmd::Logging(LoggingCmd::ClearOPHours))
}
pub fn get_no_of_errors() -> Option<u8> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetNumOfErrors(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetNumOfErrors(errors)) = packet.logging() {
            return errors;
        }
    }
    None
}
//GetErrorEntry,
pub fn clear_error_history() -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    ta.write(UsbCmd::Logging(LoggingCmd::ClearErrorHistory))
}
pub fn get_no_of_precrash_logs() -> Option<u16> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetNumOfPreCrashLogs(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetNumOfPreCrashLogs(nlogs)) = packet.logging() {
            return nlogs;
        }
    }
    None
}
pub fn get_precrash_logs() -> Option<Vec::<u8>> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetPreCrashLogs(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetPreCrashLogs(logs)) = packet.logging() {
            return logs;
        }
    }
    None
}
pub fn clear_precrash_log() -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    ta.write(UsbCmd::Logging(LoggingCmd::ClearPreCrashLog))
}
pub fn get_no_of_postcrash_logs() -> Option<u16> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetNumOfPostCrashLogs(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetNumOfPostCrashLogs(nlogs)) = packet.logging() {
            return nlogs;
        }
    }
    None
}
//GetPostCrashBulk,
pub fn clear_postcrash_log() -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    ta.write(UsbCmd::Logging(LoggingCmd::ClearPostCrashLog))
}
//GetPreCrashBulk,
pub fn get_error_history() -> Option<Vec::<u8>> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetErrorHistory(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetErrorHistory(errors)) = packet.logging() {
            return errors;
        }
    }
    None
}
pub fn get_postcrash_logs() -> Option<Vec::<u8>> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetPostCrashLogs(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetPostCrashLogs(logs)) = packet.logging() {
            return logs;
        }
    }
    None
}
pub fn get_bat_count() -> Option<u16> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Logging(LoggingCmd::GetBatCount(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(LoggingCmd::GetBatCount(bcount)) = packet.logging() {
            return bcount;
        }
    }
    None
}
//GetPreCrashENCBulk,
//GetPostCrashENCBulk,


/////////
/// Measure.
/// //.

pub fn get_charging_state() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    // gets the charging state
    let p = UsbCmd::Measure(MeasureCmd::GetChargingState(None));
    if ta.write(p).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(MeasureCmd::GetChargingState(data)) = packet.measure() {
            return data;
        }
    }
    None
}
pub fn get_zip_state() -> Option<bool> {
    let mut ta = TechAir::new().unwrap();
    // gets the zip state
    let p = UsbCmd::Measure(MeasureCmd::GetZIPSwitchState(false));
    if ta.write(p).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(MeasureCmd::GetZIPSwitchState(data)) = packet.measure() {
            return Some(data);
        }
    }
    None
}
pub fn set_led_state(state: u8) {
    let mut ta = TechAir::new().unwrap();
    let p = UsbCmd::Measure(MeasureCmd::SetEXTDisplay(state)); // USBSetLEDs()
    ta.write(p).unwrap();
    if let Ok(packet) = ta.read() {
        if let Some(MeasureCmd::SetEXTDisplay(data)) = packet.measure() {
            println!("SetEXTDisplay data ret={}", data);
        }
    }
}

pub enum MeasureVoltageType {
    Logic,
    Peripheral,
    RightHand,
    LeftHand,
    RightFoot,
    LeftFoot,
    Squib,
    Battery,
}

pub fn get_measure_voltage(mvt: MeasureVoltageType) -> Option<f32> {
    let mut ta = TechAir::new().unwrap();
    // gets voltage
    match mvt {
        MeasureVoltageType::Logic => {
            let p = UsbCmd::Measure(MeasureCmd::GetLogicVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetLogicVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::Peripheral => {
            let p = UsbCmd::Measure(MeasureCmd::GetPeripheralVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetPeripheralVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::RightHand => {
            let p = UsbCmd::Measure(MeasureCmd::GetRightHandVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetRightHandVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::LeftHand => {
            let p = UsbCmd::Measure(MeasureCmd::GetLeftHandVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetLeftHandVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::RightFoot => {
            let p = UsbCmd::Measure(MeasureCmd::GetRightFootVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetRightFootVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::LeftFoot => {
            let p = UsbCmd::Measure(MeasureCmd::GetLeftFootVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetLeftFootVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::Squib => {
            let p = UsbCmd::Measure(MeasureCmd::GetSquibVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetSquibVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
        MeasureVoltageType::Battery => {
            let p = UsbCmd::Measure(MeasureCmd::GetBatteryVoltage(0.00));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(MeasureCmd::GetBatteryVoltage(data)) = packet.measure() {
                    return Some(data);
                }
            }
        },
    }
    None
}


/////////
/// Sensor.
/// //.

pub enum SensorRevisionType {
    RightHand,
    LeftHand,
    RightFoot,
    LeftFoot,
}

pub fn get_sensor_revision(srt: SensorRevisionType) -> Option<(f32, f32)> {
    let mut ta = TechAir::new().unwrap();
    // enable sensor readings
    let mask = Some(0x01 | 0x05 | 0x15 | 0x35); // found via transaction traces?
    let p = UsbCmd::Sensor(SensorCmd::EnableSensorReading(mask)); // FIXME: this should consume a enable mask..
    if ta.write(p).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(SensorCmd::EnableSensorReading(data)) = packet.sensor() {
            println!("EnableSensorReading data = {:?}", data);
        }
    }
    // gets revision info
    match srt {
        SensorRevisionType::RightHand => {
            let p = UsbCmd::Sensor(SensorCmd::GetSWVRH((0.00, 0.00)));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(SensorCmd::GetSWVRH(data)) = packet.sensor() {
                    return Some(data);
                }
            }
        },
        SensorRevisionType::LeftHand => {
            let p = UsbCmd::Sensor(SensorCmd::GetSWVLH((0.00, 0.00)));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(SensorCmd::GetSWVLH(data)) = packet.sensor() {
                    return Some(data);
                }
            }
        },
        SensorRevisionType::RightFoot => {
            let p = UsbCmd::Sensor(SensorCmd::GetSWVRF((0.00, 0.00)));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(SensorCmd::GetSWVRF(data)) = packet.sensor() {
                    return Some(data);
                }
            }
        },
        SensorRevisionType::LeftFoot => {
            let p = UsbCmd::Sensor(SensorCmd::GetSWVLF((0.00, 0.00)));
            if ta.write(p).is_err() {
                return None;
            }
            if let Ok(packet) = ta.read() {
                if let Some(SensorCmd::GetSWVLF(data)) = packet.sensor() {
                    return Some(data);
                }
            }
        },
    }
    None
}


/////////
/// Algorithm.
/// //.

pub fn init_algorithm() -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    // init_algorithm
    ta.write(UsbCmd::Algorithm(AlgorithmCmd::InitAlgorithm))
}
pub fn get_algorithm_thresholds() -> Option<Vec::<u8>> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    if ta.write(UsbCmd::Algorithm(AlgorithmCmd::GetAlogrithmThresholds(None))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        if let Some(AlgorithmCmd::GetAlogrithmThresholds(th)) = packet.algorithm() {
            return th;
        }
    }
    None
}
pub fn set_algorithm_default_thresholds() -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    // set_algorithm_default_thresholds
    ta.write(UsbCmd::Algorithm(AlgorithmCmd::SetAlogrithmDefaultThresholds(None)))
}


/////////
/// Airbag.
/// //.

pub fn get_calib_squib_res() -> (Option<f32>, Option<f32>) {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    // gets GetCalibSquibRes()
    if ta.write(UsbCmd::Airbag(AirbagCmd::GetCalibSquibRes( (None, None) ))).is_err() {
        return (None, None);
    }
    if let Ok(packet) = ta.read() {
        println!("packet = {:#?}", packet);
        if let Some(AirbagCmd::GetCalibSquibRes(squibres)) = packet.airbag() {
            return squibres;
        }
    }
    (None, None)
}
pub fn get_inflation_type() -> Option<String> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    // gets inflation type
    if ta.write(UsbCmd::Airbag(AirbagCmd::GetInflationType(0x00))).is_err() {
        return None;
    }
    if let Ok(packet) = ta.read() {
        println!("packet = {:#?}", packet);
        if let Some(AirbagCmd::GetInflationType(inftyp)) = packet.airbag() {
            println!("inflation type = {:?}", inftyp);
            match inftyp {
                0x44 => return Some("single".to_string()),
                0xb4 => return Some("double-race".to_string()),
                0xbb => return Some("double".to_string()),
                0xff => return Some("clear".to_string()),
                _    => return None,
            }
        }
    }
    None
}
pub fn set_inflation_type(it: u8) -> Result<(), std::io::Error> {
    let mut ta = TechAir::new().unwrap();
    println!("{:#?}", ta);
    ta.write(UsbCmd::Airbag(AirbagCmd::SetInflationType(it)))
}


/////////
/// SWUpdate.
/// //.

pub struct SWUpdateBootloader {
    ta: TechAir,
}

impl SWUpdateBootloader {
    pub fn new() -> SWUpdateBootloader {
        let mut ta = TechAir::new().unwrap();
        start_bootloader(&mut ta);
        SWUpdateBootloader{
            ta: ta,
        }
    }

    pub fn get_swupdate_bootloader_version(&mut self) -> Option<u8> {
        // gets bootloader version
        if self.ta.write(UsbCmd::SWUpdate(SWUpdateCmd::GetBootLoaderVersion( None ))).is_err() {
            return None;
        }
        if let Ok(packet) = self.ta.read() {
            println!("packet = {:#?}", packet);
            if let Some(SWUpdateCmd::GetBootLoaderVersion(ver)) = packet.swupdate() {
                return ver;
            }
        }
        None
    }

    pub fn get_swupdate_bootloader_state(&mut self) -> Option<SWUpdateBootLoaderStates> {
        // gets bootloader state
        if self.ta.write(UsbCmd::SWUpdate(SWUpdateCmd::GetBootLoaderState( None ))).is_err() {
            return None;
        }
        if let Ok(packet) = self.ta.read() {
            println!("packet = {:#?}", packet);
            if let Some(SWUpdateCmd::GetBootLoaderState(state)) = packet.swupdate() {
                return state;
            }
        }
        None
    }

    //SWUpdateCmd::WriteFWData
    pub fn xfer_swupdate_bootloader_hexfile(&mut self, fwf: FwFile)
        -> Result<(), std::io::Error> {
        let mut hexfile = fwf.data();
        // as_chars(data: Vec::<u8>) -> Vec::<char>
        //
        //let mut crc16_state = State::<MODBUS>::new();
	let crc_fw = CRC16::calculate(&hexfile).as_u16();
        hexfile.push((crc_fw >>   8) as u8);
        hexfile.push((crc_fw & 0xff) as u8);
        let hexfile_len = hexfile.len();
        println!("writing out hexfile with len {:#04x?}", hexfile_len);
        let mut page_count = calculate_num_pages(hexfile_len);
        println!("calculated a page_count of {:#04x?}", page_count);
        // split buffer up and xfer hex data to bootloader
        for chunk in hexfile.chunks(256) {
            //println!("DEBUG: chunk len = {}", chunk.len());
            //crc16_state.update(chunk);
            //let crc = crc16_state.get().to_le_bytes();
	    //CRC16(crc)
            // None if wrong chunk sz.
            let fw_data = FWData::new(chunk, &page_count).unwrap();
            if chunk.len() >= 255 {
                page_count = page_count - 1;
            } else {
                // saturate to page_count = 0.
                page_count = 0;
            }
            self.ta.write(UsbCmd::SWUpdate(SWUpdateCmd::WriteFWData( fw_data )))?;
            self.ta.bytes_to_write_left();
            let packet = self.ta.read()?;
            println!("packet = {:#?}", packet);
            if let Some(SWUpdateCmd::WriteFWData(res)) = packet.swupdate() {
                // if returned page_count != num of bytes sent, then fail!.
                if res.page_count != page_count {
                    println!("{} != {}", res.page_count, page_count);
                    return Err(Error::new(ErrorKind::Other, "page_count fuck up!"));
                }
            }
    // gets ctrl mode? 
//    self.ta.write(UsbCmd::General(GeneralCmd::GetCtrlMode(None)))?;
//    if let Ok(packet) = self.ta.read() {
//        println!("packet = {:#?}", packet);
//        if let Some(GeneralCmd::GetCtrlMode(mode)) = packet.general() {
//            println!("DEBUG: ctrl_mode={:?}", mode);
//        }
//    }
            if let Some(state) = self.get_swupdate_bootloader_state() {
                println!("state: {:?}", state);
            }
        }
        loop { // XXX
            if let Some(state) = self.get_swupdate_bootloader_state() {
                println!("state: {:?}", state);
                //state: CheckReadFW
                //state: EraseFlash
                //state: FlashFW
                //state: WaitVerifyFlashedCRC
                if state == SWUpdateBootLoaderStates::WaitVerifyFlashedCRC {
                    break;
                }
            }
        }
        // XXX
        self.crccheck_swupdate_bootloader_hexfile(fwf)?;
        if let Some(state) = self.get_swupdate_bootloader_state() {
            println!("state: {:?}", state);
        }

        Ok(())
    }

    //SWUpdateCmd::CRCCheck
    fn crccheck_swupdate_bootloader_hexfile(&mut self, fwf: FwFile)
        -> Result<(), std::io::Error> {
        let bl_ver = self.get_swupdate_bootloader_version().unwrap();
        let hex_crc16: u16 = crate::crypto::crc16_fw_data(fwf, bl_ver)?;
        self.ta.write(UsbCmd::SWUpdate(SWUpdateCmd::CRCCheck( hex_crc16 )))?;
        println!("reading back crc check");
        self.ta.set_timeout(Duration::from_millis(3000))?;
        let packet = self.ta.read()?;
        if let Some(SWUpdateCmd::CRCCheck(ret)) = packet.swupdate() {
            if ret == 0 {
                return Ok(());
            } else {
                println!("xfer of fw fucked up! ret={}", ret);
            }
        }
        Err(Error::new(ErrorKind::Other, "xfer of fw fucked up!"))
    }
}

impl Drop for SWUpdateBootloader {
    fn drop(&mut self) {
        println!("SWUpdateBootloader::drop()");
        quit_bootloader(&mut self.ta);
    }
}

fn start_bootloader(ta: &mut TechAir) {
    //StartBootLoader,
    println!("StartBootLoader()");
    let p = UsbCmd::SWUpdate(SWUpdateCmd::StartBootLoader);
    if ta.write(p).is_err() {
        return;
    }
    if let Ok(packet) = ta.read() {
        if let Some(SWUpdateCmd::StartBootLoader) = packet.swupdate() {
            println!("StartBootLoader data");
        }
    }
}

fn quit_bootloader(ta: &mut TechAir) {
    //QuitBootLoader,
    println!("QuitBootLoader()");
    let p = UsbCmd::SWUpdate(SWUpdateCmd::QuitBootLoader);
    if ta.write(p).is_err() {
        return;
    }
    if let Ok(packet) = ta.read() {
        if let Some(SWUpdateCmd::QuitBootLoader) = packet.swupdate() {
            println!("QuitBootLoader data");
        }
    }
}

// XXX
//fn as_chars(data: Vec::<u8>) -> Vec::<char> {
//    data.into_iter().map(|b| b as char).collect()
//}
