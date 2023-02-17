use dotenv::dotenv;
use load_dotenv::load_dotenv;
use reqwest::Client;
mod r#struct;
use serde_derive::Serialize;
use serde_json::json;

load_dotenv!();

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct PR {
    title: String,
    url: String,
}

fn get_client() -> Client {
    let mut client: Option<Client> = None;
    if client.is_none() {
        client = Some(Client::new());
        return client.unwrap();
    } else {
        return client.unwrap();
    }
}

static QUERY2: &str = "{ \
    viewer { \
      pullRequests(last: 100, states: OPEN) { \
        totalCount \
        nodes { \
          title \
          permalink \
          repository { \
            nameWithOwner \
          } \
          reviews(first: 50, states: CHANGES_REQUESTED) { \
            nodes { \
              body \
              comments(first: 30) { \
                totalCount \
                nodes { \
                  author { \
                    login \
                  } \
                  body \
                } \
              } \
            } \
          } \
        } \
        pageInfo { \
          hasNextPage \
        } \
      } \
    } \
  }";

#[get("/")]
async fn index() -> Result<String, String> {
    let client = get_client();
    let pat = env!("GITHUB_PAT");
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
                match serde_json::from_str::<r#struct::Root>(&res2) {
                    Ok(data) => {
                        // res3.data.viewer.pull_requests.total_count
                        let total_count = data.data.viewer.pull_requests.total_count;
                        let mut i2: Vec<PR> = vec![];

                        for item in data.data.viewer.pull_requests.nodes {
                            i2.push(PR {
                                title: format!(
                                    "{} - {}",
                                    item.repository.name_with_owner, item.title,
                                ),
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
                                    "symbol": "🚧 ",
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
