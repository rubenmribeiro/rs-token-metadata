# Ethereum Token Metadata Service

This service fetches ERC-20 token metadata from Moralis API and caches results in Redis.

## Running the Service

There are 3 ways to run this service:

### 1. Using Docker (Recommended)
This is the simplest setup to run locally if you don't need Redis and want to avoid setting up dependencies locally.

1. Install Docker following the [official instructions](https://docs.docker.com/get-docker/)

2. Setup your environment variables by executing `cp .env.template .env` and setting up your Moralis API key

3. Build and run the Docker container:
   ```bash
   # First build the token_metadata image
   docker build -t token_metadata .
   
   # Command structure:
   # docker run -e MORALIS_API_KEY=<your_key> -e REDIS_URL=<redis_url> <image_name> <token_address>
   
   # Example using USDC token address:
   docker run -e MORALIS_API_KEY=your_api_key_here -e REDIS_URL=redis://redis:6379 token_metadata 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
   ```

### 2. Using Podman
This allows you to make use of docker-compose.yaml which sets up your local redis seemlessly.

1. Install Podman following the [official instructions](https://podman.io/getting-started/installation)

2. Setup your environment variables by executing `cp .env.template .env` and setting up your Moralis API key

3. Run the service using podman-compose:
   ```bash
   # Command structure:
   # podman-compose run <service-name> <binary-path> <token-address>
   
   # Example using USDC token address:
   podman-compose run token_metadata ./target/release/token_metadata 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
   ```

### 3. Running Locally

1. Install Rust following the [official instructions](https://www.rust-lang.org/tools/install)

2. Set your environment variables:
   ```bash
   export MORALIS_API_KEY=your_api_key_here
   export REDIS_URL=redis://localhost:6379 # Optional - only if using Redis caching
   ```

3. Run the service:
   ```bash
   # Command structure:
   # cargo run -- <token_address>

   # Example using USDC token address:
   cargo run -- 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48

   ```
