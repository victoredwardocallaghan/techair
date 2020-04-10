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

#[macro_use]
extern crate clap;
use clap::App;


fn parse_supply(m: &clap::ArgMatches) {
    match m.subcommand_name() {
        Some("logic")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::Logic).unwrap();
            println!("{:02} V", s);
        },
        Some("peripheral")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::Peripheral).unwrap();
            println!("{:02} V", s);
        },
        Some("right-hand")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::RightHand).unwrap();
            println!("{:02} V", s);
        },
        Some("left-hand")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::LeftHand).unwrap();
            println!("{:02} V", s);
        },
        Some("right-foot")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::RightFoot).unwrap();
            println!("{:02} V", s);
        },
        Some("left-foot")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::LeftFoot).unwrap();
            println!("{:02} V", s);
        },
        Some("squib")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::Squib).unwrap();
            println!("{:02} V", s);
        },
        Some("battery")       => {
            let s = techair::cli::get_measure_voltage(techair::cli::MeasureVoltageType::Battery).unwrap();
            println!("{:02} V", s);
        },
        Some("charging-state")       => {
            let s = techair::cli::get_charging_state().unwrap();
            println!("{}", s);
        },
        Some("zip-state")       => {
            let s = techair::cli::get_zip_state().unwrap();
            if s {
                println!("ZIP Closed");
            } else {
                println!("ZIP Open");
            }
//            techair::cli::set_led_state(0x03);
        },
        None => {
		println!("No subcommand was used");
	},
        _ => {
		println!("Some other subcommand was used");
	}
    }
}

fn sensor(m: &clap::ArgMatches) {
    match m.subcommand() {
        ("revision", Some(ms))  => {
		println!("revision --flag");
		if ms.is_present("right-hand") {
                        let (ma,mi) = techair::cli::get_sensor_revision(techair::cli::SensorRevisionType::RightHand).unwrap();
                        println!("rev {:03}, {:03}", ma, mi);
		}
		if ms.is_present("left-hand") {
                        let (ma,mi) = techair::cli::get_sensor_revision(techair::cli::SensorRevisionType::LeftHand).unwrap();
                        println!("rev {:03}{:03}", ma, mi);
		}
		if ms.is_present("right-foot") {
                        let (ma,mi) = techair::cli::get_sensor_revision(techair::cli::SensorRevisionType::RightFoot).unwrap();
                        println!("rev {:03}{:03}", ma, mi);
		}
		if ms.is_present("left-foot") {
                        let (ma,mi) = techair::cli::get_sensor_revision(techair::cli::SensorRevisionType::LeftFoot).unwrap();
                        println!("rev {:03}{:03}", ma, mi);
		}
	},
        ("accelerometer", Some(ms))  => {
		println!("accelerometer");
		if ms.is_present("right-hand") {
			println!("right-hand!");
		}
		if ms.is_present("left-hand") {
			println!("left-hand!");
		}
		if ms.is_present("body") {
			println!("body!");
		}
	},
        ("gyroscope", Some(_))      => println!("gyro"),
        ("", None) => println!("No subcommand was used"),
        // If all subcommands are defined above, anything else is unreachabe!()
	_ => unreachable!(),
    }
}

fn logs(m: &clap::ArgMatches) {
    match m.subcommand() {
        ("no-of-precrash", Some(_)) => {
            let s = techair::cli::get_no_of_precrash_logs().unwrap();
            println!("{:?}", s);
        },
        ("no-of-postcrash", Some(_)) => {
            let s = techair::cli::get_no_of_postcrash_logs().unwrap();
            println!("{:?}", s);
        },
        ("no-of-errors", Some(_)) => {
            let s = techair::cli::get_no_of_errors().unwrap();
            println!("{:?}", s);
        },
        ("bat-count", Some(_)) => {
            let s = techair::cli::get_bat_count().unwrap();
            println!("{:?}", s);
        },
        ("error-history", Some(_)) => {
            let s = techair::cli::get_error_history().unwrap();
            println!("{:?}", s);
        },
        ("op-hours", Some(_)) => {
            let s = techair::cli::get_op_hours().unwrap();
            println!("{:?}", s);
        },
        ("", None) => println!("No subcommand was used"),
        // If all subcommands are defined above, anything else is unreachabe!()
	_ => unreachable!(),
    }
}

fn main() { //-> Result<(), std::io::Error> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();

    // ...
 //       let cmd = techair::usbcmd::UsbCmd::try_from([0x00, 0x02, 0x3E, 0x81].to_vec());
//        techair::usbcmd::print_cmd(&cmd.unwrap());
//        println!("{:#?}", &cmd.unwrap());

   match m.subcommand() {
	   ("logs", Some(logs_m)) => {
                    logs(&logs_m);
	   }
	   ("supply", Some(supply_m)) => {
		   parse_supply(&supply_m);
//		   let v = techair::cli::get_supply_voltage(s);
//		   println!("{:?}", v);
	   }
	   ("sensor", Some(sensor_m)) => {
		   sensor(&sensor_m);
	   }
	   ("customer-info", _)       => {
		   let s = techair::cli::get_customer_info().unwrap();
		   println!("{}", s);
	   }
	   ("serial", _)              => {
		   if let Some(s) = techair::cli::get_serial_nr() {
                       println!("{}", s);
                   }
	   }
	   ("sw-version", _)          => {
		   let s = techair::cli::get_sw_version().unwrap();
		   println!("{}", s);
	   }
	   ("inflation-type", _)  => {
                   // TODO: move this
//		   let (sr0, sr1) = techair::cli::get_calib_squib_res();
//                   if let Some(r) = sr0 {
//                       println!("squib resistor 1 = {:?} Ohms", r);
//                   }
//                   if let Some(r) = sr1 {
//                       println!("squib resistor 2 = {:?} Ohms", r);
//                   }

		   let s = techair::cli::get_inflation_type().unwrap();
		   println!("{}", s);
	   }
	   ("service-date", _)    => {
		   let s = techair::cli::get_service_date().unwrap();
		   println!("{}", s);
	   }
	   ("operating-modus", _) => {
		   let s = techair::cli::get_operating_modus().unwrap();
		   println!("{:?}", s);
	   }
	   ("firmware", Some(m)) => {
                   println!("{:?}", m.value_of("upgrade"));
                   let fw_file = techair::crypto::idk().unwrap();
                   //println!("{:?}", fw_file.unwrap().header());
                   //println!("{:?}", fw_file.unwrap().data());
                 let mut bl = techair::cli::SWUpdateBootloader::new();
                 if bl.xfer_swupdate_bootloader_hexfile(fw_file).is_err() {
                     println!("upgrade failed");
                 }
                 println!("clearing post-crash logs..");
                 techair::cli::clear_postcrash_log().unwrap();
                 //println!("resetting inflation-type..");
                 //techair::cli::set_inflation_type(0xFF).unwrap(); // 0xbb - double, 0x44 - single
                 println!("setting algorithm default thresholds..");
                 techair::cli::set_algorithm_default_thresholds().unwrap();
                 println!("init algorithms..");
                 techair::cli::init_algorithm().unwrap();
                 println!("FW upgrade complete!");
	   }
	   ("bootloader-version", Some(_)) => {
                    let mut bl = techair::cli::SWUpdateBootloader::new();
                    let s = bl.get_swupdate_bootloader_version().unwrap();
                    println!("bootloader version = {}", s);
	   }
	   ("bootloader-state", Some(_)) => {
                    let mut bl = techair::cli::SWUpdateBootloader::new();
                    let s = bl.get_swupdate_bootloader_state().unwrap();
                    println!("bootloader state = {:?}", s);
	   }
           // If no subcommand was usd it'll match the tuple ("", None)
	   ("", None) => println!("No subcommand was used"),
           // If all subcommands are defined above, anything else is unreachabe!()
	   _ => unreachable!(),
   }

}
