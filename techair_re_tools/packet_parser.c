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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <ctype.h>

// link with -lm
#include <math.h>

typedef enum {
	GENERAL = 0x0,
	LOGGING = 0x1,
	POWER = 0x2,
	MEASURE = 0x3,
	SENSOR = 0x4,
	ALGORITHM = 0x5,
	AIRBAG = 0x6,
	SDCARD = 0x7,
	CONFIGURE = 0x10, // ???
	SW_UPDATE = 0x20,
} usb_modules_t;

typedef enum {
	GET_CTRL_MODE = 0x0,
	SET_CTRL_MODE = 0x1,
	GET_SOFTWARE_VERSION = 0x2,
	GET_OPERATING_MODUS = 0x3,
	GET_SERIALNR = 0x4,
	SET_SERIALNR = 0x5,
	GET_HARDWARE_VERSION = 0x6,
	SET_HARDWARE_VERSION = 0x7,
	GET_CUSTOMERINFO = 0x8,
	SET_CUSTOMERINFO = 0x9,
	GET_SERVICEDATE = 0xa,
	SET_SERVICEDATE = 0xb,
} general_cmd_t;

typedef enum {
	GET_OP_HOURS = 0x0,
	CLEAR_OP_HOURS = 0x1,
	GET_NUM_OF_ERRORS = 0x2,
	GET_ERROR_ENTRY = 0x3,
	CLEAR_ERROR_HISTORY = 0x4,
	GET_NUM_OF_PRECRASH_LOGS = 0x5,
	GET_PRECRASH_LOGS = 0x6,
	CLEAR_PRECRASH_LOG = 0x7,
	GET_NUM_OF_POSTCRASH_LOGS = 0x8,
	GET_POSTCRASH_BULK = 0x9,
	CLEAR_POSTCRASH_LOG = 0xa,
	GET_PRECRASH_BULK = 0xb,
	GET_ERROR_HISTORY = 0xc,
	GET_POSTCRASH_LOGS = 0xd,
	GET_BATCOUNT = 0xe,
	GET_PRECRASHENC_BULK = 0xf,
	GET_POSTCRASHENC_BULK = 0x10,
} usb_logging_cmd_t;

typedef enum {
	GET_SUPPLY_STATE = 0x0,
	EN_DIS_SUPPLY = 0x1,
} power_cmd_t;

typedef enum {
	GET_LOGIC_VOLTAGE = 0x0,
	GET_PERIPHERAL_VOLTAGE = 0x1,
	GET_RIGHT_HAND_VOLTAGE = 0x2,
	GET_LEFT_HAND_VOLTAGE = 0x3,
	GET_RIGHT_FOOT_VOLTAGE = 0x4,
	GET_LEFT_FOOT_VOLTAGE = 0x5,
	GET_SQUIB_VOLTAGE = 0x6,
	GET_BATTERY_VOLTAGE = 0x7,
	GET_CHARGING_STATE = 0x8,
	GET_ZIP_SWITCH_STATE = 0x9,
	SET_EXT_DISPLAY = 0xa, // USBSetLEDs()
} measure_cmd_t;

typedef enum {
	ENABLE_SENSOR_READING = 0x0,
	GET_SENSOR_READING_ENABLES = 0x1,
	GET_RIGHT_HAND_ACCEL = 0x2,
	GET_LEFT_HAND_ACCEL = 0x3,
	GET_RIGHT_FOOT_ACCEL = 0x4,
	GET_LEFT_FOOT_ACCEL = 0x5,
	GET_BODY_ACCEL = 0x6,
	GET_GYROSCOPE = 0x7,
	GET_SWV_RH = 0x8,
	GET_SWV_LH = 0x9,
	GET_SWV_RF = 0xa,
	GET_SWV_LF = 0xb,
} usb_sensor_cmd_t;

typedef enum {
	INIT_ALGORITHM = 0x0,
	DO_SINGLE_SAMPLE_CALC = 0x1,
	GET_SINGLE_SAMPLE_CALC_STATE = 0x2,
	GET_SINGLE_SAMPLE_CALC_RESULT = 0x3,
	GET_ALOGRITHM_THRESHOLDS = 0x4,
	SET_ALOGRITHM_THRESHOLDS = 0x5,
	SET_ALOGRITHM_DEFAULT_THRESHOLDS = 0x6,
} usb_algorithm_cmd_t;

typedef enum {
	GET_IGNITION_CTRL_MODE = 0x0,
	GET_IGNITION_CTRL_STATUS = 0x1,
	INIT_IGNITION_CTRL = 0x2,
	DIAG_IGNITION_CTRL = 0x3,
	ARM_IGNITION_CTRL = 0x4,
	FIRE_AIRBAG = 0x5,
	RESET_IGNITION_CTRL = 0x6,
	DIAG_GET_SQUIBRES = 0x7,
	GET_CALIBSQUIBRES = 0x8,
	SET_CALIBSQUIBRES = 0x9,
	GET_INFLATIONTYPE = 0xa,
	SET_INFLATIONTYPE = 0xb,
} usb_airbag_cmd_t;

typedef enum {
	INIT_SDCARD_START = 0x0,
	GET_SDCARDRECORDINFO_START = 0x1,
	GET_SDCARDSENSORDATA_BULK_START = 0x2,
	CLEAR_SDCARDRECORDS_START = 0x3,
} usb_sdcard_commands;

// ?? CONFIGURE = 0x10,

typedef enum {
	START_BOOTLOADER = 0x0,
	GET_BOOTLOADERVERSION = 0x1,
	GET_BOOTLOADERSTATE = 0x2,
	WRITE_FWDATA = 0x3,
	QUIT_BOOTLOADER = 0x4,
	CRCCHECK = 0x5,
} usb_swupdate_commands;


// --- values:

typedef enum {
	STREET = 0xaa,
	RACE_3S = 0xbb,
	RACE = 0xdd,
	RACEVESTSTREET = 0xcc,
} acu_operating_mode_t;

typedef enum {
	BOOTLOADERSTATE_WAITFW = 0x0,
	BOOTLOADERSTATE_CHECKREADFW = 0x1,
	BOOTLOADERSTATE_ERASEFLASH = 0x2,
	BOOTLOADERSTATE_FLASHFW = 0x3,
	BOOTLOADERSTATE_CALCFLASHEDCRC = 0x4,
	BOOTLOADERSTATE_WAITVERIFYFLASHEDCRC = 0x5,
	BOOTLOADERSTATE_VERIFYFLASHEDCRCMEMORY = 0x6,
} usb_swupdate_states_t;


static const char * pp_usb_modules_t(usb_modules_t pt)
{
	switch (pt) {
		case GENERAL: return "<general cmd>";
		case LOGGING: return "<logging cmd>";
		case POWER: return "<power cmd>";
		case MEASURE: return "<measure cmd>";
		case SENSOR: return "<sensors>";
		case AIRBAG: return "<query>";
		default:
			      return "unknown packet type" ;
	}
}

static const char * pp_general_cmd_t(general_cmd_t ct)
{
	switch (ct) {
		case GET_CTRL_MODE:        return "GET_CTRL_MODE";
		case SET_CTRL_MODE:        return "SET_CTRL_MODE";
		case GET_SOFTWARE_VERSION: return "GET_SOFTWARE_VERSION";
		case GET_OPERATING_MODUS:  return "GET_OPERATING_MODUS";
		case GET_SERIALNR:         return "GET_SERIALNR";
		case SET_SERIALNR:         return "SET_SERIALNR";
		case GET_HARDWARE_VERSION: return "GET_HARDWARE_VERSION";
		case SET_HARDWARE_VERSION: return "SET_HARDWARE_VERSION";
		case GET_CUSTOMERINFO:     return "GET_CUSTOMERINFO";
		case SET_CUSTOMERINFO:     return "SET_CUSTOMERINFO";
		case GET_SERVICEDATE:      return "GET_SERVICEDATE";
		case SET_SERVICEDATE:      return "SET_SERVICEDATE";
		default:
			      return "*unknown general cmd type*";
	}
}

static const char * pp_power_cmd_t(power_cmd_t pt)
{
	switch (pt) {
		case GET_SUPPLY_STATE: return "GET_SUPPLY_STATE";
		case EN_DIS_SUPPLY:    return "EN_DIS_SUPPLY";
		default:
			      return "*unknown power cmd type*";
	}
}

static const char * pp_measure_cmd_t(measure_cmd_t mt)
{
	switch (mt) {
		case GET_LOGIC_VOLTAGE:      return "GET_LOGIC_VOLTAGE";
		case GET_PERIPHERAL_VOLTAGE: return "GET_PERIPHERAL_VOLTAGE";
		case GET_RIGHT_HAND_VOLTAGE: return "GET_RIGHT_HAND_VOLTAGE";
		case GET_LEFT_HAND_VOLTAGE:  return "GET_LEFT_HAND_VOLTAGE";
		case GET_RIGHT_FOOT_VOLTAGE: return "GET_RIGHT_FOOT_VOLTAGE";
		case GET_LEFT_FOOT_VOLTAGE:  return "GET_LEFT_FOOT_VOLTAGE";
		case GET_SQUIB_VOLTAGE:      return "GET_SQUIB_VOLTAGE";
		case GET_BATTERY_VOLTAGE:    return "GET_BATTERY_VOLTAGE";
		case GET_CHARGING_STATE:     return "GET_CHARGING_STATE";
		case GET_ZIP_SWITCH_STATE:   return "GET_ZIP_SWITCH_STATE";
		case SET_EXT_DISPLAY:        return "SET_EXT_DISPLAY";
		default:
			      return "*unknown measure cmd type*";
	}
}

static unsigned parse_packet(const char *packet, uint8_t buf[])
{
	unsigned i = 0;
	char * tmp = strdup(packet);

	char * tok = strtok(tmp, " ");
	for (; tok; tok = strtok(NULL, " ")) {
		//printf("DEBUG: got token %s\n", tok);
		if (buf)
			buf[i] = strtoul(tok, NULL, 16);
		i++;
	}

	return i;
}

/* Function to calculate MODBUS CRC. */
static uint16_t crc16(uint8_t buf[], unsigned len) {
	/* MODBUS CRC initial value is 0xFFFF. */
	uint16_t crc = 0xFFFF;

	for (unsigned pos = 0; pos < len; pos++) {
		crc ^= (uint16_t) buf[pos];
		for (unsigned i = 8; i != 0; i--) {
			if (crc & 1)
				crc = (crc >> 1) ^ 0xA001;
			else
				crc = (crc >> 1);
		}
	}

	return crc;
}

static uint16_t swap_bytes(uint16_t b)
{
	return ((b>>8)&0x00ff | (b<<8)&0xff00);
}

#if 0
static float decode_parscal_48bit_float(uint8_t v[])
{
	double exp_base = 129.0;
	double exp = v[0] - exp_base;
	double mantissa = 0.0;
	double value = 1.0;

	// calculate the mantissa
	for (int i = 5; i >= 1; i--) {
		// skip the sign bit
		int sb = (i == 5) ? 6 : 7;
		// for each bit
		for (int j = sb; j >= 0; j--) {
			value = value/2;
			// if bit is set
			if (((v[i] >> j) & 1) == 1)
				mantissa += value;
		}

	}
	if (mantissa == 1.0 && v[0] == 0)
		return 0.0;

	if ((v[5] & 0x80) == 1)
		mantissa = -mantissa;

	return (1 + mantissa) * pow(2.0, exp);
}

static double fixed16_to_double_wtf(fixed_point_t v)
{
	double s = (0x8000 & v) ? -1.0 : 1.0;
	return ((double)(v &0x7FFF)/ (double)(1 << 10));
}

static float magic16_to_float(uint16_t v)
{
	uint16_t s = swap_bytes(v);
	uint8_t n = ((s>>8)&0x00f0) >> 4;
//	printf(" n = 0x%02x\n", n);
	uint32_t f = (1024 + 512 * n);
	//uint32_t f = (1024 + 512 * ((uint32_t)v/5000));
	return ((float)v / f);
}

static uint16_t rev_decode(uint16_t v)
{
	return ((v&0xff00) | 0x0400) | ((v&0x00ff) >> 1);
}
#endif


typedef uint16_t fixed_point_t;
// 16bit fixed-point format: 6.10 (6 integral bits, 10 fractional bits).
//#define FIXED_POINT_FRACTIONAL_BITS 10

// 16bit fixed-point format: 13.3 (13 integral bits, 3 fractional bits).
#define FIXED_POINT_FRACTIONAL_BITS 3
static double fixed16_to_double(fixed_point_t v)
{
	//return ((double)v / (double)(1 << FIXED_POINT_FRACTIONAL_BITS));
	//double s = (0x8000 & v) ? -1.0 : 1.0;
	return ((double)v / pow(10, FIXED_POINT_FRACTIONAL_BITS));
	//return s * ((double)(v ^ 0x7FFF) / pow(10, FIXED_POINT_FRACTIONAL_BITS));
}
static double fixed16_to_double_2(fixed_point_t v)
{
	double s = (0x8000 & v) ? -1.0 : 1.0;
	if (s < 0)
		return s * ((double)((v & 0x7FFF) ^ 0x7FFF) / (1 << 15)); // 1<<15 = 2^15
	return s * ((double)v / (1 << 15)); // 1<<15 = 2^15
}

static int decode_packet(uint8_t buf[], unsigned n)
{
	if (n <= 1) {
		printf("DEBUG: null line.. skipping..\n");
		return -1;
	}

	if (n > 2) {
		uint16_t crc = swap_bytes(crc16(buf, n-2));
		if ((buf[n-2] != (crc>>8)&0xff)
		 || (buf[n-1] != (crc    &0xff))) {
			printf("DEBUG packet .crc16 = { ");
			printf("invalid! found: 0x%02X%02X, expected: ",
					buf[n-2], buf[n-1]);
			printf("0x%X", (int)crc);
			printf(" }\n");
			return -1;
		}
	}

	printf("DEBUG[%s]: Found! '%s' [0x%02x] with <n=%d> ",
			__func__, pp_usb_modules_t(buf[0]), buf[0], n);

	if (n > 3) {
		if (buf[0] == GENERAL) {
			printf("subtype: '%s' with word len=%d\n",
					pp_general_cmd_t(buf[1]), (n-4));
			if ((n - 4) == 0)
				return 0;

			if ((buf[1] == GET_CTRL_MODE) ||
			    (buf[1] == SET_CTRL_MODE)) {
				printf("CTRL_MODE: '");
				if ((n-4) == 2) {
					if (buf[3] == 0x71)
						printf("Ready(I think?)");
				}
				for (unsigned i = 0; i < (n-4); i++) {
					int v = buf[i+2];
					printf("0x%02X ", v);
				}
				printf("'.\n");
			}
			if ((buf[1] == SET_CUSTOMERINFO) ||
			    (buf[1] == GET_CUSTOMERINFO)) {
				//assert(n < 4)
				printf(" *> setting data len = '%d' bytes\n", buf[2]);
				if (buf[2] != (n-5)) {
					printf("bad len\n");
					return -1; // bad len.
				}
				printf("DATA:\n-----\n ");
				for (unsigned i = 0; i < buf[2]; i++) {
					char c = buf[3+i];
					if (!isascii(c))
						continue;
					if (!(i % 25))
						printf("\n");
					printf("%c ", c);
				}
				printf("\n-----.\n");
			}
			if (buf[1] == GET_OPERATING_MODUS) {
				printf("OPERATING MODUS: '");
				if ((n-4) < 2)
					return 0;
				switch (buf[2]) {
					case STREET: /*0xaa*/
						printf("STREET");
						break;
					case RACE_3S: /*0xbb*/
						printf("RACE_3S");
						break;
					case RACE: /*0xdd*/
						printf("RACE");
						break;
					case RACEVESTSTREET: /*0xcc*/
						printf("RACE.VEST.STREET");
						break;
					default:
						for (unsigned i = 0; i < (n-4); i++) {
							int v = buf[i+2];
							printf("0x%02X ", v);
						}
				}
				printf("'.\n");
			}
			if (buf[1] == GET_SOFTWARE_VERSION) {
				printf("SW Version: '");
				if ((n-4) != 2) {
					printf("error");
					// for (unsigned i = 0; i < (n-4); i++) {
					// 	int v = buf[i+2];
					// 	printf("0x%02X ", v);
					// }
					return -1;
				}
				printf("%.02f", 10*fixed16_to_double((buf[2]<<8)|buf[3]));
				printf("'.\n");
			}
			if ((buf[1] == GET_HARDWARE_VERSION) ||
			    (buf[1] == SET_HARDWARE_VERSION)) {
				printf("HW Version: '");
				for (unsigned i = 0; i < (n-4); i++) {
					int v = buf[i+2];
					printf("0x%02X ", v);
				}
				printf("'.\n");
			}
			if ((buf[1] == GET_SERIALNR) ||
			    (buf[1] == SET_SERIALNR)) {
				printf("SERIAL: '");
				for (unsigned i = 0; i < (n-4); i++) {
					char c = buf[i+2];
					printf("%c ", c);
				}
				printf("'.\n");
			}
			if ((buf[1] == GET_SERVICEDATE) ||
			    (buf[1] == SET_SERVICEDATE)) {
				if ((n - 4) != 3)
					return 0;
				printf("SERVICEDATE: '");
				// DDMMYY
				printf("%02d/%02d/20%02d",
					buf[2], buf[3], buf[4]);
				printf("'.\n");
			}
		} else if (buf[0] == POWER) {
			printf("subtype: '%s' with word len=%d\n",
					pp_power_cmd_t(buf[1]), (n-4));
			if ((n - 4) == 0)
				return 0;
			switch (buf[1]) {
				case GET_SUPPLY_STATE:
					printf("?GetPowerSupplyStates()? Supply State: '");
					break;
				case EN_DIS_SUPPLY:
					printf("?SetPowerSupplyStates()? Display Supply: '");
					break;
				default:
					printf("\nwtf<power>'");
			}
			for (unsigned i = 0; i < (n-4); i++) {
				int v = buf[i+2];
				printf("0x%02X ", v);
			}
			printf("'.\n");
		} else if (buf[0] == MEASURE) {
			printf("subtype: '%s' with word len=%d\n",
					pp_measure_cmd_t(buf[1]), (n-4));
			if ((n - 4) == 0) {
				//printf("(W req I guess)\n");
				return 0;
			}
			switch (buf[1]) {
				case GET_LOGIC_VOLTAGE:
					printf("Logic Voltage: '");
					break;
				case GET_PERIPHERAL_VOLTAGE:
					printf("Peripheral Voltage: '");
					break;
				case GET_RIGHT_HAND_VOLTAGE:
					printf("Right Hand Voltage: '");
					break;
				case GET_LEFT_HAND_VOLTAGE:
					printf("Left Hand Voltage: '");
					break;
				case GET_RIGHT_FOOT_VOLTAGE:
					printf("Right Foot Voltage: '");
					break;
				case GET_LEFT_FOOT_VOLTAGE:
					printf("Left Foot Voltage: '");
					break;
				case GET_SQUIB_VOLTAGE:
					printf("Squib Voltage: '");
					break;
				case GET_BATTERY_VOLTAGE:
					printf("Battery Voltage: '");
					break;
				case GET_CHARGING_STATE:
					printf("GET_CHARGING_STATE: '");
					break;
				case GET_ZIP_SWITCH_STATE:
					printf("GET_ZIP_SWITCH_STATE: '");
					if ((n-4) == 1) {
						if (buf[2] == 0)
							printf("OPEN!\n");
						else if (buf[2] == 1)
							printf("CLOSED!\n");
						else
							printf("UNKNOWN!?\n");
						return 0;
					}
					break;
				case SET_EXT_DISPLAY:
					printf("SET_EXT_DISPLAY: SetUSBLEDs()'");
					break;
				default:
					printf("wtf<supplies>?'");
			}
			//for (unsigned i = 0; i < (n-4); i++) {
			//	int v = buf[i+2];
			//	printf("0x%02X ", v);
			//}
			printf("%.3f V", fixed16_to_double(((buf[2] << 8) | buf[3])));
			printf("'.\n");
		} else if (buf[0] == SENSOR) {
			printf(" buf[1] == 0x%02x\n", buf[1]);
			//printf("subtype: '%s' with word len=%d\n",
			//		pp_XXX_cmd_t(buf[1]), (n-4));
			if ((n - 4) == 0) {
				//printf("(W req I guess)\n");
				return 0;
			}
			if (buf[1] == ENABLE_SENSOR_READING /*0x0*/) {
				printf("ENABLE_SENSOR_READINGS (mask)= 0x%02X ", buf[2]);
			} else if (buf[1] == GET_SENSOR_READING_ENABLES /*0x1*/) {
				printf("GET_SENSOR_READINGS_ENABLES = 0x%02X ", buf[2]);
			} else if ((buf[1] == GET_RIGHT_HAND_ACCEL /*0x2*/ ) ||
				   (buf[1] == GET_LEFT_HAND_ACCEL  /*0x3*/ ) ||
				   (buf[1] == GET_RIGHT_FOOT_ACCEL /*0x4*/ ) ||
			    	   (buf[1] == GET_LEFT_FOOT_ACCEL  /*0x5*/ ) ||
			    	   (buf[1] == GET_BODY_ACCEL       /*0x6*/ ) ||
			    	   (buf[1] == GET_GYROSCOPE        /*0x7*/ )) {
				switch (buf[1]) {
					case GET_RIGHT_HAND_ACCEL:
						printf("Right hand accelerometer ");
						break;
					case GET_LEFT_HAND_ACCEL:
						printf("Left hand accelerometer ");
						break;
					case GET_BODY_ACCEL:
						printf("Body accelerometer ");
						break;
					case GET_GYROSCOPE:
						printf("Gyroscope ");
						break;
					default:
						printf("Right or Left foot accelerometer? ");
				}

				uint16_t x = ((buf[2] << 8) | buf[3]);
				uint16_t y = ((buf[4] << 8) | buf[5]);
				uint16_t z = ((buf[6] << 8) | buf[7]);
				if (buf[1] != GET_GYROSCOPE) {
					printf("(X=%.3f, Y=%.3f, Z=%.3f)",
							//((x)/32768.0 * (9984.0 / 625.0)),
							//((y)/32768.0 * (9984.0 / 625.0)),
							//((z)/32768.0 * (9984.0 / 625.0))
							(fixed16_to_double_2(x)/*/32768.0*/ * (9984.0 / 625.0)),
							(fixed16_to_double_2(y)/*/32768.0*/ * (9984.0 / 625.0)),
							(fixed16_to_double_2(z)/*/32768.0*/ * (9984.0 / 625.0))
	      				);
				} else {
					printf("(X=%.3f, Y=%.3f, Z=%.3f)",
							(fixed16_to_double_2(x)/*/32768.0*/ * (2279.513043)),
							(fixed16_to_double_2(y)/*/32768.0*/ * (2279.513043)),
							(fixed16_to_double_2(z)/*/32768.0*/ * (2279.513043))
	      				);
				}
				printf("  :  ");
				for (unsigned i = 0; i < (n-4); i++) {
					int v = buf[i+2];
					printf("0x%02X ", v);
				}
//  GET_SWV_RH = 0x8,
//  GET_SWV_LH = 0x9,
//  GET_SWV_RF = 0xa,
//  GET_SWV_LF = 0xb,
			} else if (buf[1] == 0x08) {
				uint16_t a = ((buf[2] << 8) | buf[3]);
				uint16_t b = ((buf[4] << 8) | buf[5]);
				printf("(RH) - (sw,hw) rev=(%.3f, %.3f)",
						10*fixed16_to_double(a),
						10*fixed16_to_double(b)
	      			);
			} else if (buf[1] == 0x09) {
				uint16_t a = ((buf[2] << 8) | buf[3]);
				uint16_t b = ((buf[4] << 8) | buf[5]);
				printf("(LH) - (sw,hw) rev=(%.3f, %.3f)",
						10*fixed16_to_double(a),
						10*fixed16_to_double(b)
	      			);
			} else if (buf[1] == 0x0a) {
				uint16_t a = ((buf[2] << 8) | buf[3]);
				uint16_t b = ((buf[4] << 8) | buf[5]);
				printf("(RF) - (sw,hw) rev=(%.3f, %.3f)",
						10*fixed16_to_double(a),
						10*fixed16_to_double(b)
	      			);
			} else if (buf[1] == 0x0b) {
				uint16_t a = ((buf[2] << 8) | buf[3]);
				uint16_t b = ((buf[4] << 8) | buf[5]);
				printf("(LF) - (sw,hw) rev=(%.3f, %.3f)",
						10*fixed16_to_double(a),
						10*fixed16_to_double(b)
	      			);
			} else {
				printf("idk what this [sensor] subtype (0x%02x) is? ", buf[1]);
				for (unsigned i = 0; i < (n-4); i++) {
					int v = buf[i+2];
					printf("0x%02X ", v);
				}
			}
			printf("\n");
		} else if (buf[0] == LOGGING) {
			printf(" buf[1] == 0x%02x\n", buf[1]);
			//printf("subtype: '%s' with word len=%d\n",
			//		pp_XXX_cmd_t(buf[1]), (n-4));
			if ((n - 4) >= 0) {
				if (buf[1] == GET_OP_HOURS /* 0x0*/)
					printf("GET_OP_HOURS ");
				else if (buf[1] == 0x01)
					printf("CLEAR_OP_HOURS ");
				else if (buf[1] == 0x03)
					printf("GET_ERROR_ENTRY ");
				else if (buf[1] == 0x0B)
					printf("GET_PRECRASH_BULK ");
				else if (buf[1] == GET_ERROR_HISTORY /*0xc*/)
					printf("GET_ERROR_HISTORY ");
				else
					printf(" DEBUG got this? 0x%02x ", buf[1]);
			}
			for (unsigned i = 0; i < (n-4); i++) {
				int v = buf[i+2];
				printf("0x%02X ", v);
			}
			printf("\n");
		} else if (buf[0] == AIRBAG) {
			printf(" buf[1] == 0x%02x\n", buf[1]);
			//printf("subtype: '%s' with word len=%d\n",
			//		pp_XXX_cmd_t(buf[1]), (n-4));
			if ((n-4) == 0) {
				//printf("(W req I guess)\n");
				return 0;
			}
			if ((buf[1] == GET_INFLATIONTYPE /*0xa*/)) {
//	SET_INFLATIONTYPE = 0xb,
				printf("Inflation Type: '");
				// if version == 0xB4
				// {
				//    if (buf[2] == 0x44) 
				//       "single"
				//    else
				//       "double"
				switch (buf[2]) {
					case 0x44: printf("Single");
						   break;
					default:
						   // DEBUG?
						for (unsigned i = 0; i < (n-4); i++) {
							uint8_t v = buf[i+2];
							printf("0x%02x ", v);
						}
				}
				// } else if version == 0xBB {
				//     ?
			} else if ((buf[1] == RESET_IGNITION_CTRL /*0x6*/)) {
				printf("reset ignition? ");
				for (unsigned i = 0; i < (n-4); i++) {
					uint8_t v = buf[i+2];
					printf("0x%02x ", v);
				}
			} else {
				printf("airbag? ");
				for (unsigned i = 0; i < (n-4); i++) {
					int v = buf[i+2];
					printf("0x%02X ", v);
				}
			}
			printf("'.\n");
		} else {
			printf("\n ---- unknown -----\n");
			printf("\n ---- dumping -----\n");
			for (unsigned i = 0; i < (n-4); i++) {
				int v = buf[i+2];
				printf("0x%02X ", v);
			}
			printf("\n ---- dumping -----\n");
		}
	}

	return 0;
}

int main(int argc, const char ** argv)
{
	FILE * stream;
	char * line;
	size_t len = 0;
	ssize_t nread;

	if (argc < 2) {
		printf("Usage: %s <file>\n", argv[0]);
		exit(EXIT_FAILURE);
	}

	stream = fopen(argv[1], "r");
	if (!stream) {
		perror("fopen()\n");
		exit(EXIT_FAILURE);
	}

	while ((nread = getline(&line, &len, stream)) != -1) {
		// skip lines that don't begin with 'R|W'..
		if ((line[0] != 'R') && (line[0] != 'W'))
			continue;

		if (line[0] == 'R') {
			printf("R: ");
		}
		if (line[0] == 'W') {
			printf("W: ");
		}
		line+=3;

		unsigned sz = parse_packet(line, NULL);
		uint8_t * buf = calloc(1, sz);
		parse_packet(line, buf);
		decode_packet(buf, sz);
		free(buf);
		line-=3;
	}

	free(line);
	fclose(stream);

	return 0;
}
