use super::field::{Base, Chapter8Base, GameProgress, Money};
use crate::phi_base::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableBase {
    pub is_first_run: bool,
    pub legacy_chapter_finished: bool,
    pub already_show_collection_tip: bool,
    pub already_show_auto_unlock_in_tip: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableMoney {
    pub kib: u16,
    pub mib: u16,
    pub gib: u16,
    pub tib: u16,
    pub pib: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableChapter8Base {
    pub unlock_begin: bool,
    pub unlock_second_phase: bool,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableGameProgress {
    pub base: SerializableBase,
    pub completed: String,
    pub song_update_info: u16,
    pub challenge_mode_rank: u16,
    pub money: SerializableMoney,
    pub unlock_flag_of_spasmodic: [bool; 4],
    pub unlock_flag_of_igallta: [bool; 4],
    pub unlock_flag_of_rrharil: [bool; 4],
    pub flag_of_song_record_key: [bool; 8],
    pub random_version_unlocked: [bool; 6],
    pub chapter8_base: SerializableChapter8Base,
    pub chapter8_song_unlocked: [bool; 6],
    pub flag_of_song_record_key_takumi: [bool; 3],
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
