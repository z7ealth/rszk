#![allow(dead_code)]

const USHRT_MAX: i32 = 65535;

const CMD_DB_RRQ: i8 = 7; // Read in some kind of data from the machine
const CMD_USER_WRQ: i8 = 8; // Upload the user information from PC to terminal.
const CMD_USERTEMP_RRQ: i8 = 9; // Read some fingerprint template or some kind of data entirely
const CMD_USERTEMP_WRQ: i8 = 10; // Upload some fingerprint template
const CMD_OPTIONS_RRQ: i8 = 11; // Read in the machine some configuration parameter
const CMD_OPTIONS_WRQ: i8 = 12; // Set machines configuration parameter
const CMD_ATTLOG_RRQ: i8 = 13; // Read all attendance record
const CMD_CLEAR_DATA: i8 = 14; // clear Data
const CMD_CLEAR_ATTLOG: i8 = 15; // Clear attendance records
const CMD_DELETE_USER: i8 = 18; // Delete some user
const CMD_DELETE_USERTEMP: i8 = 19; // Delete some fingerprint template
const CMD_CLEAR_ADMIN: i8 = 20; // Cancel the manager
const CMD_USERGRP_RRQ: i8 = 21; // Read the user grouping
const CMD_USERGRP_WRQ: i8 = 22; // Set users grouping
const CMD_USERTZ_RRQ: i8 = 23; // Read the user Time Zone set
const CMD_USERTZ_WRQ: i8 = 24; // Write the user Time Zone set
const CMD_GRPTZ_RRQ: i8 = 25; // Read the group Time Zone set
const CMD_GRPTZ_WRQ: i8 = 26; // Write the group Time Zone set
const CMD_TZ_RRQ: i8 = 27; // Read Time Zone set
const CMD_TZ_WRQ: i8 = 28; // Write the Time Zone
const CMD_ULG_RRQ: i8 = 29; // Read unlocks combination
const CMD_ULG_WRQ: i8 = 30; // write unlocks combination
const CMD_UNLOCK: i8 = 31; // unlock
const CMD_CLEAR_ACC: i8 = 32; // Restores Access Control set to the default condition.
const CMD_CLEAR_OPLOG: i8 = 33; // Delete attendance machines all attendance record.
const CMD_OPLOG_RRQ: i8 = 34; // Read manages the record
const CMD_GET_FREE_SIZES: i8 = 50; // Obtain machines condition, like user recording number and so on
const CMD_ENABLE_CLOCK: i8 = 57; // Ensure the machine to be at the normal work condition
const CMD_STARTVERIFY: i8 = 60; // Ensure the machine to be at the authentication condition
const CMD_STARTENROLL: i8 = 61; // Start to enroll some user, ensure the machine to be at the registration user condition
const CMD_CANCELCAPTURE: i8 = 62; // Make the machine to be at the waiting order status, please refers to the CMD_STARTENROLL description.
const CMD_STATE_RRQ: i8 = 64; // Gain the machine the condition
const CMD_WRITE_LCD: i8 = 66; // Write LCD
const CMD_CLEAR_LCD: i8 = 67; // Clear the LCD captions clear screen.
const CMD_GET_PINWIDTH: i8 = 69; // Obtain the length of user’s serial number
const CMD_SMS_WRQ: i8 = 70; // Upload the short message.
const CMD_SMS_RRQ: i8 = 71; // Download the short message
const CMD_DELETE_SMS: i8 = 72; // Delete the short message
const CMD_UDATA_WRQ: i8 = 73; // Set user’s short message
const CMD_DELETE_UDATA: i8 = 74; // Delete user’s short message
const CMD_DOORSTATE_RRQ: i8 = 75; // Obtain the door condition
const CMD_WRITE_MIFARE: i8 = 76; // Write the Mifare card
const CMD_EMPTY_MIFARE: i8 = 78; // Clear the Mifare card
const _CMD_GET_USERTEMP: i8 = 88; // UNDOCUMENTED! get an specific user template uid, fid
const _CMD_SAVE_USERTEMPS: i8 = 110; // UNDOCUMENTED! save user and multiple templates!
const _CMD_DEL_USER_TEMP: i16 = 134; // UNDOCUMENTED! delete an specific user template uid, fid16
const CMD_GET_TIME: i16 = 201; // Obtain the machine time
const CMD_SET_TIME: i16 = 202; // Set machines time
const CMD_REG_EVENT: i16 = 500; // Register the event

const CMD_CONNECT: i16 = 1000; // Connections requests
const CMD_EXIT: i16 = 1001; // Disconnection requests
const CMD_ENABLEDEVICE: i16 = 1002; // Ensure the machine to be at the normal work condition
const CMD_DISABLEDEVICE: i16 = 1003; // Make the machine to be at the shut-down condition, generally demonstrates ‘in the work ...’on LCD
const CMD_RESTART: i16 = 1004; // Restart the machine.
const CMD_POWEROFF: i16 = 1005; // Shut-down power source
const CMD_SLEEP: i16 = 1006; // Ensure the machine to be at the idle state.
const CMD_RESUME: i16 = 1007; // Awakens the sleep machine temporarily not to support
const CMD_CAPTUREFINGER: i16 = 1009; // Captures fingerprints picture
const CMD_TEST_TEMP: i16 = 1011; // Test some fingerprint exists or does not
const CMD_CAPTUREIMAGE: i16 = 1012; // Capture the entire image
const CMD_REFRESHDATA: i16 = 1013; // Refresh the machine interior data
const CMD_REFRESHOPTION: i16 = 1014; // Refresh the configuration parameter
const CMD_TESTVOICE: i16 = 1017; // Play voice
const CMD_GET_VERSION: i16 = 1100; // Obtain the firmware edition
const CMD_CHANGE_SPEED: i16 = 1101; // Change transmission speed
const CMD_AUTH: i16 = 1102; // Connections authorizations
const CMD_PREPARE_DATA: i16 = 1500; // Prepares to transmit the data
const CMD_DATA: i16 = 1501; // Transmit a data packet
const CMD_FREE_DATA: i16 = 1502; // Clear machines opened buffer
const _CMD_PREPARE_BUFFER: i16 = 1503; // UNDOCUMENTED initialize buffer for partial reads!
const _CMD_READ_BUFFER: i16 = 1504; // UNDOCUMENTED ready a partial chunk of data from buffer

const CMD_ACK_OK: i16 = 2000; // Return value for order perform successfully
const CMD_ACK_ERROR: i16 = 2001; // Return value for order perform failed
const CMD_ACK_DATA: i16 = 2002; // Return data
const CMD_ACK_RETRY: i16 = 2003; // * Regstered event occorred */
const CMD_ACK_REPEAT: i16 = 2004; // Not available
const CMD_ACK_UNAUTH: i16 = 2005; // Connection unauthorized

const CMD_ACK_UNKNOWN: u16 = 0xffff; // Unkown order
const CMD_ACK_ERROR_CMD: u16 = 0xfffd; // Order false
const CMD_ACK_ERROR_INIT: u16 = 0xfffc;
///* Not Initializated */
const CMD_ACK_ERROR_DATA: u16 = 0xfffb; // Not available

const EF_ATTLOG: i8 = 1; // Be real-time to verify successfully
const EF_FINGER: i8 = 1 << 1; // be real–time to press fingerprint be real time to return data type sign
const EF_ENROLLUSER: i8 = 1 << 2; // Be real-time to enroll user
const EF_ENROLLFINGER: i8 = 1 << 3; // be real-time to enroll fingerprint
const EF_BUTTON: i8 = 1 << 4; // be real-time to press button
const EF_UNLOCK: i8 = 1 << 5; // be real-time to unlock
const EF_VERIFY: i8 = 1 << 7; // be real-time to verify fingerprint
const EF_FPFTR: i16 = 1 << 8; // be real-time capture fingerprint minutia
const EF_ALARM: i16 = 1 << 9; // Alarm signal

const USER_DEFAULT: i8 = 0;
const USER_ENROLLER: i8 = 2;
const USER_MANAGER: i8 = 6;
const USER_ADMIN: i8 = 14;

const FCT_ATTLOG: i8 = 1;
const FCT_WORKCODE: i8 = 8;
const FCT_FINGERTMP: i8 = 2;
const FCT_OPLOG: i8 = 4;
const FCT_USER: i8 = 5;
const FCT_SMS: i8 = 6;
const FCT_UDATA: i8 = 7;

const MACHINE_PREPARE_DATA_1: i16 = 20560; // 0x5050
const MACHINE_PREPARE_DATA_2: i16 = 32130; // 0x7282
