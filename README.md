# Proxum API

A high-performance REST API built with the Axum framework and Protocol Buffers. This service uses binary serialization for data transfer and SQLx for type-safe database interactions.

## Running the Project

The project can be executed in two different modes.

### Option 1: Docker Compose
If you are running the project for the first time or have modified the code, you must build the image:

1. Build the API image:
   docker build -t proxum:latest .

2. Start the stack:
   docker-compose up -d

### Option 2: Local Development (Cargo)
Use this mode to run the API locally while keeping the database in a container.

1. Start the Database:
   docker-compose up -d <database_service_name>

2. Prerequisites:
   Ensure the Protobuf compiler (protoc) is installed on your system to allow prost to compile your .proto files.

3. Run the API:
   cargo run

## Build Configuration

### SQLx Offline Mode
The sqlx::query! macro requires a database connection at compile time. To build the project in environments without a live database (like inside a Docker build stage), we use SQLx's offline mode:

1. Generate the metadata file: cargo sqlx prepare
2. The Dockerfile is configured with ENV SQLX_OFFLINE=true to utilize the resulting sqlx-data.json.

### Protobuf Setup
The build process automatically triggers prost-build. The Dockerfile includes protobuf-compiler to handle this generation during the release build.

## Testing the API

Since the API uses binary Protobuf payloads, use curl to send pre-compiled binary files.

### 1. Create a Test Binary
You can convert a Hex string representing a CreateUserRequest into a binary file:

echo "0a076a6f686e646f6512146a6f686e2e646f65406578616d706c652e636f6d" | xxd -r -p > user.bin

### 2. Dispatch Request

curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/x-protobuf" \
  --data-binary @user.bin
