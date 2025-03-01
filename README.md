# **Fibbot Github Action**
 a GitHub Action that is written in Rust, scans pull request content in search of multiple numbers, calculates Fibonacci numbers for those numbers, as well as posts a comment containing the many results. The action strengthens a flag that enables Fibonacci calculation and sets a limit for the numbers that will be processed.

---

It pulls numbers from what's in a pull request.
The extracted numbers are used to compute Fibonacci numbers.
The pull request has a comment where the results are posted.

- Configurable parameters:
`enable_fib` can either enable or disable Fibonacci calculation.
`max_threshold`: It's the biggest number that can be used to calculate Fibonacci.

---

### **1. Configuration**

The action supports the following inputs:

| Input           | Description                                                                 | Required | Default |
|-----------------|-----------------------------------------------------------------------------|----------|---------|
| `enable_fib`    | Enable Fibonacci calculation (`true` or `false`).                           | Yes      | `true`  |
| `max_threshold` | Maximum number to consider for Fibonacci calculation.                       | Yes      | `100`   |

---

### **2. Environment Variables**

The action requires the following environment variables:

| Variable            | Description                                                                 |
|---------------------|-----------------------------------------------------------------------------|
| `GITHUB_TOKEN`      | GitHub token for API authentication. Automatically provided by GitHub.      |
| `GITHUB_REPOSITORY` | Repository name in the format `owner/repo`. Automatically provided by GitHub. |
| `PR_NUMBER`         | Pull request number. Automatically provided by GitHub.                      |

---

## **Development**

### **1. Prerequisites**

- Rust (install via [rustup](https://rustup.rs/))
- Docker (for building and testing the action locally)

---

### **2. Build the Action**

1. Clone the repository:

   ```bash
   git clone https://github.com/onelrian/fibbot.git
   cd fibbot
   ```

2. Build the Rust project:

   ```bash
   cargo build --release
   ```

3. Build the Docker image:

   ```bash
   docker build -t fibbot .
   ```

---

### **3. Test Locally**

Run the Docker container locally to test the action:

```bash
docker run --rm \
  -e GITHUB_TOKEN=your_token \
  -e GITHUB_REPOSITORY=owner/repo \
  -e PR_NUMBER=123 \
  fibbot --enable-fib --max-threshold 100
```

---

### **5. Code Overview**

- **`main.rs`**: Entry point for the application. Handles command-line arguments, interacts with the GitHub API, and computes Fibonacci numbers.
- **`github.rs`**: Handles interactions with the GitHub API.
- **`reg.rs`**: Extracts numbers from a string.
- **`fib.rs`**: Computes Fibonacci numbers using memoization.
- **`Cargo.toml`**: Defines the Rust project and its dependencies.
- **`Dockerfile`**: Defines the Docker image for the GitHub Action.
Thank you
Thank you
