# Rust Jira Clone (Iced + sqlx + Postgres) - Master Roadmap

## **Reading**
- [ ] "Zero to Production in Rust" for robust architecture patterns.
- [ ] "Domain Modeling Made Functional" for type-driven domain design.
- [ ] Study state machines in Rust using enums and compile-time pattern matching.

## **Phase 0: Preparations & Tooling**
- [ ] Set up rust-analyzer in your editor for strict type-checking and macro expansion (crucial for `sqlx`).
- [ ] Read up on the Elm Architecture (Model-Update-View) used by Iced.
- [ ] Review `sqlx` documentation on compile-time checked queries.
- [ ] Create the `docker-compose.yml` for local PostgreSQL development.

## **Phase 1: Data Layer & Schema (The Foundation)**
- [ ] Initialize Cargo workspace (`core`, `db`, `gui` crates for clean separation).
- [ ] Configure `.env` and `DATABASE_URL`.
- [ ] **Write SQL Migrations:**
  - [ ] `users` table (id, username, role).
  - [ ] `projects` table (id, key like 'ENG', name).
  - [ ] `issues` table (id, project_id, title, description, status, priority, assignee_id).
- [ ] **Implement Database Access:**
  - [ ] Write `sqlx` functions for fetching a project's issues.
  - [ ] Write `sqlx` functions for creating and updating issues.
- [ ] **Data Testing:** Write `#[tokio::test]` unit tests to verify CRUD operations against a test database before touching the GUI.

## **Phase 2: Core State Logic (The Brain)**
- [ ] Define the `State` struct: needs to hold the `PgPool`, the currently loaded `Project`, and navigation state (e.g., `BoardView`, `SettingsView`).
- [ ] Define the `Message` enum:
  - [ ] UI Events (`CreateIssueClicked`, `SearchInputChanged`).
  - [ ] Async DB Results (`IssuesLoaded(Result<Vec<Issue>, Error>)`, `IssueUpdated`).
- [ ] Implement the `update` function to map `Message` variants to state mutations and background `Task` generation.

## **Phase 3: The Core UI (The Kanban Board)**
- [ ] Create the main application layout (Sidebar for projects, Main area for the board).
- [ ] Build the `Column` widget (To Do, In Progress, Review, Done).
- [ ] Build the `IssueCard` widget displaying title, issue key (e.g., ENG-12), and priority icon.
- [ ] Wire up simple "Move Left/Right" buttons on the cards to test state transitions.
- [ ] Integrate asynchronous `Task` handlers to sync card movements to the database.

## **Phase 4: Interactions & Data Entry (Forms & Modals)**
- [ ] Implement an overlay/modal system in Iced.
- [ ] **Create Issue Form:** Build text inputs for Title and Description, and a dropdown/picker for Priority.
- [ ] **Issue Detail View:** Clicking a card opens the full issue to edit the description or change the assignee.
- [ ] Add basic keyboard shortcuts (e.g., pressing `c` opens the Create Issue modal, `Esc` closes it).

## **Phase 5: Advanced Features (The Polish)**
- [ ] **Search & Filtering:** Add a text input at the top of the board. Update the `view` function to only render cards matching the search string.
- [ ] **Drag and Drop:** Replace the basic "Move" buttons with pointer-based drag-and-drop using `iced_drop`.
- [ ] **Error Handling UI:** Add a toast/notification system so if a database query fails, the user sees a red error banner instead of the app crashing silently.

## **Phase 6: Theming & Deployment (Portfolio Readiness)**
- [ ] Define a custom Iced `Theme` (Dark mode, matching a sleek, minimalist aesthetic).
- [ ] Optimize the release build: Add a `[profile.release]` section to `Cargo.toml` with `lto = true` and `opt-level = 3` for maximum performance.
- [ ] Write a comprehensive `README.md` with screenshots, architecture diagrams, and build instructions.
- [ ] Ensure the Wayland backend for Iced is enabled and scales perfectly.
