use regex::Regex;
use reqwest::{blocking::Client, header::{HeaderValue, HeaderMap}};
use rayon::prelude::*;
use anyhow::{anyhow, Ok, Result};
use tracing::Level;
use std::{fs::{self, File}, thread, time::Duration};
use std::sync::Arc;
use async_openai::{types::{ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPart, ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs, Role}, Client as OpenAI};
use tokio::runtime::Runtime;
use std::io::Write;
use html2md::parse_html;
use clap::Parser;

const COOKIE: &str = "gr_user_id=157c50e8-f2b1-4e4c-aea2-1ffcff634ca3; _bl_uid=Uwlvspka9k5eehsI55yysgtzd0hg; a2873925c34ecbd2_gr_last_sent_cs1=riddler2020; _ga_PDVPZYN3CW=deleted; Hm_lvt_fa218a3ff7179639febdb15e372f411c=1702869219,1703257878; a2873925c34ecbd2_gr_session_id=5307cf0e-09db-4d1d-8509-fface3817274; a2873925c34ecbd2_gr_last_sent_sid_with_cs1=5307cf0e-09db-4d1d-8509-fface3817274; a2873925c34ecbd2_gr_session_id_sent_vst=5307cf0e-09db-4d1d-8509-fface3817274; Hm_lvt_f0faad39bcf8471e3ab3ef70125152c3=1708784837; _gid=GA1.2.242272118.1708784838; csrftoken=xBZxYU6Z1CietkqqidQrLTWJBRl7BBGjV5YTeL76pQiAxdV0NhOIg99tS0v0nReq; tfstk=eMHej7GkV80sM0sjT52z_dLCAdyLF-LjKYa7q0muAy4nPHBzq40tFXikJbuZP4HQRki7qz4iSygIO0gNvAuVF9Tp90XzF8YXlK9jvDw8EEZqzmi-vwseBYpXhDn3QPBrSKtphvffcPahEN_8sE0gSARfCxqlplaNEtSYAWXrY_6ReFr3OYVg7TmEQkPUoDjrxaEGyMH-TafztlEalh-wBTK3j4qi1eCRw5RTbrtvD_C8trZalh-Nw_FaQlzXDnC..; LEETCODE_SESSION=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiMzQwODgiLCJfYXV0aF91c2VyX2JhY2tlbmQiOiJhbGxhdXRoLmFjY291bnQuYXV0aF9iYWNrZW5kcy5BdXRoZW50aWNhdGlvbkJhY2tlbmQiLCJfYXV0aF91c2VyX2hhc2giOiJjMzc1OGIxYjIyNWU3NTZiYmEzNDRkNjAxMDMyZWRmMzYyZTQwMGU3NzdkZjE0MmZiYzRhYTE5MmJhNzc5NDhmIiwiaWQiOjM0MDg4LCJlbWFpbCI6Imx1eWEud2FuZ0BxcS5jb20iLCJ1c2VybmFtZSI6IlJpZGRsZXIyMDIwIiwidXNlcl9zbHVnIjoicmlkZGxlcjIwMjAiLCJhdmF0YXIiOiJodHRwczovL2Fzc2V0cy5sZWV0Y29kZS5jbi9hbGl5dW4tbGMtdXBsb2FkL3VzZXJzL3JpZGRsZXIyMDIwL2F2YXRhcl8xNTgzMjE3NTA1LnBuZyIsInBob25lX3ZlcmlmaWVkIjp0cnVlLCJfdGltZXN0YW1wIjoxNzA4Nzg0ODUyLjc0NTEwNzcsImV4cGlyZWRfdGltZV8iOjE3MTEzMDY4MDAsInZlcnNpb25fa2V5XyI6Mn0.cacWj3WWXu2Nl9C8x5Nu-0H1AS0j61jF6M9TDCd-jlg; a2873925c34ecbd2_gr_cs1=riddler2020; Hm_lpvt_f0faad39bcf8471e3ab3ef70125152c3=1708784854; _ga=GA1.1.1324949997.1705844335; _ga_PDVPZYN3CW=GS1.1.1708784838.34.1.1708784910.55.0.0";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.152 Safari/537.36";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    start: u64,

    #[arg(short, long)]
    end: u64,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let args = Cli::parse();

    let mut header = HeaderMap::new();
    header.insert("Connection", HeaderValue::from_static("keep-alive"));
    header.insert("Content-Type", HeaderValue::from_static("application/json"));
    header.insert("User-Agent", HeaderValue::from_str(USER_AGENT).unwrap());
    header.insert("cookie", HeaderValue::from_str(COOKIE).unwrap());

    // 创建一个 HTTP 客户端
    let client = Arc::new(Client::builder()
        .default_headers(header)
        .build()?
    );

    let problem_list = client.get("https://leetcode.cn/api/problems/all/")
        .send()?
        .text()?;
    // println!("Problem list: {}", problem_list);
    let problem_list: serde_json::Value = serde_json::from_str(&problem_list)?;
    let problem_list = problem_list["stat_status_pairs"]
        .as_array()
        .ok_or(anyhow!("Not fount problem list"))?;
    tracing::info!("Problem list: {}", problem_list.len());
    // craw_problem(client.clone(), 1, "two-sum")?;
    problem_list.par_iter()
        .filter(|problem| !problem["paid_only"].as_bool().unwrap_or(false))
        .for_each(|problem| {
            let id = problem["stat"]["question_id"].as_u64().unwrap();
            if args.start > id || id > args.end {return;}
            let title = problem["stat"]["question__title_slug"].as_str().unwrap();
            tracing::info!("Problem: {} {}", id, title);
            craw_problem(client.clone(), id as usize, title).unwrap();
            thread::sleep(Duration::from_secs(5));
        });
 
    Ok(())
}

fn craw_problem(client: Arc<Client>, id: usize, title: &str) -> Result<()> {
    let problem_detail = get_problem_detail(client.clone(), title)?;
    let problem_code = get_problem_code(client.clone(), title)?;
    let problem_test = get_test_code(&problem_detail, &problem_code)?;
    
    let content = format!("#![allow(unused)]\nuse crate::Solution;\n\n{}\n\n{}\n\n{}", problem_detail, problem_code, problem_test);

    fs::write(format!("problem/src/en_US/id_{}_{}.rs", id, title.replace('-', "_")), content)?;

    scan_directory("problem/src/en_US")?;
    Ok(())
}

fn get_problem_detail(client: Arc<Client>, title: &str) -> Result<String> {
    // Get problem detail
    let params = format!(
        r#"{{"query":"\n    query questionContent($titleSlug: String!) {{\n  question(titleSlug: $titleSlug) {{\n    content\n    editorType\n    mysqlSchemas\n    dataSchemas\n  }}\n}}\n    ","variables":{{"titleSlug":"{}"}},"operationName":"questionContent"}}"#,
       title 
    );

    let question_detail = client.post("https://leetcode.cn/graphql/")
            .body(params)
            .send()?
            .text()?;
    tracing::info!("Get Problem {} detail", title);
    tracing::debug!("Problem {} detail: {}", title, question_detail);
    let question_detail: serde_json::Value = serde_json::from_str(&question_detail)?;
    let question_detail = &question_detail["data"]["question"]["content"];
    let question_detail_md = parse_html(question_detail.as_str().ok_or(anyhow!(
        "{} content cannot convert to string",
        title
    ))?);
    let question_detail_md = question_detail_md.lines()
        .map(|line| format!("// {}", line))
        .collect::<Vec<_>>()
        .join("\n");
    tracing::info!("Parse problem {} detail to markdown", title);
    Ok(question_detail_md)
}

fn get_problem_code(client: Arc<Client>, title: &str) -> Result<String> {
    let params = format!(
        r#"{{"query":"\n    query questionEditorData($titleSlug: String!) {{\n  question(titleSlug: $titleSlug) {{\n    questionId\n    questionFrontendId\n    codeSnippets {{\n      lang\n      langSlug\n      code\n    }}\n    envInfo\n    enableRunCode\n    hasFrontendPreview\n    frontendPreviews\n  }}\n}}\n    ","variables":{{"titleSlug":"{}"}},"operationName":"questionEditorData"}}"#,
        title
    );

    let question_code = client.post("https://leetcode.cn/graphql/")
        .body(params)
        .send()?
        .text()?;
    let question_code: serde_json::Value = serde_json::from_str(&question_code)?;
    let question_code = &question_code["data"]["question"]["codeSnippets"]
        .as_array()
        .ok_or(anyhow!("{} codeSnippets cannot convert to array", title))?
        .iter()
        .find(|code_snippet| code_snippet["langSlug"].as_str().unwrap_or("other") == "rust")
        .ok_or(anyhow!("probelm {} have not rust codeSnippets now.", title))?;  
    let question_code = &question_code["code"]
        .as_str()
        .ok_or(anyhow!("{} codeSnippets not found", title))?;
    let question_code = question_code
        .trim_matches('"')
        .replace("\n\n", "\n\t\tunimplemented!()\n");
    tracing::info!("Get Problem {} code", title);
    Ok(question_code)
}

fn get_test_code(detail: &str, code: &str) -> Result<String> {
    let client = OpenAI::new();
    let prompt_message = ChatCompletionRequestUserMessageContent::Array(vec![
        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText { r#type: "text".to_string(), text: "I will provide you with a piece of Rust code with comments describing the data for test cases. You need to write unit test code in Rust based on the comments and return the results in the specified format. ".to_string() } ),
        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText { r#type: "text".to_string(), text: r"An example is provided for reference: 
        // Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

        // Example 1:
        // Input: nums = [2,7,11,15], target = 9
        // Output: [0,1]
        
        impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
                unimplemented!();
            }
        }
        
        #[cfg(test)]
        mod tests {
        }".to_string() } ),
        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText { r#type: "text".to_string(), text: "Now, let's proceed to the formal question. You need to provide accurate answers and return the results following the specified output format.".to_string() } ),
        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText { r#type: "text".to_string(), text: format!("{}\n{}\n The test code is:", detail, code) } ),
    ]);
    let query_message = ChatCompletionRequestUserMessageContent::Array(vec![
        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText { r#type: "text".to_string(), text: "Now, let's proceed to the formal question. You need to provide accurate answers and return the results following the specified output format.".to_string() } ),
        ChatCompletionRequestMessageContentPart::Text(ChatCompletionRequestMessageContentPartText { r#type: "text".to_string(), text: format!("{}\n{}\n The test code is:", detail, code) } ),
    ]);
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage { role: Role::System, content: "You are a Rust programmer.".to_string(), ..Default::default() }),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage{ role: Role::User, content: prompt_message, ..Default::default() }),
            ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage { role: Role::Assistant, content: Some(r"The test code is:
            #[cfg(test)]
            mod tests {
                use super::*;
                
                fn test_example_1() {
                    let nums = vec![2, 7, 11, 15];
                    let target = 9;
                    let ans = vec![0, 1];
                    let res = Solution::two_sum(nums, target);
                    assert_eq!(res, ans);
                }
            }
            ".to_string()), ..Default::default() }),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage{ role: Role::User, content: query_message, ..Default::default() }),
        ])
        .build()?;
    
    let rt = Runtime::new()?;
    
    let mut response = rt.block_on( async move {
        client.chat().create(request).await
    })?;
    
    let content = response.choices.swap_remove(0).message.content.ok_or(anyhow!("assistant no message"))?;

    let re = Regex::new(r"(?s)```rust\n(.*?)```").unwrap();
    let captures = re.captures(&content).ok_or(anyhow!("assistant no message"))?;
    let content = captures.get(1).map_or("#[cfg(test)]\nmod tests{\n}".to_string(), |m| m.as_str().to_string());

    Ok(content)
}

fn scan_directory(dir_path: &str) -> Result<()> {
    let mut mod_file = File::create(format!("{}/mod.rs", dir_path))?;
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with("id_") {
            writeln!(mod_file, "mod {};", file_name.trim_end_matches(".rs"))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_openai() -> Result<()> {
        // let api_key = "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"; // This secret could be from a file, or environment variable.
        // let config = OpenAIConfig::new()
        //     .with_api_key(api_key);
        use async_openai::types::{
                ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
                ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        };

        let client = OpenAI::new();

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a helpful assistant.")
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content("Who won the world series in 2020?")
                    .build()?
                    .into(),
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content("The Los Angeles Dodgers won the World Series in 2020.")
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content("Where was it played?")
                    .build()?
                    .into(),
            ])
            .build()?;

        println!("{}", serde_json::to_string(&request).unwrap());

        let response = client.chat().create(request).await?;

        println!("\nResponse:\n");
        for choice in response.choices {
            println!(
                "{}: Role: {}  Content: {:?}",
                choice.index, choice.message.role, choice.message.content
            );
        }
        Ok(())
    }
}
