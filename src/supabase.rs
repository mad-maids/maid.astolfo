use postgrest::Postgrest;

struct Client ();

pub const CLIENT: Postgrest = Postgrest::new("https://your.postgrest.endpoint");

pub async fn get() -> String {
    let resp = CLIENT
        .from("your_table")
        .select("*")
        .execute()
        .await;

    let body = resp
        .text()
        .await?;
    return body;
}
