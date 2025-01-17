use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Error, Formatter};

const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";

const DAILY_QUERY_STRING: &str = r#"
query questionOfToday {
    activeDailyCodingChallengeQuestion {
        question {
            frontendQuestionId: questionFrontendId
        }
    }
}"#;
const DAILY_QUERY_OPERATION: &str = "questionOfToday";

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub meta_data: MetaData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub name: Option<String>,
    pub params: Option<Vec<MetaDataParam>>,
    #[serde(rename = "return")]
    pub return_: Option<MetaDataReturn>,
    // systemdesign only (class design)
    pub systemdesign: Option<bool>,
    pub classname: Option<String>,
    pub constructor: Option<MetaDataConstructor>,
    pub methods: Option<Vec<MetaDataMethod>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataParam {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub dealloc: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataReturn {
    #[serde(rename = "type")]
    pub type_: String,
    pub dealloc: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataConstructor {
    pub params: Vec<MetaDataParam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataMethod {
    pub name: String,
    pub params: Vec<MetaDataParam>,
    #[serde(rename = "return")]
    pub return_: MetaDataReturn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemStats {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    pub question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawProblem {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub content: String,
    pub stats: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: String,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(rename = "metaData")]
    pub meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawDaily {
    pub data: DailyData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyData {
    #[serde(rename = "activeDailyCodingChallengeQuestion")]
    pub active_daily_coding_challenge_question: ActiveDailyCodingChallengeQuestion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveDailyCodingChallengeQuestion {
    pub question: DailyQuestion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyQuestion {
    #[serde(rename = "frontendQuestionId")]
    pub frontend_question_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    pub fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }

    pub fn daily_query() -> Query {
        Query {
            operation_name: DAILY_QUERY_OPERATION.to_owned(),
            variables: json!({}),
            query: DAILY_QUERY_STRING.to_owned(),
        }
    }
}
