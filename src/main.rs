use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let req = QuestionRequest{
        operation_name:"questionData".to_string(), 
        query: "query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n}\n}\n".to_string(), 
        variables: GraphqlVariables { title_slug: "sort-colors".to_string() } 
    };
    let req_str = serde_json::to_string(&req).unwrap();
    let res = client
        .post("https://leetcode.cn/graphql/")
        .body(req_str).header("Content-Type", "application/json")
        .send().await.unwrap();
    let res_text = res.text().await.unwrap();
    let resp_content: QuestionResponse = serde_json::from_str(&res_text).unwrap();
    let content_fix = build_desc(&resp_content.data.question.translated_content);
    println!("{}", content_fix)
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
        .replace("\n", "\n * ")
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
    pub title_slug: String,
    pub content: String,
    pub translated_title: String,
    pub translated_content: String,
}
