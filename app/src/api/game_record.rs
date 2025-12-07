use crate::api::{Data, malloc_bytes};
use crate::phi_field::base::*;
use crate::phi_field::game_record::{GameRecord, LevelRecord, SongEntry};
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;
use std::collections::HashMap;

static DIFF_ORDER: [&str; 5] = ["EZ", "HD", "IN", "AT", "Legacy"];

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
        let mut map: HashMap<String, SerializableSongRecord> = HashMap::new();
        for song in gr.song_list {
            let mut song_map: HashMap<String, SerializableLevelRecord> = HashMap::new();
            let mut level_idx = 0;
            for i in 0..5 {
                if song.unlock[i].0 {
                    let level = &song.levels[level_idx];
                    song_map.insert(
                        DIFF_ORDER[i].to_string(),
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
        let mut song_list: Vec<SongEntry> = Vec::new();
        for (name, song_map) in sgr.0 {
            let mut unlock = [BitBool(false); 5];
            let mut fc = [BitBool(false); 5];
            let mut levels: Vec<LevelRecord> = Vec::new();
            for (i, diff) in DIFF_ORDER.iter().enumerate() {
                if let Some(rec) = song_map.get(*diff) {
                    unlock[i] = BitBool(true);
                    fc[i] = BitBool(rec.fc);
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
pub unsafe extern "C" fn parse_game_record(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return Data {
            len: 0,
            ptr: std::ptr::null_mut(),
        };
    }

    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };

    let bits = BitSlice::<u8, Lsb0>::from_slice(&bytes);
    let mut ctx = HashMap::new();
    let (game_record, _) = match GameRecord::parse(bits, &mut ctx, None, None) {
        Ok(r) => r,
        Err(_) => {
            return Data {
                len: 0,
                ptr: std::ptr::null_mut(),
            };
        }
    };

    let serializable = SerializableGameRecord::from(game_record);
    let json = match serde_json::to_vec(&serializable) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Data {
                len: 0,
                ptr: std::ptr::null_mut(),
            };
        }
    };

    unsafe { malloc_bytes(json) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_game_record(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return Data {
            len: 0,
            ptr: std::ptr::null_mut(),
        };
    }

    let json_bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };

    let serializable: SerializableGameRecord = match serde_json::from_slice(json_bytes) {
        Ok(v) => v,
        Err(_) => {
            return Data {
                len: 0,
                ptr: std::ptr::null_mut(),
            };
        }
    };

    let game_record: GameRecord = GameRecord::from(serializable);
    let bitvec = game_record.build();
    let bytes = bitvec.into_vec();

    unsafe { malloc_bytes(bytes) }
}
