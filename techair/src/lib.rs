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

pub mod cli;
pub mod usbcmd;
pub mod serial;
pub mod encoder;
pub mod crypto;

// export the pub interface to cmd/mod.rs
pub mod cmd;

pub use crate::cmd::prelude::*;
pub use crate::usbcmd::UsbCmd;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::TechAirEncoder;

    #[test]
    fn get_sw_version() {
        let cmd = UsbCmd::General(GeneralCmd::GetSoftwareVersion(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x00,0x02,0x80,0x71]);
    }
    #[test]
    fn get_operating_modus() {
        let cmd = UsbCmd::General(GeneralCmd::GetOperatingModus(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x00,0x03,0x41,0xb1]);
    }
    #[test]
    fn get_serial_nr() {
        let cmd = UsbCmd::General(GeneralCmd::GetSerialNr(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x00,0x04,0x00,0x73]);
    }
    #[test]
    fn get_customer_info() {
        let cmd = UsbCmd::General(GeneralCmd::GetCustomerInfo(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x00,0x08,0x00,0x76]);
    }
    #[test]
    fn get_service_date() {
        let cmd = UsbCmd::General(GeneralCmd::GetServiceDate(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x00,0x0a,0x81,0xb7]);
    }


    #[test]
    fn get_op_hours() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetOPHours(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x00,0x00,0x20]);
    }
    #[test]
    fn clear_op_hours() {
        let cmd = UsbCmd::Logging(LoggingCmd::ClearOPHours);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x01,0xc1,0xe0]);
    }
    #[test]
    pub fn get_no_of_errors() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetNumOfErrors(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x02,0x81,0xe1]);
    }
    //GetErrorEntry,
    #[test]
    fn clear_error_history() {
        let cmd = UsbCmd::Logging(LoggingCmd::ClearErrorHistory);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x04,0x01,0xe3]);
    }
    #[test]
    fn get_no_of_precrash_logs() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetNumOfPreCrashLogs(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x05,0xc0,0x23]);
    }
    #[test]
    fn get_precrash_logs() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetPreCrashLogs(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x06,0x80,0x22]);
    }
    #[test]
    fn clear_precrash_log() {
        let cmd = UsbCmd::Logging(LoggingCmd::ClearPreCrashLog);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x07,0x41,0xe2]);
    }
    #[test]
    fn get_no_of_postcrash_logs() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetNumOfPostCrashLogs(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x08,0x01,0xe6]);
    }
    //GetPostCrashBulk,
    #[test]
    fn clear_postcrash_log() {
        let cmd = UsbCmd::Logging(LoggingCmd::ClearPostCrashLog);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x0a,0x80,0x27]);
    }
    //GetPreCrashBulk,
    #[test]
    fn get_error_history() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetErrorHistory(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x0c,0x00,0x25]);
    }
    #[test]
    fn get_postcrash_logs() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetPostCrashLogs(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x0d,0xc1,0xe5]);
    }
    #[test]
    fn get_bat_count() {
        let cmd = UsbCmd::Logging(LoggingCmd::GetBatCount(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x01,0x0e,0x81,0xe4]);
    }


    #[test]
    fn get_charging_state() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetChargingState(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x08,0x00,0x86]);
    }
    #[test]
    fn get_zip_state() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetZIPSwitchState(false));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x09,0xc1,0x46]);
    }
    #[test]
    fn set_led_state() {
        let state = 0xFF;
        let cmd = UsbCmd::Measure(MeasureCmd::SetEXTDisplay(state)); // USBSetLEDs()
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x0a,0xff,0xc7,0x20]);
    }
    #[test]
    fn get_logic_voltage() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetLogicVoltage(0.00));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x00,0x01,0x40]);
    }
    #[test]
    fn get_peripheral_voltage() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetPeripheralVoltage(0.00));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x01,0xc0,0x80]);
    }
    #[test]
    fn get_right_hand_voltage() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetRightHandVoltage(0.00));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x02,0x80,0x81]);
    }
    #[test]
    fn get_left_hand_voltage() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetLeftHandVoltage(0.00));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x03,0x41,0x41]);
    }
    #[test]
    fn get_right_foot_voltage() {
        let cmd = UsbCmd::Measure(MeasureCmd::GetRightFootVoltage(0.00));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x03,0x04,0x00,0x83]);
    }

    #[test]
    fn enable_sensor_readings() {
        let mask = Some(0x01 | 0x05 | 0x15 | 0x35); // found via transaction traces?
        let cmd = UsbCmd::Sensor(SensorCmd::EnableSensorReading(mask)); // FIXME: this should consume a enable mask..
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x04,0x00,0x35,0xf0,0x16]);
    }
    #[test]
    fn get_right_hand_sensor_revision() {
        let cmd = UsbCmd::Sensor(SensorCmd::GetSWVRH((0.00, 0.00)));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x04,0x08,0x02,0xb6]);
    }
    #[test]
    fn get_left_hand_sensor_revision() {
        let cmd = UsbCmd::Sensor(SensorCmd::GetSWVLH((0.00, 0.00)));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x04,0x09,0xc3,0x76]);
    }
    #[test]
    fn get_right_foot_sensor_revision() {
        let cmd = UsbCmd::Sensor(SensorCmd::GetSWVRF((0.00, 0.00)));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x04,0x0a,0x83,0x77]);
    }
    #[test]
    fn get_left_foot_sensor_revision() {
        let cmd = UsbCmd::Sensor(SensorCmd::GetSWVLF((0.00, 0.00)));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x04,0x0b,0x42,0xb7]);
    }

    #[test]
    fn init_algorithm() {
        let cmd = UsbCmd::Algorithm(AlgorithmCmd::InitAlgorithm);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x05,0x00,0x02,0xe0]);
    }
    #[test]
    fn get_algorithm_thresholds() {
        let cmd = UsbCmd::Algorithm(AlgorithmCmd::GetAlogrithmThresholds(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x05,0x04,0x03,0x23]);
    }
    #[test]
    fn set_algorithm_default_thresholds() {
        let cmd = UsbCmd::Algorithm(AlgorithmCmd::SetAlogrithmDefaultThresholds(None));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x05,0x06,0x82,0xe2]);
    }

    #[test]
    fn get_calib_squib_res() {
        let cmd = UsbCmd::Airbag(AirbagCmd::GetCalibSquibRes( (None, None) ));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x06,0x08,0x03,0xd6]);
    }
    #[test]
    fn get_inflation_type() {
        let cmd = UsbCmd::Airbag(AirbagCmd::GetInflationType(0x00));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x06,0x0a,0x82,0x17]);
    }
    #[test]
    fn set_inflation_type() {
        let it = 0xFF;
        let cmd = UsbCmd::Airbag(AirbagCmd::SetInflationType(it));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x06,0x0b,0xff,0xd6,0xb1]);
    }

    #[test]
    fn start_bootloader() {
        let cmd = UsbCmd::SWUpdate(SWUpdateCmd::StartBootLoader);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x20,0x00,0x18,0x70]);
    }
    #[test]
    fn get_swupdate_bootloader_version() {
        let cmd = UsbCmd::SWUpdate(SWUpdateCmd::GetBootLoaderVersion( None ));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x20,0x01,0xd9,0xb0]);
    }
    #[test]
    fn get_swupdate_bootloader_state() {
        let cmd = UsbCmd::SWUpdate(SWUpdateCmd::GetBootLoaderState( None ));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x20,0x02,0x99,0xb1]);
    }
    #[test]
    fn xfer_swupdate_bootloader_hexfile() {
        let chunk = [0x00,0x01,0x02];
        let page_count = 0xCC;
        let fw_data = FWData::new(&chunk, &page_count).unwrap();
        let cmd = UsbCmd::SWUpdate(SWUpdateCmd::WriteFWData( fw_data ));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x20,0x03,0x00,0xcc,0x00,0x03,0x00,0x01,0x02,0x42,0xfd]);
    }
    #[test]
    fn quit_bootloader() {
        let cmd = UsbCmd::SWUpdate(SWUpdateCmd::QuitBootLoader);
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x20,0x04,0x19,0xb3]);
    }
    #[test]
    fn crccheck_swupdate_bootloader_hexfile() {
        let hex_crc16 = 0xAABB;
        let cmd = UsbCmd::SWUpdate(SWUpdateCmd::CRCCheck( hex_crc16 ));
        let mut buf: Vec<u8> = Vec::new();
        cmd.write_bytes(&mut buf);
        assert_eq!(buf, [0x20,0x05,0xaa,0xbb,0x25,0x36]);
    }
}
