use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use serde_json;

pub(crate) type Level = u8;

#[cfg(feature = "chrono")]
type DateTime = ::chrono::DateTime<::chrono::Utc>;
#[cfg(feature = "chrono")]
use chrono::serde::ts_seconds;

#[cfg(not(feature = "chrono"))]
type DateTime = i64;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Response<T> {
    pub user_information: UserInformation,
    pub requested_information: T,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserInformation {
    pub username: String,
    pub gravatar: String,
    pub level: Level,
    pub title: String,
    pub about: String,
    pub website: Option<String>,
    pub twitter: Option<String>,
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
pub struct RecentUnlock {
    unlocked_date: DateTime,
    item: Item,
}

impl<'de, 'a> Deserialize<'de> for RecentUnlock {
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
pub struct CriticalItem {
    percentage: u8,
    item: Item,
}

impl<'de, 'a> Deserialize<'de> for CriticalItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PercentageHelper {
            percentage: String,
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
pub enum Item {
    #[serde(rename = "kanji")]
    Kanji(Kanji),
    #[serde(rename = "radical")]
    Radical(Radical),
    #[serde(rename = "vocabulary")]
    Vocabulary(Vocabulary),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Radical {
    pub level: Level,
    pub data: RadicalData,
    pub meaning: String,
    pub user_specific: Option<UserSpecific>,
}

#[serde(untagged)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum RadicalData {
    #[serde(rename = "character")]
    Character { character: String },
    Image {
        #[serde(rename = "image_file_name")]
        file_name: String,
        #[serde(rename = "image_content_type")]
        content_type: String,
        #[serde(rename = "image_file_size")]
        file_size: u32,
        #[serde(rename = "image")]
        url: String,
    },
}

impl<'de, 'a> Deserialize<'de> for Radical {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RadicalHelper {
            level: Level,
            meaning: String,
            user_specific: Option<UserSpecific>,
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
pub struct UserSpecific {
    pub srs: String,
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
    pub meaning_note: Option<String>,
    pub reading_correct: Option<u32>, // is null for radicals
    pub reading_incorrect: Option<u32>,
    pub reading_max_streak: Option<u32>,
    pub reading_current_streak: Option<u32>,
    pub reading_note: Option<String>,
    #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
    pub user_synonyms: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Kanji {
    pub level: Level,
    pub character: String,
    pub meaning: String,
    pub onyomi: String,
    pub kunyomi: Option<String>,
    pub important_reading: String,
    pub nanori: Option<String>,
    pub user_specific: Option<UserSpecific>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Vocabulary {
    pub level: Level,
    pub character: String,
    pub kana: String,
    pub meaning: String,
    pub user_specific: Option<UserSpecific>,
}

pub fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(|items| items.unwrap_or(Vec::with_capacity(0)))
}

mod ts_seconds_opt {
    use super::*;
    use serde::ser::Serialize;

    #[cfg(feature = "chrono")]
    #[derive(Deserialize, Serialize)]
    struct DateTimeSeconds(
        #[serde(with = "ts_seconds")]
        DateTime
    );

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[cfg(feature = "chrono")]
        {
            Option::<DateTimeSeconds>::deserialize(deserializer).map(|result| match result {
                Some(ref dt) if dt.0.timestamp() != 0 => Some(dt.0),
                _ => None,
            })
        }

        #[cfg(not(feature = "chrono"))]
        Option::<DateTime>::deserialize(deserializer).map(|result| match result {
            Some(dt) if dt != 0 => Some(dt),
            _ => None,
        })
    }

    pub fn serialize<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[cfg(feature = "chrono")]
        {
            Option::<DateTimeSeconds>::serialize(&dt.map(DateTimeSeconds), serializer)
        }

        #[cfg(not(feature = "chrono"))] Option::<DateTime>::serialize(&dt, serializer)
    }
}

#[cfg(not(feature = "chrono"))]
mod ts_seconds {
    use super::*;
    use serde::ser::Serialize;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        DateTime::deserialize(deserializer)
    }

    pub fn serialize<S>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DateTime::serialize(&dt, serializer)
    }
}
