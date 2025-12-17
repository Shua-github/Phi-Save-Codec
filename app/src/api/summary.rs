use crate::api::{Data, empty_data, malloc_data};
use crate::phi_field::base::*;
use crate::phi_field::summary::{Summary, MultiLevel, Level};
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;

// ---------------- Serializable structs ----------------

#[derive(Serialize, Deserialize)]
struct SerializableLevel {
    clear: u16,
    fc: u16,
    phi: u16,
}

#[derive(Serialize, Deserialize)]
struct SerializableMultiLevel {
    ez: SerializableLevel,
    hd: SerializableLevel,
    #[serde(rename = "in")]
    r#in: SerializableLevel,
    at: SerializableLevel,
}

#[derive(Serialize, Deserialize)]
struct SerializableSummary {
    save_version: u8,
    challenge_mode_rank: u16,
    rks: f32,
    game_version: u16,
    avatar: String,
    level: SerializableMultiLevel,
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_summary(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }

    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let bits = BitSlice::<u8, Lsb0>::from_slice(bytes);

    let (summary, _) = match Summary::parse(bits, &None) {
        Ok(r) => r,
        Err(_) => return empty_data(),
    };

    let serializable = SerializableSummary::from(summary);

    let bin = match rmp_serde::to_vec_named(&serializable) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    unsafe { malloc_data(bin) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_summary(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }

    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };

    let serializable: SerializableSummary = match rmp_serde::from_slice(bytes) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    let summary: Summary = Summary::from(serializable);

    let bitvec = match summary.build(&None) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    unsafe { malloc_data(bitvec.into_vec()) }
}
