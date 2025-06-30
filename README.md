# Todo App with Solana Integration

This is a Rust-based web application built with the Axum framework, featuring a Todo CRUD API and handlers to interact with a Solana blockchain contract. The app runs without a database, using an in-memory store, and is designed for deployment to a public URL (e.g., via Fly.io) as part of a community project.

## Features
- **CRUD Operations**: Create, Read, Update, and Delete Todo items.
- **Solana Integration**: Includes handlers to read and increment a counter on the Solana blockchain (mocked for now).
- **No Database**: Uses in-memory storage and Solana for state management.

## Prerequisites
- Rust (latest stable version, e.g., 1.82)
- Cargo (Rust's package manager)
- Docker (for deployment)
- Fly.io CLI (`flyctl`) for deployment
- Solana CLI (for blockchain interaction)

## Installation
1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd hello-world
