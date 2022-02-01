use postgrest::Postgrest;
use std::error;

pub struct Dungeon {
    pub client: Postgrest,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            client: Postgrest::new(dotenv::var("SUPABASE_URL").unwrap())
                .insert_header("apikey", dotenv::var("SUPABASE_KEY").unwrap()),
        }
    }

    pub async fn get_groups(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self.client.from("Groups").select("*").execute().await?;
        println!("{}", resp.text().await?);
        Ok(())
    }
}
