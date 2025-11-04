# RustLearn

Welcome to RustLearn, an ambitious e-learning platform designed to revolutionize the world of online education. We're here to foster a new era of learning where dedication and achievement are rewarded in the most tangible way – through our own ERC 20 crypto token.

## Motivation

Why should students be rewarded for learning? Simple – motivation matters. In traditional learning environments, the rewards for learning are often abstract; it's not until far in the future that students reap the benefits of their education. RustLearn changes the game by providing immediate, real-world incentives for educational achievements.

Every line of code deciphered, every concept mastered, and every test aced translates into cryptocurrency rewards. This not only gives learners a sense of ownership and accomplishment but also fosters a supportive community where education is valued and celebrated.

With RustLearn, we're not just investing in knowledge; we're investing in our students' futures, one token at a time.

## Configuration

Before running RustLearn, you need to set up your environment. Follow these steps to generate your RSA keys for securing our API and configure your PostgreSQL database.

### Generating RSA keys

Run these commands in your terminal:

```bash
openssl genpkey -algorithm RSA -out private.key -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in private.key -out public.key
```

After you've generated the keys, you'll need to add them to your `.env` file:

```plaintext
PRIVATE_KEY="Paste your private key here"
PUBLIC_KEY="Paste your public key here"
```

### Configuring the Database

Next, configure your PostgreSQL database connection:

```plaintext
DATABASE_URL=postgres://your_username:your_password@db:5432/your_db_name
POSTGRES_DB=your_db_name
POSTGRES_USER=your_username
POSTGRES_PASSWORD=your_password
```

Replace `your_username`, `your_password`, and `your_db_name` with your PostgreSQL credentials and database name.

## Installation and Running

To get RustLearn up and running on your machine, you'll have to set up your environment correctly.

1. (For Windows Users) Install Windows Subsystem for Linux (WSL 2) by following the instructions [here](https://docs.microsoft.com/en-us/windows/wsl/install).

2. Once WSL 2 is installed or you have Linux or MacOS, we recommend running the following commands to install Homebrew, a package manager:

    ```bash
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    ```

3. Next, install Docker, Docker Compose, and Colima using Homebrew with:

    ```bash
    brew install docker docker-compose colima
    ```

4. Start Colima to handle container virtualization:
    
    ```bash
    colima start
    ```

5. With Colima running, you can now start your containers using Docker Compose:

    ```bash
    docker-compose up
    ```

    ## Worker service (background video processing)

    The `worker` service runs the background job processor that dequeues `upload_jobs` and uses ffmpeg via server-side code.

    Build and run notes:

    - Building the Rust worker binary can require several GB of RAM during linking. If your Docker VM is low on memory, the linker may be killed (SIGKILL) during image build or `cargo run` inside the container.
    - To avoid runtime compilation and the associated memory spikes, the image builds the `worker` binary at image build time and installs it to `/usr/local/bin/worker`. The `worker` service runs that binary directly.
    - If the image build fails while building `worker`, increase Docker VM memory (Docker Desktop → Resources, or `colima start --memory 8192`).

    Recommended steps:

    1. Increase Docker VM memory if needed (8GB+ suggested).
    2. Build the app image (this also builds the `worker` binary inside the image):

        ```bash
        docker compose build app
        ```

    3. Start infra and the worker:

        ```bash
        docker compose up -d db minio
        docker compose up -d worker
        ```

    4. Alternatively, prebuild the worker inside the app container and then start the `worker` service:

        ```bash
        docker compose run --rm app sh -lc '/usr/local/cargo/bin/cargo build --release --bin worker'
        docker compose up -d worker
        ```

    Troubleshooting:

    - If `docker compose up -d worker` errors with `exec: "/usr/local/bin/worker": no such file or directory`, the binary wasn’t baked into the image. Rebuild the `app` image after increasing memory, or prebuild the worker using the alternative step above.
    - The worker writes a heartbeat file at `/tmp/worker_alive` which is used by the service healthcheck. If the worker becomes unhealthy, check logs:

      ```bash
      docker compose logs -f worker
      ```

    Tuning:

    - Control concurrency: `WORKER_CONCURRENCY` (default 1)
    - Retry policy: `WORKER_MAX_ATTEMPTS` (default 5), `WORKER_BASE_BACKOFF_SECONDS` (default 60)

This should spin up all the necessary services for RustLearn to function. You're now ready to jump into the world of incentivized learning!

---

Should you have any questions or need further assistance, please raise an issue in the repository and we'll be happy to help.

## Third-party libraries & software

See `THIRD_PARTY.md` for a curated list of Rust crates, Docker images, and external tools used by this project (versions and links).