use {
    crate::models::types::*,
    serde::{
        Deserialize,
        de::{
            self,
            Deserializer,
            Visitor
        }
    },
    anyhow::{
        Result,
        anyhow,
    },
    std::fmt
};

#[derive(Debug)]
pub enum ActivityMode {
    None,
    Story,
    Strike,
    Raid,
    AllPvP,
    Patrol,
    AllPvE,
    Reserved9,
    Control,
    Reserved11,
    Clash,
    Reserved13,
    CrimsonDoubles,
    Nightfall,
    HeroicNightfall,
    AllStrikes,
    IronBanner,
    Reserved20,
    Reserved21,
    Reserved22,
    Reserved24,
    AllMayhem,
    Reserved26,
    Reserved27,
    Reserved28,
    Reserved29,
    Reserved30,
    Supremacy,
    PrivateMatchesAll,
    Survival,
    Countdown,
    TrialsOfTheNine,
    Social,
    TrialsCountdown,
    TrialsSurvival,
    IronBannerControl,
    IronBannerClash,
    IronBannerSupremacy,
    ScoredNightfall,
    ScoredHeroicNightfall,
    Rumble,
    AllDoubles,
    Doubles,
    PrivateMatchesClash,
    PrivateMatchesControl,
    PrivateMatchesSupremacy,
    PrivateMatchesCountdown,
    PrivateMatchesSurvival,
    PrivateMatchesMayhem,
    PrivateMatchesRumble,
    HeroicAdventure,
    Showdown,
    Lockdown,
    Scorched,
    ScorchedTeam,
    Gambit,
    AllPvECompetitive,
    Breakthrough,
    BlackArmoryRun,
    Salvage,
    IronBannerSalvage,
    PvPCompetitive,
    PvPQuickplay,
    ClashQuickplay,
    ClashCompetitive,
    ControlQuickplay,
    ControlCompetitive,
    GambitPrime,
    Reckoning,
    Menagerie,
    VexOffensive,
    NightmareHunt,
    Elimination,
    Momentum,
    Dungeon,
    Sundial,
    TrialsOfOsiris,
}

impl ActivityMode {
    pub fn to_int32(&self) -> Int32 {
        match self {
            ActivityMode::None => 0,
            ActivityMode::Story => 2,
            ActivityMode::Strike => 3,
            ActivityMode::Raid => 4,
            ActivityMode::AllPvP => 5,
            ActivityMode::Patrol => 6,
            ActivityMode::AllPvE => 7,
            ActivityMode::Reserved9 => 9,
            ActivityMode::Control => 10,
            ActivityMode::Reserved11 => 11,
            ActivityMode::Clash => 12,
            ActivityMode::Reserved13 => 13,
            ActivityMode::CrimsonDoubles => 15,
            ActivityMode::Nightfall => 16,
            ActivityMode::HeroicNightfall => 17,
            ActivityMode::AllStrikes => 18,
            ActivityMode::IronBanner => 19,
            ActivityMode::Reserved20 => 20,
            ActivityMode::Reserved21 => 21,
            ActivityMode::Reserved22 => 22,
            ActivityMode::Reserved24 => 24,
            ActivityMode::AllMayhem => 25,
            ActivityMode::Reserved26 => 26,
            ActivityMode::Reserved27 => 27,
            ActivityMode::Reserved28 => 28,
            ActivityMode::Reserved29 => 29,
            ActivityMode::Reserved30 => 30,
            ActivityMode::Supremacy => 31,
            ActivityMode::PrivateMatchesAll => 32,
            ActivityMode::Survival => 37,
            ActivityMode::Countdown => 38,
            ActivityMode::TrialsOfTheNine => 39,
            ActivityMode::Social => 40,
            ActivityMode::TrialsCountdown => 41,
            ActivityMode::TrialsSurvival => 42,
            ActivityMode::IronBannerControl => 43,
            ActivityMode::IronBannerClash => 44,
            ActivityMode::IronBannerSupremacy => 45,
            ActivityMode::ScoredNightfall => 46,
            ActivityMode::ScoredHeroicNightfall => 47,
            ActivityMode::Rumble => 48,
            ActivityMode::AllDoubles => 49,
            ActivityMode::Doubles => 50,
            ActivityMode::PrivateMatchesClash => 51,
            ActivityMode::PrivateMatchesControl => 52,
            ActivityMode::PrivateMatchesSupremacy => 53,
            ActivityMode::PrivateMatchesCountdown => 54,
            ActivityMode::PrivateMatchesSurvival => 55,
            ActivityMode::PrivateMatchesMayhem => 56,
            ActivityMode::PrivateMatchesRumble => 57,
            ActivityMode::HeroicAdventure => 58,
            ActivityMode::Showdown => 59,
            ActivityMode::Lockdown => 60,
            ActivityMode::Scorched => 61,
            ActivityMode::ScorchedTeam => 62,
            ActivityMode::Gambit => 63,
            ActivityMode::AllPvECompetitive => 64,
            ActivityMode::Breakthrough => 65,
            ActivityMode::BlackArmoryRun => 66,
            ActivityMode::Salvage => 67,
            ActivityMode::IronBannerSalvage => 68,
            ActivityMode::PvPCompetitive => 69,
            ActivityMode::PvPQuickplay => 70,
            ActivityMode::ClashQuickplay => 71,
            ActivityMode::ClashCompetitive => 72,
            ActivityMode::ControlQuickplay => 73,
            ActivityMode::ControlCompetitive => 74,
            ActivityMode::GambitPrime => 75,
            ActivityMode::Reckoning => 76,
            ActivityMode::Menagerie => 77,
            ActivityMode::VexOffensive => 78,
            ActivityMode::NightmareHunt => 79,
            ActivityMode::Elimination => 80,
            ActivityMode::Momentum => 81,
            ActivityMode::Dungeon => 82,
            ActivityMode::Sundial => 83,
            ActivityMode::TrialsOfOsiris => 84,
        }
    }

    pub fn from_int32(int: Int32) -> Result<Self> {
        Ok(match int {
            0 => ActivityMode::None,
            2 => ActivityMode::Story,
            3 => ActivityMode::Strike,
            4 => ActivityMode::Raid,
            5 => ActivityMode::AllPvP,
            6 => ActivityMode::Patrol,
            7 => ActivityMode::AllPvE,
            9 => ActivityMode::Reserved9,
            10 => ActivityMode::Control,
            11 => ActivityMode::Reserved11,
            12 => ActivityMode::Clash,
            13 => ActivityMode::Reserved13,
            15 => ActivityMode::CrimsonDoubles,
            16 => ActivityMode::Nightfall,
            17 => ActivityMode::HeroicNightfall,
            18 => ActivityMode::AllStrikes,
            19 => ActivityMode::IronBanner,
            20 => ActivityMode::Reserved20,
            21 => ActivityMode::Reserved21,
            22 => ActivityMode::Reserved22,
            24 => ActivityMode::Reserved24,
            25 => ActivityMode::AllMayhem,
            26 => ActivityMode::Reserved26,
            27 => ActivityMode::Reserved27,
            28 => ActivityMode::Reserved28,
            29 => ActivityMode::Reserved29,
            30 => ActivityMode::Reserved30,
            31 => ActivityMode::Supremacy,
            32 => ActivityMode::PrivateMatchesAll,
            37 => ActivityMode::Survival,
            38 => ActivityMode::Countdown,
            39 => ActivityMode::TrialsOfTheNine,
            40 => ActivityMode::Social,
            41 => ActivityMode::TrialsCountdown,
            42 => ActivityMode::TrialsSurvival,
            43 => ActivityMode::IronBannerControl,
            44 => ActivityMode::IronBannerClash,
            45 => ActivityMode::IronBannerSupremacy,
            46 => ActivityMode::ScoredNightfall,
            47 => ActivityMode::ScoredHeroicNightfall,
            48 => ActivityMode::Rumble,
            49 => ActivityMode::AllDoubles,
            50 => ActivityMode::Doubles,
            51 => ActivityMode::PrivateMatchesClash,
            52 => ActivityMode::PrivateMatchesControl,
            53 => ActivityMode::PrivateMatchesSupremacy,
            54 => ActivityMode::PrivateMatchesCountdown,
            55 => ActivityMode::PrivateMatchesSurvival,
            56 => ActivityMode::PrivateMatchesMayhem,
            57 => ActivityMode::PrivateMatchesRumble,
            58 => ActivityMode::HeroicAdventure,
            59 => ActivityMode::Showdown,
            60 => ActivityMode::Lockdown,
            61 => ActivityMode::Scorched,
            62 => ActivityMode::ScorchedTeam,
            63 => ActivityMode::Gambit,
            64 => ActivityMode::AllPvECompetitive,
            65 => ActivityMode::Breakthrough,
            66 => ActivityMode::BlackArmoryRun,
            67 => ActivityMode::Salvage,
            68 => ActivityMode::IronBannerSalvage,
            69 => ActivityMode::PvPCompetitive,
            70 => ActivityMode::PvPQuickplay,
            71 => ActivityMode::ClashQuickplay,
            72 => ActivityMode::ClashCompetitive,
            73 => ActivityMode::ControlQuickplay,
            74 => ActivityMode::ControlCompetitive,
            75 => ActivityMode::GambitPrime,
            76 => ActivityMode::Reckoning,
            77 => ActivityMode::Menagerie,
            78 => ActivityMode::VexOffensive,
            79 => ActivityMode::NightmareHunt,
            80 => ActivityMode::Elimination,
            81 => ActivityMode::Momentum,
            82 => ActivityMode::Dungeon,
            83 => ActivityMode::Sundial,
            84 => ActivityMode::TrialsOfOsiris,
            _ => return Err(anyhow!("Invalid int32 for activitymode"))
        })
    }
}

struct ActivityModeVisitor;

impl<'de> Visitor<'de> for ActivityModeVisitor {
    type Value = ActivityMode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i32 representing a valid ActivityMode")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match ActivityMode::from_int32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }
}

impl<'de> Deserialize<'de> for ActivityMode {
    fn deserialize<D>(d: D) -> Result<ActivityMode, D::Error>
    where
        D: Deserializer<'de>
    {
        d.deserialize_i32(ActivityModeVisitor)
    }
}