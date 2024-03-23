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

This should spin up all the necessary services for RustLearn to function. You're now ready to jump into the world of incentivized learning!

---

Should you have any questions or need further assistance, please raise an issue in the repository and we'll be happy to help.