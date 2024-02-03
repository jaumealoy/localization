# Localization

An average human speaks 1.5 to 2 languages, if your application is not available in your users' language, chances are that you are losing them.

`localization` is a simple API that allows you to manage your translations for any amount of languages. 
It works as follows:

1. You set a default language and write all your texts in that language.
2. Translate all the texts to all the available languages
3. Consume your translation within your app using the API


## Setup
You will need a relational database such as MariaDB or MySQL.

1. Clone the repository and set `api` as your working directory.
2. Run the command `cargo build --release` to create a release version of the API. The executable file should be at `target/release/`.
3. Create a database, import the `database.sql` file and configure `.env` file.
```
DB_HOST=ip or hostname
DB_NAME=database
DB_USER=username
DB_PASS=password
```
4. Run the release executable and you are ready! You may change the API port using the environment variable `PORT`.

## Features
This is a work-in-progress and it is not production-ready. Therefore, you should use it at your own risk and take into account that API and database model might change.

The current features are:

- Retrieve a page of translations
- Retrieve a single translation from a text
- Rust library to consume the REST API, an in-memory cache client is also provided
- User interface to manage languages, pages and translations

### Rust API Client

You may use the Rust API client as follows:
```rust
use localization_client::Localization;

// other imports

let localization_client = localization_client::CachedLocalizationClient::new(
    std::time::Duration::from_sec(300), 
    "http://localhost:3000"
);
```

### Web User Interface

Sample .env file:
```
API_URL=http://localhost:3000
```