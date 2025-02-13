use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(feature = "enum-iter")]
pub use strum::IntoEnumIterator;

pub const FINLAND_STAGES: [&'static str; 6] = [
    "noormarkku",
    "lamppi",
    "palus",
    "lassila",
    "kairila",
    "haapajärvi",
];
pub const SARDINIA_STAGES: [&'static str; 6] = [
    "villacidro",
    "san gavino monreale",
    "san benedetto",
    "gennamari",
    "portu maga",
    "montevecchio",
];
pub const JAPAN_STAGES: [&'static str; 6] = [
    "nasu highland",
    "mount asama",
    "mount akagi",
    "nikko",
    "tsumagoi",
    "mount haruna",
];
pub const NORWAY_STAGES: [&'static str; 6] = [
    "lapstad",
    "vestpollen",
    "stronstad",
    "kvannkjosen",
    "grunnfor",
    "lake rostavatn",
];
pub const GERMANY_STAGES: [&'static str; 6] = [
    "hockweiler",
    "franzenheim",
    "holzerath",
    "farschweiler",
    "mertesdorf",
    "gonnesweiler",
];
pub const KENYA_STAGES: [&'static str; 6] = [
    "mount kenya",
    "karura",
    "homa bay",
    "ndere island",
    "lake baringo",
    "lake nakuru",
];
pub const INDONESIA_STAGES: [&'static str; 6] = [
    "mount kawi",
    "semangka bay",
    "satonda island",
    "oreng valley",
    "sangeang island",
    "kalabakan valley",
];
pub const AUSTRALIA_STAGES: [&'static str; 6] = [
    "gum scrub",
    "toorooka",
    "nulla nulla",
    "comara canyon",
    "lake lucernia",
    "wombamurra",
];

#[derive(Debug, Clone, Copy)]
pub struct Stage {
    pub area: Area,
    pub stage_number: usize, // in range 1..=6
    pub direction: Direction,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!((1..=6).contains(&self.stage_number));
        let stages = match self.area {
            Area::Finland => FINLAND_STAGES,
            Area::Sardinia => SARDINIA_STAGES,
            Area::Japan => JAPAN_STAGES,
            Area::Norway => NORWAY_STAGES,
            Area::Germany => GERMANY_STAGES,
            Area::Kenya => KENYA_STAGES,
            Area::Indonesia => INDONESIA_STAGES,
            Area::Australia => AUSTRALIA_STAGES,
        };
        write!(
            f,
            "{}{}",
            stages[self.stage_number - 1],
            if self.direction == Direction::Forward {
                ""
            } else {
                " - r"
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enum-iter", derive(strum::EnumIter))]
pub enum Area {
    Finland,
    Sardinia,
    Japan,
    Norway,
    Germany,
    Kenya,
    Indonesia,
    Australia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enum-iter", derive(strum::EnumIter))]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enum-iter", derive(strum::EnumIter))]
pub enum Weather {
    Dry,
    Wet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enum-iter", derive(strum::EnumIter))]
pub enum Group {
    Sixties,
    Seventies,
    Eighties,
    GroupB,
    GroupS,
    GroupA,
    BonusVans,
    BonusPiaggio,
    BonusDakar,
    BonusLogging,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Group::Sixties => "sixties",
            Group::Seventies => "seventies",
            Group::Eighties => "eighties",
            Group::GroupB => "group b",
            Group::GroupS => "group s",
            Group::GroupA => "group a",
            Group::BonusVans => "vans",
            Group::BonusPiaggio => "triwheelers",
            Group::BonusDakar => "trucks",
            Group::BonusLogging => "logging",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Filter {
    Top,
    AroundMe,
    Number,
    Count,
    PlayerRank,
    NumberOne,
    Friends,
    OnlyMe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    Epic,
    Gog,
    Steam,
    Xbox,
    Playstation,
    Nintendo,
    None,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub leaderboard: Vec<LeaderboardEntry>,
    pub result: isize, // ?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    #[serde(rename = "uniqueID")]
    pub unique_id: u64, // ?
    #[serde(rename = "userName")]
    pub user_name: String,
    pub rank: usize,
    pub score: usize,
    pub country: usize,
    #[serde(rename = "carID")]
    pub car_id: usize,
    #[serde(rename = "replayData")]
    pub replay_data: String,
    #[serde(rename = "platformID")]
    pub platform_id: u8,
}

#[derive(Debug, Clone)]
pub struct Leaderboard {
    pub stage: Stage,
    pub weather: Weather,
    pub group: Group,
    pub filter: Filter,
    pub platform: Platform,
}

impl Leaderboard {
    fn fmt_area(area: Area) -> &'static str {
        match area {
            Area::Finland => "Finland",
            Area::Sardinia => "Sardinia",
            Area::Japan => "Japan",
            Area::Norway => "Norway",
            Area::Germany => "Germany",
            Area::Kenya => "Kenya",
            Area::Indonesia => "Indonesia",
            Area::Australia => "Australia",
        }
    }

    fn fmt_direction(direction: Direction) -> &'static str {
        match direction {
            Direction::Forward => "Forward",
            Direction::Backward => "Backward",
        }
    }

    fn fmt_weather(weather: Weather) -> &'static str {
        match weather {
            Weather::Dry => "Dry",
            Weather::Wet => "Wet",
        }
    }

    fn fmt_group(group: Group) -> &'static str {
        match group {
            Group::Sixties => "60s",
            Group::Seventies => "70s",
            Group::Eighties => "80s",
            Group::GroupB => "GroupB",
            Group::GroupS => "GroupS",
            Group::GroupA => "GroupA",
            Group::BonusVans => "Bonus_Vans",
            Group::BonusPiaggio => "Bonus_Monkey",
            Group::BonusDakar => "Bonus_Dakar",
            Group::BonusLogging => "Bonus_Logging",
        }
    }

    fn fmt_filter(filter: Filter) -> usize {
        match filter {
            Filter::Top => 0,
            Filter::AroundMe => 1,
            Filter::Number => 2,
            Filter::Count => 3,
            Filter::PlayerRank => 4,
            Filter::NumberOne => 5,
            Filter::Friends => 6,
            Filter::OnlyMe => 7,
        }
    }

    fn fmt_platform(platform: Platform) -> usize {
        match platform {
            Platform::Epic => 0,
            Platform::Gog => 1,
            Platform::Steam => 2,
            Platform::Xbox => 3,
            Platform::Playstation => 4,
            Platform::Nintendo => 5,
            Platform::None => 6,
        }
    }

    pub fn as_url(&self, user: u64, friends: &[u64]) -> String {
        let area = Self::fmt_area(self.stage.area);
        let stage = self.stage.stage_number;
        let direction = Self::fmt_direction(self.stage.direction);
        let weather = Self::fmt_weather(self.weather);
        let group = Self::fmt_group(self.group);
        let filter = Self::fmt_filter(self.filter);
        let platform = Self::fmt_platform(self.platform);
        let friends = friends
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!("https://www.funselektorfun.com/artofrally/leaderboard/{area}_Stage_{stage}_{direction}_{weather}_{group}/{filter}/{platform}/{user}/[{friends}]")
    }
}
