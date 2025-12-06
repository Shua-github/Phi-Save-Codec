use crate::api::main::malloc_str;
use crate::phi_field::base::*;
use crate::phi_field::game_record::{GameRecord, LevelRecord, SongEntry};
use base64::{Engine, engine::general_purpose};
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Serialize, Deserialize)]
struct SerializableLevelRecord {
    score: u32,
    acc: f32,
    fc: bool,
}

type SerializableSongRecord = HashMap<String, SerializableLevelRecord>;

#[derive(Serialize, Deserialize)]
struct SerializableGameRecord(HashMap<String, SerializableSongRecord>);

impl From<GameRecord> for SerializableGameRecord {
    fn from(gr: GameRecord) -> Self {
        let diffs = ["EZ", "HD", "IN", "AT", "Legacy"];
        let mut map: HashMap<String, SerializableSongRecord> = HashMap::new();
        for song in gr.song_list {
            let mut song_map: HashMap<String, SerializableLevelRecord> = HashMap::new();
            let mut level_idx = 0;
            for i in 0..5 {
                if song.unlock[i].0 {
                    let level = &song.levels[level_idx];
                    song_map.insert(
                        diffs[i].to_string(),
                        SerializableLevelRecord {
                            score: level.score,
                            acc: level.acc,
                            fc: song.fc[i].0,
                        },
                    );
                    level_idx += 1;
                }
            }
            map.insert(song.name.0, song_map);
        }
        SerializableGameRecord(map)
    }
}

impl From<SerializableGameRecord> for GameRecord {
    fn from(sgr: SerializableGameRecord) -> Self {
        let diff_map: HashMap<&str, usize> =
            HashMap::from([("EZ", 0), ("HD", 1), ("IN", 2), ("AT", 3), ("Legacy", 4)]);
        let diff_order = ["EZ", "HD", "IN", "AT", "Legacy"];
        let mut song_list: Vec<SongEntry> = Vec::new();
        for (name, song_map) in sgr.0 {
            let mut unlock = [BitBool(false); 5];
            let mut fc = [BitBool(false); 5];
            let mut levels: Vec<LevelRecord> = Vec::new();
            for &diff in diff_order.iter() {
                if let Some(rec) = song_map.get(diff) {
                    let idx = *diff_map.get(diff).unwrap();
                    unlock[idx] = BitBool(true);
                    fc[idx] = BitBool(rec.fc);
                    levels.push(LevelRecord {
                        score: rec.score,
                        acc: rec.acc,
                    });
                }
            }
            song_list.push(SongEntry {
                name: PhiString(name),
                length: VarInt((levels.len() as u16) * 8 + 2),
                unlock,
                fc,
                levels,
            });
        }
        GameRecord {
            song_sum: VarInt(song_list.len() as u16),
            song_list,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_game_record(base64_str_ptr: *const c_char) -> *mut c_char {
    if base64_str_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(base64_str_ptr) };
    let base64_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let bytes = match general_purpose::STANDARD.decode(base64_str) {
        Ok(d) => d,
        Err(_) => return std::ptr::null_mut(),
    };
    let bits = BitSlice::<u8, Lsb0>::from_slice(&bytes);
    let mut ctx = HashMap::new();
    let (game_record, _) = match GameRecord::parse(bits, &mut ctx, None, None) {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };
    let serializable = SerializableGameRecord::from(game_record);
    let json = match serde_json::to_string(&serializable) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe { malloc_str(json) }
}

#[unsafe(no_mangle)]
pub extern "C" fn build_game_record(json_ptr: *const c_char) -> *mut c_char {
    if json_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(json_ptr) };
    let json_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let serializable: SerializableGameRecord = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    let game_record: GameRecord = GameRecord::from(serializable);
    let bitvec = game_record.build();
    let bytes_vec = bitvec.into_vec();
    let base64_str = general_purpose::STANDARD.encode(&bytes_vec);
    unsafe { malloc_str(base64_str) }
}
