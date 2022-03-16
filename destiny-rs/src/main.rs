use destiny_rs::codegen;

#[tokio::main]
fn main() {
    let client = reqwest::Client::new();

    let locales = codegen::GetAvailableLocales(&client).unwrap();
}