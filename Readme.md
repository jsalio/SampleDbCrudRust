# Firebase Rust Integration

This project demonstrates how to interact with Firebase Realtime Database using Rust. It includes features such as retrieving and updating data, handling multiple records, and dynamically loading Firebase configuration from a JSON file.

## Features

- Firebase Realtime Database Integration: Connect to and query Firebase.
- Asynchronous Operations: Fully asynchronous interaction using tokio.
- Dynamic Configuration: Load Firebase configuration from an external JSON file.
- CRUD Operations:
   - List all records in a database node.
   - Retrieve a single record by key.
   - Add or update records.
   
## Requirements

   - Rust: Version 1.70 or later
   - Firebase Realtime Database: An active Firebase project
   - Dependencies:
      -  firebase-rs for Firebase interactions
      -  serde for JSON handling
      -  tokio for async runtime
      
## Project structure

	.
	├── src
	│   ├── main.rs             # Main application logic
	│   ├── firebase_config.json # Firebase configuration file
	│   ├── models.rs           # Definitions of data structures (e.g., `CardInfo`)
	│   └── database.rs         # Abstraction for database operations
	├── Cargo.toml              # Project dependencies
	├── README.md               # Project documentation

## Configuration

### Firebase Setup

   - Set up a Firebase Realtime Database in your Firebase project.
   - Enable authentication and database rules as needed.
   - Copy your Firebase credentials (e.g., API key, database URL).
   
## Configuration File

Create a firebase_config.json file with your Firebase configuration:

	{
    	"api_key": "<YOUR_API_KEY>",
    	"auth_domain": "<YOUR_AUTH_DOMAIN>",
   	"database_url": "<YOUR_DATABASE_URL>",
    	"project_id": "<YOUR_PROJECT_ID>",
    	"storage_bucket": "<YOUR_STORAGE_BUCKET>",
    	"messaging_sender_id": "<YOUR_MESSAGING_SENDER_ID>",
    	"app_id": "<YOUR_APP_ID>"
	}

Place the file in the root directory of your project.

## Usage
Running the Project

    Clone the repository:
	git clone <repository_url>
	cd <repository_name>

Install dependencies:

	cargo build

Run the application:

    cargo run

### Example Outputs
List All Records

	Record fetched:
	id                                      client
	<uuid>                                  John Doe
	<uuid>                                  Jane Smith

Fetch a Single Record

	Record fetched:
	Key: example_key, Value: CardInfo { ... }

## Example Code

Models

	#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct CardInfo {
    	pub client_name: String,
    	pub configuration: Vec<Configuration>,
    	pub creation_date: String,
    	pub creator_email: String,
    	pub creator_name: String,
    	pub creator_phone: String,
    	pub current_document_in_process: i64,
    	pub expiry_date: String,
    	pub process_document_requirement: i64,
    	pub process_id: String,
    	pub process_name: String,
    	pub status: String,
	}

	#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Configuration {
    	pub handle: Option<i64>,
    	pub id: i64,
    	pub is_required: bool,
    	pub is_template: bool,
    	pub name: String,
    	pub status: String,
	}

## Database Interactions

Retrieve all records:

	fn list_records(
    	&self,
	) -> Pin<Box<dyn Future<Output = Result<HashMap<String, T>, RequestError>> + 	'_>>;

Retrieve a single record:

	fn get_record(
	    &self,
	    key: &str,
	) -> Pin<Box<dyn Future<Output = Result<Option<T>, RequestError>> + '_>>;

## Future Enhancements

   - Authentication: Integrate Firebase authentication for secure access.
   - Error Handling: Improve error handling with custom error types.
   - Unit Tests: Add comprehensive tests for database operations.

## License

This project is open-source under the MIT License.
Contributions

Contributions are welcome! Feel free to open issues or submit pull requests to enhance the functionality.