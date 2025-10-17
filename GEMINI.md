# Gemini Instructions

This document provides a set of guidelines for interacting with the Gemini AI assistant in this project. Please follow these instructions to ensure a smooth and efficient workflow.

## General Instructions

- **Write Tests:** Always write tests for the code you produce. This ensures that your changes are working as expected and helps to prevent regressions.

## Docker and docker-compose

- **Always work in Docker:** To ensure a consistent development environment, please perform all development tasks within the provided Docker container. This helps to avoid issues related to dependency conflicts and environment inconsistencies.
- **Use `docker-compose`:** Please use `docker-compose` (with a hyphen) instead of `Docker Compose` in all commands and documentation.

## Instructions for Migrations

When creating or modifying database migrations, please adhere to the following guidelines:

- **Modify Existing Tables with Caution:** Before creating a new migration that alters an existing table, always check the current schema to avoid conflicts.

- **Prefer Altering Over Creating:** Whenever possible, prefer altering existing tables to creating new ones. This helps to keep the database schema clean and concise.

- **Consult the Schema:** The source of truth for the database schema is `src/db/schema.rs`. Please review this file carefully before making any changes.

```
src/db/schema.rs
```