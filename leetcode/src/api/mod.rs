mod models;

pub use models::*;

use reqwest::blocking as reqwest;
use serde_json::Value;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

pub fn list_problems() -> Option<ProblemStats> {
    reqwest::get(PROBLEMS_URL).unwrap().json().ok()?
}

pub fn get_problem(problem_stat: &StatWithStatus) -> Option<Problem> {
    let client = reqwest::Client::new();
    let res: RawProblem = client
        .post(GRAPHQL_URL)
        .json(&Query::question_query(
            problem_stat.stat.question_title_slug.as_ref().unwrap(),
        ))
        .send()
        .unwrap()
        .json()
        .ok()?;

    Some(Problem {
        title: problem_stat.stat.question_title.clone().unwrap(),
        title_slug: problem_stat.stat.question_title_slug.clone().unwrap(),
        code_definition: serde_json::from_str(&res.data.question.code_definition).unwrap(),
        content: res.data.question.content,
        sample_test_case: res.data.question.sample_test_case,
        difficulty: problem_stat.difficulty.to_string(),
        question_id: problem_stat.stat.frontend_question_id,
        return_type: {
            let v: Value = serde_json::from_str(&res.data.question.meta_data).unwrap();
            v["return"]["type"].to_string().replace("\"", "")
        },
    })
}
