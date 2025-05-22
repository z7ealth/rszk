// def get_attendance(self):
//         """
//         return attendance record

//         :return: List of Attendance object
//         """
//         self.read_sizes()
//         if self.records == 0:
//             return []
//         users = self.get_users()
//         if self.verbose: print (users)
//         attendances = []
//         attendance_data, size = self.read_with_buffer(const.CMD_ATTLOG_RRQ)
//         if size < 4:
//             if self.verbose: print ("WRN: no attendance data")
//             return []
//         total_size = unpack("I", attendance_data[:4])[0]
//         record_size = total_size // self.records
//         if self.verbose: print ("record_size is ", record_size)
//         attendance_data = attendance_data[4:]
//         if record_size == 8:
//             while len(attendance_data) >= 8:
//                 uid, status, timestamp, punch = unpack('HB4sB', attendance_data.ljust(8, b'\x00')[:8])
//                 if self.verbose: print (codecs.encode(attendance_data[:8], 'hex'))
//                 attendance_data = attendance_data[8:]
//                 tuser = list(filter(lambda x: x.uid == uid, users))
//                 if not tuser:
//                     user_id = str(uid)
//                 else:
//                     user_id = tuser[0].user_id
//                 timestamp = self.__decode_time(timestamp)
//                 attendance = Attendance(user_id, timestamp, status, punch, uid)
//                 attendances.append(attendance)
//         elif record_size == 16:
//             while len(attendance_data) >= 16:
//                 user_id, timestamp, status, punch, reserved, workcode = unpack('<I4sBB2sI', attendance_data.ljust(16, b'\x00')[:16])
//                 user_id = str(user_id)
//                 if self.verbose: print(codecs.encode(attendance_data[:16], 'hex'))
//                 attendance_data = attendance_data[16:]
//                 tuser = list(filter(lambda x: x.user_id == user_id, users))
//                 if not tuser:
//                     if self.verbose: print("no uid {}", user_id)
//                     uid = str(user_id)
//                     tuser = list(filter(lambda x: x.uid == user_id, users))
//                     if not tuser:
//                         uid = str(user_id)
//                     else:
//                         uid = tuser[0].uid
//                         user_id = tuser[0].user_id
//                 else:
//                     uid = tuser[0].uid
//                 timestamp = self.__decode_time(timestamp)
//                 attendance = Attendance(user_id, timestamp, status, punch, uid)
//                 attendances.append(attendance)
//         else:
//             while len(attendance_data) >= 40:
//                 uid, user_id, status, timestamp, punch, space = unpack('<H24sB4sB8s', attendance_data.ljust(40, b'\x00')[:40])
//                 if self.verbose: print (codecs.encode(attendance_data[:40], 'hex'))
//                 user_id = (user_id.split(b'\x00')[0]).decode(errors='ignore')
//                 timestamp = self.__decode_time(timestamp)

//                 attendance = Attendance(user_id, timestamp, status, punch, uid)
//                 attendances.append(attendance)
//                 attendance_data = attendance_data[record_size:]
//         return attendances

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use byteorder::{ByteOrder, LittleEndian};
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};

use crate::{attandance::Attendance, consts, exception::ZKError, user::User};

#[derive(Debug)]
pub struct ZK {
    pub address: SocketAddr,
    pub socket: ZkSocket,
    pub timeout: Duration,
    pub session_id: u16,
    pub reply_id: u16,
    pub is_connect: bool,
    pub user_packet_size: usize,
    pub users: usize,
    pub records: usize,
    pub verbose: bool,
    pub data: Vec<u8>,
    pub response: u16,
    pub force_udp: bool,
}

#[derive(Debug)]
pub enum ZkSocket {
    Udp(UdpSocket),
    Tcp(TcpStream),
}

impl ZkSocket {
    pub fn send_to(&mut self, buf: &[u8], target: SocketAddr) -> std::io::Result<usize> {
        match self {
            ZkSocket::Udp(sock) => sock.send_to(buf, target),
            ZkSocket::Tcp(sock) => sock.write(buf),
        }
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ZkSocket::Udp(sock) => sock.recv(buf),
            ZkSocket::Tcp(sock) => sock.read(buf),
        }
    }
}

struct CommandResponse {
    status: bool,
}

fn decode_time(raw: &[u8]) -> DateTime<Local> {
    let mut t = LittleEndian::read_u32(raw);
    let second = t % 60;
    t /= 60;
    let minute = t % 60;
    t /= 60;
    let hour = t % 24;
    t /= 24;
    let day = (t % 31) + 1;
    t /= 31;
    let month = (t % 12) + 1;
    t /= 12;
    let year = t + 2000;

    let naive = NaiveDateTime::parse_from_str(
        &format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        ),
        "%Y-%m-%d %H:%M:%S",
    )
    .unwrap();
    DateTime::<Local>::from_naive_utc_and_offset(naive, FixedOffset::east_opt(0).unwrap())
}

fn create_checksum(buf: &[u8]) -> u16 {
    let mut checksum = 0u32;
    let mut i = 0;

    while i + 1 < buf.len() {
        let val = LittleEndian::read_u16(&buf[i..i + 2]);
        checksum += val as u32;
        if checksum > 0xFFFF {
            checksum -= 0xFFFF;
        }
        i += 2;
    }

    if i < buf.len() {
        checksum += buf[i] as u32;
    }

    while checksum > 0xFFFF {
        checksum -= 0xFFFF;
    }

    (!checksum as u16) & 0xFFFF
}

impl ZK {
    pub fn new<A: ToSocketAddrs>(
        addr: A,
        timeout_secs: u64,
        verbose: bool,
        force_udp: bool,
    ) -> Result<Self, ZKError> {
        let timeout = Duration::from_secs(timeout_secs);
        let address = addr
            .to_socket_addrs()
            .unwrap()
            .next()
            .ok_or_else(|| ZKError::ResponseError)?;

        let socket = if force_udp {
            let udp = UdpSocket::bind("0.0.0.0:0").map_err(|_e| ZKError::NetworkError)?;
            udp.set_read_timeout(Some(timeout))
                .map_err(|_e| ZKError::NetworkError)?;
            udp.set_write_timeout(Some(timeout))
                .map_err(|_e| ZKError::NetworkError)?;
            ZkSocket::Udp(udp)
        } else {
            let tcp = TcpStream::connect(address).map_err(|_e| ZKError::NetworkError)?;
            tcp.set_read_timeout(Some(timeout))
                .map_err(|_e| ZKError::NetworkError)?;
            tcp.set_write_timeout(Some(timeout))
                .map_err(|_e| ZKError::NetworkError)?;
            ZkSocket::Tcp(tcp)
        };

        Ok(Self {
            address,
            socket,
            timeout,
            session_id: 0,
            reply_id: 0xffff - 1,
            is_connect: false,
            user_packet_size: 28,
            users: 0,
            records: 0,
            verbose,
            data: Vec::new(),
            response: 0,
            force_udp,
        })
    }

    pub fn get_attendance(&mut self) -> Result<Vec<Attendance>, ZKError> {
        self.read_sizes()?;
        if self.records == 0 {
            return Ok(vec![]);
        }

        let users = self.get_users()?;
        let (mut attendance_data, size) = self.read_with_buffer(consts::CMD_ATTLOG_RRQ, 0, 0)?;

        if size < 4 {
            return Ok(vec![]);
        }

        let total_size = LittleEndian::read_u32(&attendance_data[..4]) as usize;
        let record_size = total_size / self.records;
        attendance_data = attendance_data[4..].to_vec();

        let mut attendances = Vec::new();

        let mut i = 0;
        while i + record_size <= attendance_data.len() {
            let chunk = &attendance_data[i..i + record_size];
            if record_size == 8 {
                let uid = LittleEndian::read_u16(&chunk[0..2]);
                let status = chunk[2];
                let timestamp = decode_time(&chunk[3..7]);
                let punch: i32 = chunk[7].into();

                let user_id = users.iter().find(|u| u.uid == uid).unwrap().user_id;

                attendances.push(Attendance::new(
                    user_id,
                    timestamp.to_string(),
                    status.to_string(),
                    punch,
                    uid.into(),
                ));
            } else {
                // Support other record sizes later (16, 40, etc)
                return Err(ZKError::UnsupportedRecordSize);
            }
            i += record_size;
        }

        Ok(attendances)
    }

    fn create_header(&mut self, command: u16, command_string: &[u8]) -> Result<Vec<u8>, ZKError> {
        let mut buf = Vec::with_capacity(8 + command_string.len());
        buf.extend(&command.to_le_bytes());
        buf.extend(&0u16.to_le_bytes()); // temporary checksum placeholder
        buf.extend(&self.session_id.to_le_bytes());
        buf.extend(&self.reply_id.to_le_bytes());
        buf.extend(command_string);

        let checksum = create_checksum(&buf);
        self.reply_id = self.reply_id.wrapping_add(1);
        if self.reply_id >= 0xFFFF {
            self.reply_id = 0;
        }

        // Now patch the checksum
        buf[2..4].copy_from_slice(&checksum.to_le_bytes());

        Ok(buf)
    }

    fn send_command(
        &mut self,
        command: u16,
        payload: &[u8],
        response_size: usize,
    ) -> Result<CommandResponse, ZKError> {
        // TODO: implement create_header like Python version
        let buf = self.create_header(command, payload)?;

        self.socket.send_to(&buf, self.address).unwrap();
        let mut recv_buf = vec![0u8; response_size];
        let len = self.socket.recv(&mut recv_buf).unwrap();

        self.response = LittleEndian::read_u16(&recv_buf[0..2]);
        self.data = recv_buf[8..len].to_vec(); // skip header
        Ok(CommandResponse {
            status: matches!(
                self.response,
                consts::CMD_ACK_OK | consts::CMD_DATA | consts::CMD_PREPARE_DATA
            ),
        })
    }

    pub fn read_sizes(&mut self) -> Result<(), ZKError> {
        let response = self.send_command(consts::CMD_GET_FREE_SIZES, &[], 1024)?;
        if !response.status {
            return Err(ZKError::ResponseError);
        }

        let data = &self.data;
        if data.len() >= 80 {
            let mut fields = [0i32; 20];
            for i in 0..20 {
                fields[i] = LittleEndian::read_i32(&data[i * 4..(i + 1) * 4]);
            }

            self.users = fields[4] as usize;
            self.records = fields[8] as usize;
        }

        Ok(())
    }

    pub fn get_users(&mut self) -> Result<Vec<User>, ZKError> {
        self.read_sizes()?;
        if self.users == 0 {
            return Ok(vec![]);
        }

        let (mut data, size) =
            self.read_with_buffer(consts::CMD_USERTEMP_RRQ, consts::FCT_USER, 0)?;
        if size <= 4 {
            return Ok(vec![]);
        }

        let total_size = LittleEndian::read_u32(&data[..4]) as usize;
        self.user_packet_size = total_size / self.users;

        let mut users = Vec::new();
        data = data[4..].to_vec();
        let mut i = 0;

        while i + self.user_packet_size <= data.len() {
            let chunk = &data[i..i + self.user_packet_size];

            if self.user_packet_size == 28 {
                let uid = LittleEndian::read_u16(&chunk[0..2]);
                let privilege: u16 = chunk[2].into();
                let password = std::str::from_utf8(&chunk[3..8])
                    .unwrap_or("")
                    .trim_end_matches('\0')
                    .to_string();
                let name = std::str::from_utf8(&chunk[8..16])
                    .unwrap_or("")
                    .trim_end_matches('\0')
                    .to_string();
                let card: u64 = LittleEndian::read_u32(&chunk[16..20]).into();
                let group_id = chunk[20].to_string();
                let user_id = LittleEndian::read_u32(&chunk[24..28]);

                users.push(User::new(
                    uid, name, privilege, password, group_id, user_id, card,
                ));
            }

            // Optionally handle 72-byte format here

            i += self.user_packet_size;
        }

        Ok(users)
    }

    pub fn free_data(&mut self) -> Result<(), ZKError> {
        let response = self.send_command(consts::CMD_FREE_DATA, &[], 8)?;
        if response.status {
            Ok(())
        } else {
            Err(ZKError::ResponseError)
        }
    }

    fn receive_chunk(&mut self) -> Result<Vec<u8>, ZKError> {
        match self.response {
            consts::CMD_DATA => {
                // Already have the data in self.data
                Ok(self.data.clone())
            }
            consts::CMD_PREPARE_DATA => {
                let size = self.get_data_size()?; // Read 4 bytes as u32
                let mut data = Vec::with_capacity(size);
                let mut remaining = size;

                while remaining > 0 {
                    let mut buffer = vec![0u8; 1032];
                    let len = self.socket.recv(&mut buffer).unwrap();
                    let header = LittleEndian::read_u16(&buffer[..2]);
                    if header != consts::CMD_DATA {
                        break;
                    }
                    data.extend_from_slice(&buffer[8..len]);
                    remaining -= len - 8;
                }

                // Read ACK_OK to complete transfer
                let mut ack_buf = [0u8; 16];
                let _ = self.socket.recv(&mut ack_buf).unwrap();

                Ok(data)
            }
            _ => Err(ZKError::ResponseError),
        }
    }

    fn get_data_size(&self) -> Result<usize, ZKError> {
        if self.data.len() >= 4 {
            Ok(LittleEndian::read_u32(&self.data[..4]) as usize)
        } else {
            Err(ZKError::ResponseError)
        }
    }

    pub fn read_with_buffer(
        &mut self,
        command: u16,
        fct: u32,
        ext: u32,
    ) -> Result<(Vec<u8>, usize), ZKError> {
        const MAX_CHUNK: usize = 16 * 1024;

        let mut buf = Vec::new();
        let mut command_string = Vec::with_capacity(11);
        command_string.push(1); // 1 byte
        command_string.extend(&command.to_le_bytes());
        command_string.extend(&fct.to_le_bytes());
        command_string.extend(&ext.to_le_bytes());

        let response = self
            .send_command(consts::CMD_PREPARE_BUFFER, &command_string, 1024)
            .unwrap();

        if !response.status {
            return Err(ZKError::ResponseError);
        }

        if self.response == consts::CMD_DATA {
            return Ok((self.data.clone(), self.data.len()));
        }

        let total_size = LittleEndian::read_u32(&self.data[1..5]) as usize;
        let remain = total_size % MAX_CHUNK;
        let packets = total_size / MAX_CHUNK;
        let mut start = 0;

        for _ in 0..packets {
            let chunk = self.read_chunk(start, MAX_CHUNK)?;
            buf.extend(chunk);
            start += MAX_CHUNK;
        }

        if remain > 0 {
            let chunk = self.read_chunk(start, remain)?;
            buf.extend(chunk);
        }

        self.free_data()?;
        Ok((buf, start + remain))
    }

    fn read_chunk(&mut self, start: usize, size: usize) -> Result<Vec<u8>, ZKError> {
        let mut command_string = Vec::new();
        command_string.extend(&(start as i32).to_le_bytes());
        command_string.extend(&(size as i32).to_le_bytes());

        self.send_command(consts::CMD_READ_BUFFER, &command_string, size + 8)?;
        self.receive_chunk()
    }
}
