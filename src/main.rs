pub mod solution;

use serde::{Deserialize, Serialize};
use tokio::fs::{write, read_to_string, read};
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long="id", short="i")]
    id: i64,
}

const QUESTIONS_URL: &str = "https://leetcode.cn/api/problems/algorithms/";
const LEETCODE_GRAPHQL_URL: &str = "https://leetcode.cn/graphql/";

#[tokio::main]
async fn main() {
    // init cli
    let args = Cli::from_args();
    let id = args.id.clone();
    let client = reqwest::Client::new();
    // get question stats
    let stats = get_question_stats_by_question_id(id).await.unwrap();
    // request from leetcode graphql
    let req = QuestionRequest{
        operation_name:"questionData".to_string(), 
        query: "query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    titleSlug\n    content\n    translatedTitle\n    sampleTestCase\n   translatedContent\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n}\n}\n".to_string(), 
        variables: GraphqlVariables { title_slug: stats.question_title_slug.clone() } 
    };
    let req_str = serde_json::to_string(&req).unwrap();
    let res = client
        .post(LEETCODE_GRAPHQL_URL)
        .body(req_str).header("Content-Type", "application/json")
        .send().await.unwrap();
    let res_text = res.text().await.unwrap();
    // get all response
    let resp_content: QuestionResponse = serde_json::from_str(&res_text).unwrap();
    // remove html
    let content_fix = build_desc(&resp_content.data.question.translated_content);
    // create file name
    let file_name = format!(
        "s{}_{}", 
        build_question_index(resp_content.data.question.question_id), 
        build_index_name(resp_content.data.question.title_slug.clone())
    );
    // append new mod to mod.rs
    let mod_path = "./src/solution/mod.rs";
    let mut mod_file = read(&mod_path).await.unwrap();
    let _ = writeln!(&mut mod_file, "mod {};", &file_name).unwrap();
    write(&mod_path, mod_file).await.unwrap();
    // create new question file
    // read layout
    let mut output = read_to_string("layout.tmpl").await.unwrap();
    // insert question description
    output = output.replace("__QUESTION_DESCRIPTION__", &content_fix);
    // insert question link
    let link = format!("https://leetcode.cn/problems/{}/", &resp_content.data.question.title_slug);
    output = output.replace("__QUESTION_LINK__", &link);
    // insert code template
    let mut code = "".to_string();
    for tmpl in resp_content.data.question.code_snippets.iter() {
        if tmpl.lang == "Rust".to_string() {
            code = tmpl.code.clone();
        }
    }
    output = output.replace("__CODE__", &code);
    // insert test fn name
    let fn_name = build_index_name(resp_content.data.question.title_slug.clone());
    output = output.replace("__TEST_FN_NAME__", &fn_name);
    // insert test cases
    output = output.replace("__TEST_CASE__", &resp_content.data.question.sample_test_case);
    // write to file
    let _ = write(format!("./src/solution/{}.rs", file_name), output).await.unwrap();
}

async fn get_question_stats_by_question_id(id: i64) -> Option<Stat> {
    let client = reqwest::Client::new();
    // get all question
    let resp = client.get(QUESTIONS_URL).send().await.unwrap();
    let resp_text = resp.text().await.unwrap();
    let resp_content: AllQuestionsResponse = serde_json::from_str(&resp_text).unwrap();
    for v in resp_content.stat_status_pairs {
        if v.stat.question_id == id {
            return Some(v.stat.clone())
        }
    }
    None   
}

// leetcode maximum number of digits is 4，fix it.
fn build_question_index(index: String) -> String {
    let mut copy = index.clone();
    while copy.len() < 4 {
        copy = "0".to_string() + &copy;
    }
    return copy
}

// slug to file name
fn build_index_name(name: String) -> String {
    return name.replace("-", "_")
}

// https://github.com/aylei/leetcode-rust/blob/master/src/main.rs
fn build_desc(content: &str) -> String {
    // TODO: fix this shit
    content
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct QuestionRequest {
    operation_name: String,
    query: String,
    variables: GraphqlVariables
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphqlVariables {
    title_slug: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResponse {
    pub data: QuestionResponseData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResponseData {
    pub question: Question,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
    pub title_slug: String,
    pub content: String,
    pub translated_title: String,
    pub translated_content: String,
    pub code_snippets: Vec<CodeSnippets>,
    pub sample_test_case: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippets {
    code: String,
    lang: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllQuestionsResponse {
    #[serde(rename = "stat_status_pairs")]
    pub stat_status_pairs: Vec<StatStatusPair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatStatusPair {
    pub stat: Stat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "question_id")]
    pub question_id: i64,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub level: i64,
}