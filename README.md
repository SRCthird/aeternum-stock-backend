# Aeternum Stock | Backend

This is Aeternum Stock's backend API designed for efficient inventory management. It is built using Rust with Diesel for ORM, Rocket as the web framework, and MySQL as the database.

## Features

- Inventory tracking and management
- Real-time stock updates
- Secure API endpoints for CRUD operations

## Requirements

- Rust
- Cargo
- MySQL
- Diesel CLI

## Setup

### Clone the repository

```bash
git clone https://github.com/SRCthird/aeternum-stock-backend.git
cd aeternum-stock-backend
```

### Set up MySQL

Ensure MySQL is installed and running on your machine. Create a database for the project:

```sql
CREATE DATABASE aeternum_stock;
```

### Configure Environment Variables

Create a `.env` file in the root directory and update the following information with your database credentials. This project is designed to use the .env file from your current working directory first. If one is not found in your cwd it searches the location of the .exe file. In debug that will be `./target/debug`:

```
DB_CONNECTION=mysql://username:password@localhost/aeternum_stock
```

### Install Dependencies

```bash
cargo build
```

### Run Diesel Migrations

To set up your database schema, run:

```bash
diesel migration run
```

### Launch the Application

```bash
cargo run
```

The API will be available at `http://localhost:5000`.

## API Endpoints

The api endpoints follow this same format for all sections:

| Endpoint             | Method | Description                           |
|----------------------|--------|---------------------------------------|
| `/api/inventory`     | GET    | Retrieves all inventory items.        |
| `/api/inventory/{id}`| GET    | Retrieves a single inventory item.    |
| `/api/inventory`     | POST   | Adds a new inventory item.            |
| `/api/inventory/{id}`| PATCH  | Updates an existing inventory item.   |
| `/api/inventory/{id}`| DELETE | Deletes an existing inventory item.   |

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
