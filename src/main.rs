use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
{
    "article": "Amptudix",
    "author": "Ashan",
    "paragraph": [
{
    "name": "AI/ML Platform"
    },
{
    "name":"Created by Ashan Niwantha"
    },
{
    "name": "Rate and track your music"
    }
    ]
    }
    "#;

    let parsed: Article = read_json_types(json);

    println!(
        "\n\n The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}

fn read_json_types(raw_json: &str) -> Article {
    serde_json::from_str(raw_json).unwrap()
}
