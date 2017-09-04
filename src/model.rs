use std::borrow::Cow;

// TODO: Optional chrono dependency
type DateTime = u64;
type Level = u8;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Response<'a, T> {
    user_information: UserInformation<'a>,
    requested_information: T,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ErrorResponse<'a> {
    error: Error<'a>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Error<'a> {
    code: Cow<'a, str>,
    message: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserInformation<'a> {
    username: Cow<'a, str>,
    gravatar: Cow<'a, str>,
    level: Level,
    title: Cow<'a, str>,
    about: Cow<'a, str>,
    website: Option<Cow<'a, str>>,
    twitter: Option<Cow<'a, str>>,
    topics_count: u32,
    posts_count: u32,
    creation_date: DateTime,
    vacation_date: Option<DateTime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct StudyQueue {
    lessons_available: u32,
    reviews_available: u32,
    next_review_date: Option<DateTime>,
    reviews_available_next_hour: u32,
    reviews_available_next_day: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct LevelProgression {
    radicals_progress: u32,
    radicals_total: u32,
    kanji_progress: u32,
    kanji_total: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SrsDistributionCounts {
    radicals: u32,
    kanji: u32,
    vocabulary: u32,
    total: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SrsDistribution {
    apprentice: SrsDistributionCounts,
    guru: SrsDistributionCounts,
    master: SrsDistributionCounts,
    enlighten: SrsDistributionCounts,
    burned: SrsDistributionCounts,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct RecentUnlock<'a> {
    #[serde(rename = "type")]
    item_type: Cow<'a, str>,
    character: Cow<'a, str>,
    kana: Cow<'a, str>,
    meaning: Cow<'a, str>,
    level: Level,
    unlocked_date: DateTime,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct CriticalItem<'a> {
    #[serde(rename = "type")]
    item_type: Cow<'a, str>,
    character: Cow<'a, str>,
    kana: Cow<'a, str>,
    meaning: Cow<'a, str>,
    level: Level,
    percentage: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Radical<'a> {
    level: Level,
    character: Cow<'a, str>,
    meaning: Cow<'a, str>,
    image_file_name: Option<Cow<'a, str>>,
    image_content_type: Option<Cow<'a, str>>,
    image_file_size: Option<u32>,
    user_specific: Option<UserSpecific<'a>>,
    image: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserSpecific<'a> {
    srs: Cow<'a, str>,
    srs_numeric: u32,
    unlocked_date: Option<DateTime>,
    available_date: Option<DateTime>,
    burned: bool,
    burned_date: Option<DateTime>, // can be 0
    meaning_correct: u32,
    meaning_incorrect: u32,
    meaning_max_streak: u32,
    meaning_current_streak: u32,
    reading_correct: Option<u32>, // is null for radicals
    reading_incorrect: Option<u32>,
    reading_max_streak: Option<u32>,
    reading_current_streak: Option<u32>,
    meaning_note: Option<Cow<'a, str>>,
    reading_note: Option<Cow<'a, str>>,
    user_synonyms: Option<Vec<Cow<'a, str>>>, // is null if no synonyms
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Kanji<'a> {
    level: Level,
    character: Cow<'a, str>,
    meaning: Cow<'a, str>,
    onyomi: Cow<'a, str>,
    kunyomi: Cow<'a, str>,
    important_reading: Cow<'a, str>,
    nanori: Option<Cow<'a, str>>,
    user_specific: Option<UserSpecific<'a>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Vocabulary<'a> {
    level: Level,
    character: Cow<'a, str>,
    kana: Cow<'a, str>,
    meaning: Cow<'a, str>,
    user_specific: Option<UserSpecific<'a>>,
}
