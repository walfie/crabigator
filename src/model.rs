use serde::de::{self, Deserialize, Deserializer};
use serde_json;
use std::borrow::Cow;

// TODO: Optional chrono dependency
type DateTime = u64;
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
    pub creation_date: DateTime,
    pub vacation_date: Option<DateTime>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StudyQueue {
    pub lessons_available: u32,
    pub reviews_available: u32,
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

// TODO: Change contents to be an enum? (with/without image)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Radical<'a> {
    pub level: Level,
    pub character: Option<Cow<'a, str>>,
    pub meaning: Cow<'a, str>,
    pub image_file_name: Option<Cow<'a, str>>,
    pub image_content_type: Option<Cow<'a, str>>,
    pub image_file_size: Option<u32>,
    pub user_specific: Option<UserSpecific<'a>>,
    pub image: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserSpecific<'a> {
    pub srs: Cow<'a, str>,
    pub srs_numeric: u32,
    pub unlocked_date: Option<DateTime>,
    pub available_date: Option<DateTime>,
    pub burned: bool,
    pub burned_date: Option<DateTime>, // can be 0
    pub meaning_correct: u32,
    pub meaning_incorrect: u32,
    pub meaning_max_streak: u32,
    pub meaning_current_streak: u32,
    pub reading_correct: Option<u32>, // is null for radicals
    pub reading_incorrect: Option<u32>,
    pub reading_max_streak: Option<u32>,
    pub reading_current_streak: Option<u32>,
    pub meaning_note: Option<Cow<'a, str>>,
    pub reading_note: Option<Cow<'a, str>>,
    pub user_synonyms: Option<Vec<Cow<'a, str>>>, // is null if no synonyms
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
