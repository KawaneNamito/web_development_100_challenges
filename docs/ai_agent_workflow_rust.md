# AI Agent Standard Development Workflow

This document outlines the standard workflow for implementing features in the "Web 開発百本ノック" project (Rust Backend). AI agents should follow this process to ensure quality, consistency, and maintainability.

## 1. Preparation & Planning phase
Before writing any code, establish a clear plan.

- **Requirement Analysis**: Understand the user's request thoroughly.
- **Implementation Plan Creation**:
    - Create a markdown file: `docs/plan_request_XX.md`.
    - **Checklist Strategy**: Define tasks in the following strict order:
        1.  **Database Definition**: Table changes, Migrations.
        2.  **API Specification**: OpenAI changes.
        3.  **Implementation**: Code generation, Repository, Handler.
        4.  **Verification**: Static Analysis (`cargo check`, `grep`).
        5.  **Testing**: Unit/Integration tests.
        6.  **Final Review**.

## 2. Schema Definition Phase (The Source of Truth)
Define schemas first. Code should follow schema, not vice versa.

### 2.1. Database
- **Document**: Update `docs/table.md` with new columns/tables.
- **Migration**: Create SQL migration files in `infra/database/migration`.
    - Naming convention: `VVV_description.sql` (e.g., `002_add_category.sql`).

### 2.2. API Specification (OpenAPI)
- **Document**: Update `docs/api.yml`.
- **Versioning**: Explicitly update `info.version` and API paths (e.g., `/api/v2/...`) if changes are breaking.
- **[CRITICAL] Best Practice: Component Extraction**:
    - **Do not** define request/response bodies inline.
    - **Do** define them in `components/schemas`.
    - **Reason**: Inline schemas generate unstable names like `ApiV2StreamsPostRequest`. Managed components generate clean names like `CreateStreamRequest`.
    - **Example**:
        ```yaml
        requestBody:
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/CreateStreamRequest"
        ```

## 3. Implementation Phase (Rust)

### 3.1. Code Generation
- Execute OpenAPI Generator from `backend-rust` directory.
- Ensure `Cargo.toml` depends on the generated crate (e.g., `openapi_types`).
- **Schema Export**: Re-export generated types in `src/schema/mod.rs` to abstract the generation layer.

### 3.2. Coding Order
1.  **Domain Models** (`src/model/`): Define internal Rust structs matching the DB schema.
2.  **Repository** (`src/repository/`): Implement data access logic.
    - Signature: `async fn method(&self, ...) -> Result<...>`
3.  **Handler** (`src/handler/`): Implement API logic.
    - Import clean types (`CreateStreamRequest`) from `src/schema`.
    - Map internal models to API responses.

## 4. Verification Phase
- **Static Analysis**: Run `cargo check` frequently.
- **Legacy Cleanup**: If versioning up (v1 -> v2), run `grep` to ensure no v1 types (`ApiV1...`) remain in the codebase.

## 5. Testing Phase
Implement tests immediately after coding.

- **Co-located Tests**: Write unit tests in the same file using `#[cfg(test)] mod tests`.
- **Repository Tests**:
    - Use embedded tests to verify SQL logic.
- **Handler Tests**:
    - Use **Mock Repository** (manual mock struct implementing the trait) to test validation and logic without DB dependency.
    - Verify status codes and error messages.

## 6. Final Review
- Confirm all checklist items in `docs/plan_request_XX.md` are checked.
- Notify the user with a summary of changes.
