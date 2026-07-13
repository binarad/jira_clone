# App Architecture

## Sub-project Core (Foundation)
- Goal: keep business logic and type defenitions separated and isolated from main projects.
- Contains: 
    1. structs(e.g. Project, Issue, User)
    2. enums(e.g. IssueStatus like ToDo, InProgress, Done)
    3. validation logic(e.g. a function that checks if an issue title is too long).
- Shouldn't contains:
    1. sqlx queries. 
    2. database pools.
    3. UI widgets.

## Sub-project DB (Database Operations)
- Goal: entirely responsible for database operations.
- Contains: 
    1. sqlx macros.
    2. async functions that take a PgPool and return struct from the core sub-project.
    3. custom DbError using thiserror.
- Shouldn't contains:
    1. GUI related things.
    2. loading .env file.

## Sub-projct GUI (User Interface)
- Goal: to draw user interface.
- Contains: 
    1. loading .env file and handling errors with anyhow.
    2. ui widgets.
