use dotenv::dotenv;
use reqwest::Client;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

pub type ResponseType = Vec<Root>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub viewer: Viewer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub pull_requests: PullRequests,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequests {
    pub total_count: i64,
    pub nodes: Vec<Node>,
    pub page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub title: String,
    pub permalink: String,
    pub reviews: Reviews,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reviews {
    pub nodes: Vec<Node2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    pub body: String,
    pub comments: Comments,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    pub total_count: i64,
    pub nodes: Vec<Node3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node3 {
    pub author: Author,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
}

#[derive(Serialize)]
pub struct PR {
    title: String,
    url: String,
}

#[macro_use]
extern crate rocket;

fn get_client() -> Client {
    let mut client: Option<Client> = None;
    if client.is_none() {
        client = Some(Client::new());
        return client.unwrap();
    } else {
        return client.unwrap();
    }
}

static QUERY2: &str = "query {viewer {      pullRequests(last: 100, states: OPEN) {        totalCount        nodes {          title          permalink          reviews(first:50, states: CHANGES_REQUESTED){            nodes{              body              comments(first:30 ){                totalCount                nodes{                  author{                    login                  }                  body                }              }            }          }        }        pageInfo {          hasNextPage        }      }    }  }";

#[get("/")]
async fn index() -> Result<String, String> {
    let client = get_client();
    let pat = std::env::var("GITHUB_PAT").unwrap();
    let res = client
        .post("https://api.github.com/graphql")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {pat}"))
        .header(reqwest::header::USER_AGENT, "Combustiblemon")
        .body(format!("{{\n \"query\": \"{QUERY2}\"\n}}"))
        .send()
        .await;
    // println!("{}", format!("{{ \"query\": \"{QUERY}\"}}"));
    match res {
        Ok(res) => match res.text().await {
            Err(err) => Err(err.to_string()),
            Ok(res2) => {
                match serde_json::from_str::<Root>(&res2) {
                    Ok(data) => {
                        // res3.data.viewer.pull_requests.total_count
                        let total_count = data.data.viewer.pull_requests.total_count;
                        let mut i2: Vec<PR> = vec![];

                        for item in data.data.viewer.pull_requests.nodes {
                            i2.push(PR {
                                title: item.title,
                                url: item.permalink,
                            })
                        }

                        Ok(json!({
                           "data": {
                               "wip": i2
                           },
                           "metadata": {
                            "display": {
                                "wip": {
                                    "priority": 11,
                                    "symbol": "🚧",
                                    "title": "🚧 Work in progress"
                                },
                            }
                           }
                        })
                        .to_string())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
        },
        Err(err) => Err(err.to_string()),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index])
}