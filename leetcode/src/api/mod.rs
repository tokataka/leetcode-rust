mod models;

pub use models::*;

use reqwest::blocking as reqwest;

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
        meta_data: serde_json::from_str(&res.data.question.meta_data).unwrap(),
    })
}

pub fn get_daily_problem_id() -> Option<u32> {
    let client = reqwest::Client::new();
    let res: RawDaily = client
        .post(GRAPHQL_URL)
        .json(&Query::daily_query())
        .send()
        .unwrap()
        .json()
        .ok()?;

    res.data
        .active_daily_coding_challenge_question
        .question
        .frontend_question_id
        .parse()
        .ok()
}

pub fn get_random_problem_title_slug(difficulties: &[&str]) -> Option<u32> {
    let client = reqwest::Client::new();
    let res: RawRandom = client
        .post(GRAPHQL_URL)
        .json(&Query::random_query(difficulties))
        .send()
        .unwrap()
        .json()
        .ok()?;

    Some(
        res.data
            .random_question_v2
            .question_frontend_id
            .parse()
            .unwrap(),
    )
}
