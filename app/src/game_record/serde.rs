use super::field::{GameRecord, LevelRecord, SongEntry};
use crate::phi_base::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

static DIFF_ORDER: [&str; 5] = ["EZ", "HD", "IN", "AT", "Legacy"];

#[derive(Serialize, Deserialize)]
pub struct SerializableLevelRecord {
    pub score: u32,
    pub acc: f32,
    pub fc: bool,
}
pub type SerializableSongRecord = BTreeMap<String, SerializableLevelRecord>;
#[derive(Serialize, Deserialize)]
pub struct SerializableGameRecord(BTreeMap<String, SerializableSongRecord>);

impl From<GameRecord> for SerializableGameRecord {
    fn from(gr: GameRecord) -> Self {
        let mut map: BTreeMap<String, SerializableSongRecord> = BTreeMap::new();
        for song in gr.song_list {
            let mut song_map: BTreeMap<String, SerializableLevelRecord> = BTreeMap::new();
            let mut level_idx = 0;
            for i in 0..5 {
                if song.unlock[i] {
                    let level = &song.levels[level_idx];
                    song_map.insert(
                        DIFF_ORDER[i].to_string(),
                        SerializableLevelRecord {
                            score: level.score,
                            acc: level.acc,
                            fc: song.fc[i],
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
            let mut unlock = [false; 5];
            let mut fc = [false; 5];
            let mut levels: Vec<LevelRecord> = Vec::new();
            for (i, diff) in DIFF_ORDER.iter().enumerate() {
                if let Some(rec) = song_map.get(*diff) {
                    unlock[i] = true;
                    fc[i] = rec.fc;
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
