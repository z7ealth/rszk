use byteorder::{LittleEndian, WriteBytesExt};
use hex::{decode as hex_decode, encode as hex_encode};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Finger {
    pub uid: u16,
    pub fid: u8,
    pub valid: u8,
    pub template: Vec<u8>,
    pub size: usize,
    pub mark: String,
}

impl Finger {
    pub fn new(uid: u16, fid: u8, valid: u8, template: Vec<u8>) -> Self {
        let size = template.len();
        let mark = format!(
            "{}...{}",
            hex_encode(&template[..8.min(size)]),
            hex_encode(&template[size.saturating_sub(8)..])
        );
        Self {
            uid,
            fid,
            valid,
            template,
            size,
            mark,
        }
    }

    pub fn repack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size + 6);
        buf.write_u16::<LittleEndian>((self.size + 6) as u16)
            .unwrap();
        buf.write_u16::<LittleEndian>(self.uid).unwrap();
        buf.write_u8(self.fid).unwrap();
        buf.write_u8(self.valid).unwrap();
        buf.extend_from_slice(&self.template);
        buf
    }

    pub fn repack_only(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size + 2);
        buf.write_u16::<LittleEndian>(self.size as u16).unwrap();
        buf.extend_from_slice(&self.template);
        buf
    }

    pub fn json_pack(&self) -> serde_json::Value {
        serde_json::json!({
            "size": self.size,
            "uid": self.uid,
            "fid": self.fid,
            "valid": self.valid,
            "template": hex_encode(&self.template),
        })
    }

    pub fn json_unpack(json: &serde_json::Value) -> Option<Self> {
        Some(Self::new(
            json.get("uid")?.as_u64()? as u16,
            json.get("fid")?.as_u64()? as u8,
            json.get("valid")?.as_u64()? as u8,
            hex_decode(json.get("template")?.as_str()?).ok()?,
        ))
    }

    pub fn dump(&self) -> String {
        format!(
            "<Finger> [uid:{:>3}, fid:{}, size:{:>4} v:{} t:{}]",
            self.uid,
            self.fid,
            self.size,
            self.valid,
            hex_encode(&self.template)
        )
    }
}

impl PartialEq for Finger {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
            && self.fid == other.fid
            && self.valid == other.valid
            && self.template == other.template
    }
}

impl fmt::Display for Finger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Finger> [uid:{:>3}, fid:{}, size:{:>4} v:{} t:{}]",
            self.uid, self.fid, self.size, self.valid, self.mark
        )
    }
}

impl fmt::Debug for Finger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Reuse Display implementation
        fmt::Display::fmt(self, f)
    }
}
