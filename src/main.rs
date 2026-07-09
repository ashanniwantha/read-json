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

    let article: Article = Article {
        article: String::from("Who is Ashan Niwantha?"),
        author: String::from("Ashan Niwantha"),
        paragraph: vec![
            Paragraph {
                name: String::from("He loves rust!"),
            },
            Paragraph {
                name: String::from("He loves json!"),
            },
        ],
    };

    let json: String = write_to_json(&article);

    println!("\n JSON: {}", json);
}

fn read_json_types(raw_json: &str) -> Article {
    serde_json::from_str(raw_json).unwrap()
}

fn write_to_json(payload: &Article) -> String {
    serde_json::to_string(&payload).unwrap()
}
