//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc

/// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct BlogPost {
    id: u32,
    title: String,
}

fn main() -> anyhow::Result<()> {
    let post: BlogPost = {
        let data = fetch_data();
        serde_json::from_str(&data)?
    };
    println!("deserialized = {:?}", post);

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {:?}", post_json);

    Ok(())
}
