use super::field::{Level, MultiLevel, Summary};
use crate::phi_base::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableLevel {
    pub clear: u16,
    pub fc: u16,
    pub phi: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableMultiLevel {
    pub ez: SerializableLevel,
    pub hd: SerializableLevel,
    #[serde(rename = "in")]
    pub r#in: SerializableLevel,
    pub at: SerializableLevel,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableSummary {
    pub save_version: u8,
    pub challenge_mode_rank: u16,
    pub rks: f32,
    pub game_version: u16,
    pub avatar: String,
    pub level: SerializableMultiLevel,
}

impl From<Level> for SerializableLevel {
    fn from(l: Level) -> Self {
        Self {
            clear: l.clear,
            fc: l.fc,
            phi: l.phi,
        }
    }
}

impl From<SerializableLevel> for Level {
    fn from(l: SerializableLevel) -> Self {
        Level {
            clear: l.clear,
            fc: l.fc,
            phi: l.phi,
        }
    }
}

impl From<MultiLevel> for SerializableMultiLevel {
    fn from(m: MultiLevel) -> Self {
        Self {
            ez: m.ez.into(),
            hd: m.hd.into(),
            r#in: m.r#in.into(),
            at: m.at.into(),
        }
    }
}

impl From<SerializableMultiLevel> for MultiLevel {
    fn from(m: SerializableMultiLevel) -> Self {
        MultiLevel {
            ez: m.ez.into(),
            hd: m.hd.into(),
            r#in: m.r#in.into(),
            at: m.at.into(),
        }
    }
}

impl From<Summary> for SerializableSummary {
    fn from(s: Summary) -> Self {
        Self {
            save_version: s.save_version,
            challenge_mode_rank: s.challenge_mode_rank,
            rks: s.rks,
            game_version: s.game_version.0,
            avatar: s.avatar.0,
            level: s.level.into(),
        }
    }
}

impl From<SerializableSummary> for Summary {
    fn from(s: SerializableSummary) -> Self {
        Summary {
            save_version: s.save_version,
            challenge_mode_rank: s.challenge_mode_rank,
            rks: s.rks,
            game_version: VarInt(s.game_version),
            avatar: PhiString(s.avatar),
            level: s.level.into(),
        }
    }
}
