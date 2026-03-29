<div align="center">

<img src="https://capsule-render.vercel.app/api?type=transparent&color=0:141E30,100:243B55&height=240&section=header&text=Task%Manager%20Rust&fontSize=50&fontColor=ffffff&animation=fadeIn&desc=High-Performance%20%E2%80%A2%20Type-Safe%20%E2%80%A2%20Scalable&descSize=20&descAlignY=65" />

<p>
  <img src="https://img.shields.io/badge/Language-Rust_1.70+-orange?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Framework-Axum_0.7-blue?style=for-the-badge&logo=tokio&logoColor=white" />
  <img src="https://img.shields.io/badge/Database-SQLx_SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white" />
</p>

<p>
  <img src="https://img.shields.io/badge/Architecture-Clean_Layered-blueviolet?style=flat-square" />
  <img src="https://img.shields.io/badge/Security-Bcrypt_Verified-success?style=flat-square" />
  <img src="https://img.shields.io/badge/CI/CD-Docker_Optimized-2496ED?style=flat-square&logo=docker&logoColor=white" />
  <img src="https://img.shields.io/badge/Status-Production_Ready-emerald?style=flat-square" />
</p>

---

### Safety of Rust, Speed of Axum, Simplicity of SQLite.

[Explore Docs](#-api-blueprint) • [Quick Start](#-getting-started) • [Contributing](#-contributing)

</div>

## 🪐 Core Philosophy

This Task Manager is a reference implementation of a **Modern Rust Backend**. It bypasses the complexity of heavy ORMs in favor of **SQLx**, providing compile-time safety for your queries while maintaining the raw performance of SQLite.

### ⚡ Key Architectural Pillars
- **Strict Domain Isolation**: Logic is decoupled from the transport layer (Axum) and storage (SQLx).
- **Zero-Trust Auth**: Session-based security with secure-only, site-restricted cookies.
- **Async Efficiency**: Powered by `tokio`, capable of handling thousands of concurrent tasks.
- **Atomic Operations**: Ensuring data integrity through SQLite transactions.

---

## 🛠 Engineering Stack

| Layer | Solution | Description |
|:---|:---|:---|
| **API Engine** | `Axum` | Ergonimic, macro-free routing |
| **Runtime** | `Tokio` | Industry-standard async executor |
| **Database** | `SQLx` | Pure SQL with Rust type-safety |
| **Identity** | `Bcrypt` | Secure-by-default hashing |
| **Telemetry** | `Tracing` | High-fidelity structured logging |

---

## 🏗 System Design

```mermaid
graph LR
    Client((fa:fa-user Client)) -.-> Auth{Auth Middleware}
    Auth --> Handlers[API Handlers]
    Handlers --> Repo[Repositories]
    Repo --> DB[(fa:fa-database SQLite)]
    
    style Auth fill:#243B55,stroke:#fff,stroke-width:2px,color:#fff
    style DB fill:#141E30,stroke:#fff,stroke-width:2px,color:#fff
````

-----

## 🚦 Getting Started

### 1\. Instant Setup

```bash
# Clone the powerhouse
git clone [https://github.com/albertidiatullin/task-manager.git](https://github.com/albertidiatullin/task-manager.git) && cd task-manager

# Environment
cp .env.example .env

# Run with maximum optimizations
cargo run --release
```

### 2\. Containerized Strategy

```bash
docker build -t task-manager .
docker run -p 8080:8080 --env-file .env task-manager
```

-----

## 📑 API Blueprint

### 🔐 Security & Identity

  - `POST /users/register` — Identity creation
  - `POST /users/auth` — Session initiation (Set-Cookie)
  - `POST /users/me/logout` — Secure session revocation

### 📝 Task Orchestration

  - `GET /tasks` — Fetch user-specific task collection
  - `POST /tasks/create_task` — Atomic task creation
  - `GET /tasks/:id` — Singular resource retrieval
  - `PUT /tasks/edit` — Patch existing task state

-----

## 🤝 Contribution Standard

We follow the **Conventional Commits** specification. To contribute:

1.  **Fork** the repository.
2.  Create your **Feature Branch** (`git checkout -b feat/sharding-support`).
3.  Ensure **Clippy** is happy: `cargo clippy -- -D warnings`.
4.  Open a **Pull Request**.

-----

<div align="center">

**Task Manager Rust** • 2026  
Created with precision by **Albert Idiatullin**

[](https://www.google.com/search?q=LICENSE)

</div>

