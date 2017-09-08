use chrono::serde::ts_seconds;
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use serde_json;
use std::borrow::Cow;
type DateTime = ::chrono::DateTime<::chrono::Utc>;

mod ts_seconds_opt {
    use super::*;
    use serde::ser::Serialize;

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct DateTimeSeconds(
        #[serde(with = "ts_seconds")]
        DateTime
    );

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<DateTimeSeconds>::deserialize(deserializer).map(|result| match result {
            Some(ref dt) if dt.0.timestamp() != 0 => Some(dt.0),
            _ => None,
        })
    }

    pub fn serialize<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Option::<DateTime>::serialize(&dt, serializer)
    }
}

pub(crate) type Level = u8;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Response<'a, T> {
    pub user_information: UserInformation<'a>,
    pub requested_information: T,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ErrorResponse<'a> {
    pub error: Error<'a>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Error<'a> {
    pub code: Cow<'a, str>,
    pub message: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserInformation<'a> {
    pub username: Cow<'a, str>,
    pub gravatar: Cow<'a, str>,
    pub level: Level,
    pub title: Cow<'a, str>,
    pub about: Cow<'a, str>,
    pub website: Option<Cow<'a, str>>,
    pub twitter: Option<Cow<'a, str>>,
    pub topics_count: u32,
    pub posts_count: u32,
    #[serde(with = "ts_seconds")]
    pub creation_date: DateTime,
    #[serde(with = "ts_seconds_opt")]
    pub vacation_date: Option<DateTime>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StudyQueue {
    pub lessons_available: u32,
    pub reviews_available: u32,
    #[serde(with = "ts_seconds_opt")]
    pub next_review_date: Option<DateTime>,
    pub reviews_available_next_hour: u32,
    pub reviews_available_next_day: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LevelProgression {
    pub radicals_progress: u32,
    pub radicals_total: u32,
    pub kanji_progress: u32,
    pub kanji_total: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SrsDistributionCounts {
    pub radicals: u32,
    pub kanji: u32,
    pub vocabulary: u32,
    pub total: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SrsDistribution {
    pub apprentice: SrsDistributionCounts,
    pub guru: SrsDistributionCounts,
    pub master: SrsDistributionCounts,
    pub enlighten: SrsDistributionCounts,
    pub burned: SrsDistributionCounts,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecentUnlock<'a> {
    unlocked_date: DateTime,
    item: Item<'a>,
}

impl<'de, 'a> Deserialize<'de> for RecentUnlock<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct UnlockedDateHelper {
            unlocked_date: DateTime,
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        let UnlockedDateHelper { unlocked_date } = UnlockedDateHelper::deserialize(&v).map_err(
            de::Error::custom,
        )?;

        let item = Item::deserialize(v).map_err(de::Error::custom)?;
        Ok(RecentUnlock {
            unlocked_date,
            item,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CriticalItem<'a> {
    percentage: u8,
    item: Item<'a>,
}

impl<'de, 'a> Deserialize<'de> for CriticalItem<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PercentageHelper<'a> {
            percentage: Cow<'a, str>,
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        let PercentageHelper { percentage } =
            PercentageHelper::deserialize(&v).map_err(de::Error::custom)?;

        let percentage = percentage.parse::<u8>().map_err(|_| {
            de::Error::invalid_value(
                de::Unexpected::Str(percentage.as_ref()),
                &"a numeric string",
            )
        })?;

        let item = Item::deserialize(v).map_err(de::Error::custom)?;
        Ok(CriticalItem { percentage, item })
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Item<'a> {
    #[serde(rename = "kanji")]
    Kanji(Kanji<'a>),
    #[serde(rename = "radical")]
    Radical(Radical<'a>),
    #[serde(rename = "vocabulary")]
    Vocabulary(Vocabulary<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Radical<'a> {
    pub level: Level,
    pub data: RadicalData<'a>,
    pub meaning: Cow<'a, str>,
    pub user_specific: Option<UserSpecific<'a>>,
}

#[serde(untagged)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum RadicalData<'a> {
    #[serde(rename = "character")]
    Character { character: Cow<'a, str> },
    Image {
        #[serde(rename = "image_file_name")]
        file_name: Cow<'a, str>,
        #[serde(rename = "image_content_type")]
        content_type: Cow<'a, str>,
        #[serde(rename = "image_file_size")]
        file_size: u32,
        #[serde(rename = "image")]
        url: Cow<'a, str>,
    },
}

impl<'de, 'a> Deserialize<'de> for Radical<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RadicalHelper<'a> {
            level: Level,
            meaning: Cow<'a, str>,
            user_specific: Option<UserSpecific<'a>>,
        }

        let v = serde_json::Value::deserialize(deserializer)?;
        let common = RadicalHelper::deserialize(&v).map_err(de::Error::custom)?;
        let data = RadicalData::deserialize(&v).map_err(de::Error::custom)?;

        Ok(Radical {
            level: common.level,
            meaning: common.meaning,
            user_specific: common.user_specific,
            data,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserSpecific<'a> {
    pub srs: Cow<'a, str>,
    pub srs_numeric: u32,
    #[serde(with = "ts_seconds_opt")]
    pub unlocked_date: Option<DateTime>,
    #[serde(with = "ts_seconds_opt")]
    pub available_date: Option<DateTime>,
    pub burned: bool,
    #[serde(with = "ts_seconds_opt")]
    pub burned_date: Option<DateTime>, // can be 0
    pub meaning_correct: u32,
    pub meaning_incorrect: u32,
    pub meaning_max_streak: u32,
    pub meaning_current_streak: u32,
    pub meaning_note: Option<Cow<'a, str>>,
    pub reading_correct: Option<u32>, // is null for radicals
    pub reading_incorrect: Option<u32>,
    pub reading_max_streak: Option<u32>,
    pub reading_current_streak: Option<u32>,
    pub reading_note: Option<Cow<'a, str>>,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub user_synonyms: Vec<Cow<'a, str>>,
}

pub fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(|items| items.unwrap_or(Vec::with_capacity(0)))
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Kanji<'a> {
    pub level: Level,
    pub character: Cow<'a, str>,
    pub meaning: Cow<'a, str>,
    pub onyomi: Cow<'a, str>,
    pub kunyomi: Option<Cow<'a, str>>,
    pub important_reading: Cow<'a, str>,
    pub nanori: Option<Cow<'a, str>>,
    pub user_specific: Option<UserSpecific<'a>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Vocabulary<'a> {
    pub level: Level,
    pub character: Cow<'a, str>,
    pub kana: Cow<'a, str>,
    pub meaning: Cow<'a, str>,
    pub user_specific: Option<UserSpecific<'a>>,
}
