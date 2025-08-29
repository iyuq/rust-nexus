# Rust Nexus - Hexagonal Architecture Web API

A production-ready HTTP web server built with Rust, featuring hexagonal architecture and domain-driven design for user management.

## Features

- **Hexagonal Architecture**: Clean, maintainable code structure with clear boundaries
- **Domain-Driven Design**: Rich domain models with business logic encapsulation
- **Axum Framework**: High-performance async web framework
- **PostgreSQL Integration**: Robust database with SQLx for type-safe queries
- **CRUD Operations**: Complete user management API with domain validation
- **Error Handling**: Comprehensive error handling with proper HTTP status codes
- **Logging**: Structured logging with tracing
- **CORS Support**: Cross-origin resource sharing enabled
- **Database Migrations**: Automated database schema management
- **Input Validation**: Domain-level validation with value objects
- **Health Checks**: Health endpoint for monitoring
- **Dependency Injection**: Clean dependency management and inversion of control

## Tech Stack

- **Rust** - Systems programming language
- **Axum** - Web application framework
- **Tokio** - Asynchronous runtime
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Database
- **UUID** - Unique identifiers
- **Serde** - Serialization framework
- **Tracing** - Structured logging

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database server
- Environment variables configured

## Setup

1. **Clone the repository**:
   ```bash
   git clone <your-repo-url>
   cd rust-nexus
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Setup PostgreSQL**:
   - Install and start PostgreSQL
   - Create a database user and database
   - Update the `.env` file with your database credentials

4. **Configure environment variables**:
   ```env
   DATABASE_URL=postgresql://username:password@localhost/rust_nexus_db
   RUST_LOG=debug
   PORT=3000
   ```

5. **Run the application**:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000`

## API Endpoints

### Health Check
- `GET /health` - Check server health

### Users CRUD Operations

#### Create User
- **POST** `/api/users`
- **Body**:
  ```json
  {
    "name": "John Doe",
    "email": "john.doe@example.com"
  }
  ```
- **Response**: `201 Created`
  ```json
  {
    "success": true,
    "data": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "John Doe",
      "email": "john.doe@example.com",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  }
  ```

#### Get All Users
- **GET** `/api/users`
- **Response**: `200 OK`
  ```json
  {
    "success": true,
    "data": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "John Doe",
        "email": "john.doe@example.com",
        "created_at": "2024-01-01T12:00:00Z",
        "updated_at": "2024-01-01T12:00:00Z"
      }
    ]
  }
  ```

#### Get User by ID
- **GET** `/api/users/{id}`
- **Response**: `200 OK` or `404 Not Found`

#### Update User
- **PUT** `/api/users/{id}`
- **Body**:
  ```json
  {
    "name": "Jane Doe",
    "email": "jane.doe@example.com"
  }
  ```
- **Response**: `200 OK` or `404 Not Found`

#### Delete User
- **DELETE** `/api/users/{id}`
- **Response**: `204 No Content` or `404 Not Found`

## Error Responses

All error responses follow this format:
```json
{
  "success": false,
  "data": null,
  "message": "Error description"
}
```

Common HTTP status codes:
- `200 OK` - Success
- `201 Created` - Resource created
- `204 No Content` - Success with no response body
- `400 Bad Request` - Invalid request data
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists (e.g., duplicate email)
- `500 Internal Server Error` - Server error

## Project Structure

```
src/
├── main.rs                     # Application entry point and dependency injection
├── database.rs                 # Database setup and configuration
├── domain/                     # Core business logic (Domain Layer)
│   ├── entities/              # Domain entities with business rules
│   │   └── user.rs           # User entity and value objects
│   ├── services/             # Domain services for complex business logic
│   │   └── user_service.rs   # User domain service
│   └── ports/                # Interfaces for external dependencies
│       └── user_repository_port.rs  # Repository interface
├── application/               # Use cases and orchestration (Application Layer)
│   ├── dto/                  # Data transfer objects
│   │   └── user_dto.rs       # Request/response DTOs
│   └── services/             # Application services
│       └── user_app_service.rs  # User use case orchestration
└── infrastructure/           # External adapters (Infrastructure Layer)
    ├── database/            # Database implementations
    │   └── postgres_user_repository.rs  # PostgreSQL adapter
    └── web/                 # HTTP interface
        ├── handlers.rs      # HTTP request handlers
        └── routes.rs        # Route definitions
migrations/
└── 001_create_users_table.sql  # Database migrations
tests/
└── integration_tests.rs        # Integration tests
```

## Architecture

This project follows **Hexagonal Architecture** (also known as Clean Architecture) with **Domain-Driven Design** principles:

### Domain Layer (Core)
- **Entities**: Rich domain objects with business logic and validation
- **Value Objects**: Immutable objects that represent domain concepts
- **Domain Services**: Business logic that doesn't naturally fit in entities
- **Ports**: Interfaces that define contracts for external dependencies

### Application Layer
- **Application Services**: Orchestrate use cases and coordinate domain services
- **DTOs**: Data transfer objects for API communication
- **Use Cases**: Specific application workflows

### Infrastructure Layer (External)
- **Database Adapters**: Concrete implementations of repository interfaces
- **Web Adapters**: HTTP handlers and routing
- **External Service Adapters**: Third-party service integrations

### Benefits of This Architecture:
1. **Dependency Inversion**: Core business logic doesn't depend on external concerns
2. **Testability**: Easy to unit test business logic in isolation
3. **Maintainability**: Clear separation of concerns and responsibilities
4. **Flexibility**: Easy to swap implementations (e.g., different databases)
5. **Domain Focus**: Business rules are clearly expressed and protected

## Development

### Running Tests

The hexagonal architecture enables different levels of testing:

#### Unit Tests (Domain Layer)
Test business logic in isolation:
```bash
cargo test --test unit_tests
```

#### Integration Tests (Application Layer)
Test use cases with mock adapters:
```bash
cargo test --test integration_tests
```

#### End-to-End Tests
Test complete workflows:
```bash
cargo test
```

#### API Tests
Use the provided HTTP test file:
```bash
# Start the server first
cargo run

# Then run API tests using VS Code REST Client
# or your preferred HTTP client with api_tests.http
```

### Database Operations

#### Create a new migration
```bash
sqlx migrate add <migration_name>
```

#### Run migrations
```bash
sqlx migrate run
```

#### Revert last migration
```bash
sqlx migrate revert
```

### Code Quality

#### Format code
```bash
cargo fmt
```

#### Lint code
```bash
cargo clippy
```

## Best Practices Implemented

1. **Hexagonal Architecture**: Clean separation between domain, application, and infrastructure layers
2. **Domain-Driven Design**: Rich domain entities with business logic and validation
3. **Dependency Inversion**: Core business logic independent of external dependencies
4. **Port/Adapter Pattern**: Clean interfaces for external systems (database, web)
5. **Error Handling**: Comprehensive error handling with proper HTTP status codes
6. **Type Safety**: Using SQLx for compile-time SQL validation
7. **Async/Await**: Fully async implementation with Tokio
8. **Logging**: Structured logging for debugging and monitoring
9. **Input Validation**: Domain-level validation with value objects
10. **Database Migrations**: Version-controlled database schema changes
11. **CORS Support**: Cross-origin resource sharing for web clients
12. **Health Checks**: Monitoring endpoint for service health
13. **Environment Configuration**: Configuration through environment variables
14. **Single Responsibility**: Each layer has a single, well-defined responsibility
15. **Testable Design**: Architecture supports unit, integration, and end-to-end testing

## Production Considerations

### Architecture Benefits for Production:
- **Modularity**: Easy to scale individual components
- **Testability**: Comprehensive testing at all layers
- **Maintainability**: Clear separation of concerns
- **Flexibility**: Easy to swap implementations (database, external services)

### Production Checklist:
- Set `RUST_LOG=info` for production
- Use connection pooling (already implemented)
- Add authentication/authorization middleware to infrastructure layer
- Implement rate limiting in web adapters
- Add API versioning to routes
- Set up monitoring and alerting
- Use HTTPS in production
- Implement caching strategies in infrastructure layer
- Add comprehensive integration tests
- Set up CI/CD pipeline
- Consider splitting into microservices using the same hexagonal structure

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

This project is licensed under the MIT License.
