use crate::api::{Data, empty_data, malloc_data};
use crate::phi_field::base::*;
use crate::phi_field::game_progress::{Base, Money, Chapter8Base, GameProgress};
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use shua_struct::field::BinaryField;

#[derive(Serialize, Deserialize)]
struct SerializableBase {
    is_first_run: bool,
    legacy_chapter_finished: bool,
    already_show_collection_tip: bool,
    already_show_auto_unlock_in_tip: bool,
}

#[derive(Serialize, Deserialize)]
struct SerializableMoney {
    kib: u16,
    mib: u16,
    gib: u16,
    tib: u16,
    pib: u16,
}

#[derive(Serialize, Deserialize)]
struct SerializableChapter8Base {
    unlock_begin: bool,
    unlock_second_phase: bool,
    passed: bool,
}

#[derive(Serialize, Deserialize)]
struct SerializableGameProgress {
    base: SerializableBase,
    completed: String,
    song_update_info: u16,
    challenge_mode_rank: u16,
    money: SerializableMoney,
    unlock_flag_of_spasmodic: [bool; 4],
    unlock_flag_of_igallta: [bool; 4],
    unlock_flag_of_rrharil: [bool; 4],
    flag_of_song_record_key: [bool; 8],
    random_version_unlocked: [bool; 6],
    chapter8_base: SerializableChapter8Base,
    chapter8_song_unlocked: [bool; 6],
    flag_of_song_record_key_takumi: [bool; 3],
}

impl From<Base> for SerializableBase {
    fn from(b: Base) -> Self {
        Self {
            is_first_run: b.is_first_run,
            legacy_chapter_finished: b.legacy_chapter_finished,
            already_show_collection_tip: b.already_show_collection_tip,
            already_show_auto_unlock_in_tip: b.already_show_auto_unlock_in_tip,
        }
    }
}

impl From<SerializableBase> for Base {
    fn from(b: SerializableBase) -> Self {
        Base {
            is_first_run: b.is_first_run,
            legacy_chapter_finished: b.legacy_chapter_finished,
            already_show_collection_tip: b.already_show_collection_tip,
            already_show_auto_unlock_in_tip: b.already_show_auto_unlock_in_tip,
        }
    }
}

impl From<Money> for SerializableMoney {
    fn from(m: Money) -> Self {
        Self {
            kib: m.kib.0,
            mib: m.mib.0,
            gib: m.gib.0,
            tib: m.tib.0,
            pib: m.pib.0,
        }
    }
}

impl From<SerializableMoney> for Money {
    fn from(m: SerializableMoney) -> Self {
        Money {
            kib: VarInt(m.kib),
            mib: VarInt(m.mib),
            gib: VarInt(m.gib),
            tib: VarInt(m.tib),
            pib: VarInt(m.pib),
        }
    }
}

impl From<Chapter8Base> for SerializableChapter8Base {
    fn from(c: Chapter8Base) -> Self {
        Self {
            unlock_begin: c.unlock_begin,
            unlock_second_phase: c.unlock_second_phase,
            passed: c.passed,
        }
    }
}

impl From<SerializableChapter8Base> for Chapter8Base {
    fn from(c: SerializableChapter8Base) -> Self {
        Chapter8Base {
            unlock_begin: c.unlock_begin,
            unlock_second_phase: c.unlock_second_phase,
            passed: c.passed,
        }
    }
}

impl From<GameProgress> for SerializableGameProgress {
    fn from(g: GameProgress) -> Self {
        Self {
            base: g.base.into(),
            completed: g.completed.0,
            song_update_info: g.song_update_info.0,
            challenge_mode_rank: g.challenge_mode_rank,
            money: g.money.into(),
            unlock_flag_of_spasmodic: g.unlock_flag_of_spasmodic,
            unlock_flag_of_igallta: g.unlock_flag_of_igallta,
            unlock_flag_of_rrharil: g.unlock_flag_of_rrharil,
            flag_of_song_record_key: g.flag_of_song_record_key,
            random_version_unlocked: g.random_version_unlocked,
            chapter8_base: g.chapter8_base.into(),
            chapter8_song_unlocked: g.chapter8_song_unlocked,
            flag_of_song_record_key_takumi: g.flag_of_song_record_key_takumi,
        }
    }
}

impl From<SerializableGameProgress> for GameProgress {
    fn from(g: SerializableGameProgress) -> Self {
        GameProgress {
            base: g.base.into(),
            completed: PhiString(g.completed),
            song_update_info: VarInt(g.song_update_info),
            challenge_mode_rank: g.challenge_mode_rank,
            money: g.money.into(),
            unlock_flag_of_spasmodic: g.unlock_flag_of_spasmodic,
            unlock_flag_of_igallta: g.unlock_flag_of_igallta,
            unlock_flag_of_rrharil: g.unlock_flag_of_rrharil,
            flag_of_song_record_key: g.flag_of_song_record_key,
            random_version_unlocked: g.random_version_unlocked,
            chapter8_base: g.chapter8_base.into(),
            chapter8_song_unlocked: g.chapter8_song_unlocked,
            flag_of_song_record_key_takumi: g.flag_of_song_record_key_takumi,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_game_progress(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }

    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    let bits = BitSlice::<u8, Lsb0>::from_slice(bytes);

    let (gp, _) = match GameProgress::parse(bits, &None) {
        Ok(r) => r,
        Err(_) => return empty_data(),
    };

    let serializable = SerializableGameProgress::from(gp);

    let buf = match rmp_serde::to_vec_named(&serializable) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    unsafe { malloc_data(buf) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_game_progress(data_ptr: *const u8, data_len: usize) -> Data {
    if data_ptr.is_null() || data_len == 0 {
        return empty_data();
    }

    let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };

    let serializable: SerializableGameProgress = match rmp_serde::from_slice(bytes) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    let gp: GameProgress = serializable.into();

    let bitvec = match gp.build(&None) {
        Ok(v) => v,
        Err(_) => return empty_data(),
    };

    unsafe { malloc_data(bitvec.into_vec()) }
}
