use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
// async fn create_surrealdb(
//     surql_file_path: &str,
//     url: &str,
//     namespace: &str,
//     database: &str
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Create the .surql file
//     let mut file = File::create("./")?;

//     // Write the connection details to the file
//     writeln!(file, "url: {}", url)?;
//     writeln!(file, "test: {}", namespace)?;
//     writeln!(file, "test: {}", database)?;

//     // Write a sample SurrealQL query to the file (optional)
//     writeln!(file, "CREATE table:users")?;

//     Ok(())
// }

// async fn connect_to_surrealdb(
//     surql_file_path: &str
// ) -> Result<Surreal<Ws>, Box<dyn std::error::Error>> {
//     // Read the .surql file
//     let mut file = File::open(surql_file_path)?;
//     let mut surql_content = String::new();
//     file.read_to_string(&mut surql_content)?;

//     // Parse the .surql file
//     let parsed_content = parse_surql(&surql_content)?;

//     // Extract connection details from the parsed .surql file
//     let url = parsed_content.get("url").ok_or("Missing 'url' in .surql file")?.to_string();
//     let namespace = parsed_content
//         .get("namespace")
//         .ok_or("Missing 'namespace' in .surql file")?
//         .to_string();
//     let database = parsed_content
//         .get("database")
//         .ok_or("Missing 'database' in .surql file")?
//         .to_string();

//     // Connect to SurrealDB
//     let db = Surreal::new::<Ws>(url).await?.use_ns(namespace).await?.use_db(database).await?;

//     Ok(db)
// }

// fn parse_surql(surql_content: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
//     let mut parsed_content = HashMap::new();
//     for line in surql_content.lines() {
//         let parts: Vec<&str> = line.split(":").collect();
//         if parts.len() == 2 {
//             let key = parts[0].trim().to_string();
//             let value = parts[1].trim().to_string();
//             parsed_content.insert(key, value);
//         }
//     }
//     Ok(parsed_content)
// }
pub static DB: Lazy<Surreal<Client>> = Lazy::new(|| Surreal::init());
