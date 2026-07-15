# Rust Issue Tracker (Iced + sqlx + Postgres) - Master Roadmap

## Reading & Concept
- [ ] "Zero to Production in Rust" for robust architecture patterns.
- [ ] "Domain Modeling Made Functional" for type-driven domain design.
- [x] Study state machines in Rust using enums and compile-time pattern matching.
- [ ] Design Goal: Blazing fast, keyboard-centric, and distraction-free (anti-Jira).

## Phase 0: Preparations & Tooling
- [ ] Set up rust-analyzer in your editor for strict type-checking and macro expansion.
- [ ] Read up on the Elm Architecture (Model-Update-View) used by Iced.
- [ ] Review sqlx documentation on compile-time checked queries.
- [x] Create the docker-compose.yml for local PostgreSQL development.

## Phase 1: Data Layer & Schema (The Foundation)
- [x] Initialize Cargo workspace (core, db, gui crates).
- [x] Configure .env and DATABASE_URL.
- [x] Write SQL Migrations:
- [x] users, projects, and issues tables.
- [x] Implement Database Access (Issues):
- [x] get_project_issues
- [x] create_issue
- [x] update_issue

### Implement Database Access (Projects):
- [ ] Write create_project(pool, name, key) -> Result<i32, DbError>
- [ ] Write get_all_projects(pool) -> Result<Vec<Project>, DbError>
- [ ] Write delete_project(pool, id) -> Result<(), DbError>
- [ ] Write tests for Project CRUD.
- [ ] Implement Database Access (Users):
- [ ] Write create_user(...) and get_all_users(...).
- [ ] Write tests for User CRUD.

## Phase 2: Core State & Bootstrapping (The Engine)
### GUI Setup:
- [ ] Add iced, tokio, and dotenvy to gui/Cargo.toml.
- [ ] Write the main() function to load .env and initialize the PgPool.
- [ ] Define the App State (gui/src/app.rs):
- [ ] Create struct AppState containing the PgPool.
- [ ] Add active_project_id: Option<i32> to state.
- [ ] Create enum ViewState { Loading, ProjectList, Board, Error(String) } and add it to state.
- [ ] Define the Messages (gui/src/message.rs):
- [ ] enum Message { ProjectSelected(i32), ProjectsLoaded(Vec<Project>), IssuesLoaded(Vec<Issue>) }
### Implement iced::Application:
- [ ] Implement new(): Return initial state and a Command to fetch projects.
- [ ] Implement update(): Handle the basic data-loading messages.
- [ ] Implement view(): Just return a basic text("Hello World").into() for now to prove it compiles and runs.

## Phase 3: The Project Sidebar & Navigation
- [ ] Sidebar Widget (gui/src/ui/sidebar.rs):
- [ ] Create a function fn sidebar_view(projects: &[Project], active_id: Option<i32>) -> Element<Message>.
- [ ] Use iced::widget::column to list projects.
- [ ] Wrap each project name in a button that emits Message::ProjectSelected(id).
### Main Layout:
- [ ] Update app.view() to use a row![sidebar_view(...), main_content_view(...)].
### State Wiring:
- [ ] When ProjectSelected hits the update() function, change active_project_id and return a Command to fetch that project's issues.

## Phase 4: The Kanban Board (Visualizing Data)
- [ ] Issue Card Widget (gui/src/ui/card.rs):
- [ ] Create fn issue_card(issue: &Issue) -> Element<Message>.
- [ ] Display the issue_number (e.g., ENG-1) and summary.
- [ ] Add tiny buttons for "Move Left" and "Move Right" (emitting Message::MoveIssue(id, NewStatus)).
- [ ] Column Widget (gui/src/ui/board.rs):
- [ ] Create fn board_column(title: &str, issues: &[&Issue]) -> Element<Message>.
- [ ] Iterate over the filtered issues and push them into an iced::widget::scrollable.
### Board View:
- [ ] Create fn board_view(issues: &[Issue]) -> Element<Message>.
- [ ] Filter issues into three vecs (ToDo, InProgress, Done).
- [ ] Place three board_columns side-by-side in a row!.
### State Wiring:
- [ ] Handle Message::MoveIssue in update() -> Trigger DB update Task -> reload issues.

## Phase 5: Data Entry (Forms & Modals)
- [ ] Form State: Add draft_issue_title: String and draft_issue_desc: String to AppState.
- [ ] Form UI (gui/src/ui/form.rs):
- [ ] Use iced::widget::text_input for the title. (Emits Message::TitleChanged(String)).
- [ ] Use iced::widget::text_editor for the description.
- [ ] Add a "Submit" button emitting Message::SubmitNewIssue.
### Modal Overlay:
- [ ] Create an is_modal_open: bool flag in state.
- [ ] Update the main view() to render the form layered over the board if is_modal_open == true.
### State Wiring:
- [ ] Handle TitleChanged by mutating draft_issue_title.
- [ ] Handle SubmitNewIssue -> Trigger DB Insert Task -> Close Modal -> Reload issues.

## Phase 6: The "Anti-Jira" Differentiators & Polish
- [ ] Command Palette (Ctrl+K): Implement fuzzy-search overlay to quickly switch projects or create issues.
- [ ] Focus Mode: A toggle that hides the sidebar and other columns, centering only the ticket currently "In Progress".
- [ ] Custom Theme: Define a sleek, dark-mode Iced Theme.
- [ ] Release Optimization: Add [profile.release] with lto = true to Cargo.toml.
- [ ] Write a comprehensive README.md.
