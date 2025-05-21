use byteorder::{LittleEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::io::Write;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub uid: u16,
    pub name: String,
    pub privilege: u16,
    pub password: String,
    pub group_id: String,
    pub user_id: u32,
    pub card: u64, // original card is 64-bit int but only 40 bits used in comment
}

impl User {
    pub fn new(
        uid: u16,
        name: String,
        privilege: u16,
        password: String,
        group_id: String,
        user_id: u32,
        card: u64,
    ) -> Self {
        Self {
            uid,
            name,
            privilege,
            password,
            group_id,
            user_id,
            card,
        }
    }

    pub fn json_unpack(json: &Value) -> Option<Self> {
        Some(Self {
            uid: json.get("uid")?.as_u64()? as u16,
            name: json.get("name")?.as_str()?.to_string(),
            privilege: json.get("privilege")?.as_u64()? as u16,
            password: json.get("password")?.as_str()?.to_string(),
            group_id: json.get("group_id")?.as_str()?.to_string(),
            user_id: json.get("user_id")?.as_u64()? as u32,
            card: json.get("card")?.as_u64()?,
        })
    }

    /// Pack as per repack29 (size 29 for zk6)
    pub fn repack29(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_u8(2).unwrap();
        buf.write_u16::<LittleEndian>(self.uid).unwrap();
        buf.write_u16::<LittleEndian>(self.privilege).unwrap();

        let mut pw_bytes = [0u8; 5];
        let pw_encoded = self.password.as_bytes();
        let len_pw = pw_encoded.len().min(5);
        pw_bytes[..len_pw].copy_from_slice(&pw_encoded[..len_pw]);
        buf.extend_from_slice(&pw_bytes);

        let mut name_bytes = [0u8; 8];
        let name_encoded = self.name.as_bytes();
        let len_name = name_encoded.len().min(8);
        name_bytes[..len_name].copy_from_slice(&name_encoded[..len_name]);
        buf.extend_from_slice(&name_bytes);

        buf.write_u64::<LittleEndian>(self.card).unwrap();

        let group_id_num = self.group_id.parse::<u32>().unwrap_or(0);
        buf.write_u32::<LittleEndian>(group_id_num).unwrap();

        buf.write_u8(0).unwrap(); // unknown zero

        buf.write_u32::<LittleEndian>(self.user_id).unwrap();

        buf
    }

    /// Pack as per repack73 (size 73 for zk8)
    pub fn repack73(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_u8(2).unwrap();
        buf.write_u16::<LittleEndian>(self.uid).unwrap();
        buf.write_u16::<LittleEndian>(self.privilege).unwrap();

        let mut pw_bytes = [0u8; 8];
        let pw_encoded = self.password.as_bytes();
        let len_pw = pw_encoded.len().min(6);
        pw_bytes[..len_pw].copy_from_slice(&pw_encoded[..len_pw]);
        pw_bytes[len_pw] = 0;
        pw_bytes[len_pw + 1] = 0x77;
        buf.extend_from_slice(&pw_bytes);

        let mut name_bytes = [0u8; 24];
        let name_encoded = self.name.as_bytes();
        let len_name = name_encoded.len().min(24);
        name_bytes[..len_name].copy_from_slice(&name_encoded[..len_name]);
        buf.extend_from_slice(&name_bytes);

        buf.write_u64::<LittleEndian>(self.card).unwrap();

        buf.write_u8(1).unwrap(); // unknown 1

        let mut group_id_bytes = [0u8; 7];
        let gid_bytes = self.group_id.as_bytes();
        let len_gid = gid_bytes.len().min(7);
        group_id_bytes[..len_gid].copy_from_slice(&gid_bytes[..len_gid]);
        buf.extend_from_slice(&group_id_bytes);

        buf.write_u8(0).unwrap(); // unknown zero byte

        let mut user_id_bytes = [0u8; 24];
        let binding = self.user_id.to_string();
        let uid_bytes = binding.as_bytes();
        let len_uid = uid_bytes.len().min(24);
        user_id_bytes[..len_uid].copy_from_slice(&uid_bytes[..len_uid]);
        buf.extend_from_slice(&user_id_bytes);

        buf
    }

    pub fn is_disabled(&self) -> bool {
        (self.privilege & 1) != 0
    }

    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }

    pub fn usertype(&self) -> u16 {
        self.privilege & 0xE
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<User>: [uid:{}, name:{} user_id:{}]",
            self.uid, self.name, self.user_id
        )
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Reuse Display implementation
        fmt::Display::fmt(self, f)
    }
}
