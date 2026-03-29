<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=transparent&color=0:0f2027,50:203a43,100:2c5364&height=200&section=header&text=Security%20Policy&fontSize=45&fontColor=ffffff&animation=fadeIn" />
</p>

<p align="center">
  <b>Standard Security Protocols for Task Manager Rust</b><br>
  <sub>Ensuring data integrity, privacy, and resilient infrastructure.</sub>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Security-Strict-success?style=for-the-badge&logo=shield" />
  <img src="https://img.shields.io/badge/Compliance-GDPR_Ready-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Audit-Cargo_Audit_Passed-orange?style=for-the-badge&logo=rust" />
</p>

---

## 🛡️ Vulnerability Disclosure Policy

At **Task Manager Rust**, we take the security of our users' data seriously. If you have discovered a security vulnerability, we appreciate your help in disclosing it to us in a responsible manner.

### 🚨 How to Report
> **CRITICAL:** Do not open GitHub issues for security vulnerabilities.

Please report sensitive bugs via the following secure channels:
* **Primary Contact:** [idiatullinalbert965@gmail.com](mailto:idiatullinalbert965@gmail.com)
* **Response SLA:** Acknowledgment within **24-48 hours**.
* **PGP/Encryption:** Available upon request for high-risk disclosures.

---

## 🔐 Security Implementation Details

### 1. Identity & Access Management (IAM)
* **Hashing**: Industry-standard `bcrypt` with a work factor of `12`.
* **Session Strategy**: Statefull session management via **HTTP-Only, Secure, Same-Site** cookies to mitigate XSS and CSRF.
* **Brute-Force Protection**: Rate limiting is enforced on all `/auth` and `/register` endpoints.

### 2. Data Integrity
* **SQL Injection**: Prevented at the compiler level via `SQLx` prepared statements and parameterized queries.
* **Transit Security**: TLS 1.3 is required for all production traffic.
* **Input Validation**: Zero-trust approach using `Serde` for strict DTO schema enforcement.

### 3. Supply Chain Security
* **Audit**: Weekly automated scans using `cargo-audit` and `cargo-deny`.
* **Lockfiles**: Checksum-verified dependencies via `Cargo.lock`.
* **Minimal Runtime**: Distroless/Alpine-based Docker images to reduce the attack surface.

---

## 🛠 Best Practices for Contributors

To maintain our security posture, all contributors must adhere to the following:

| Requirement | Description |
| :--- | :--- |
| **No Secrets** | Use `.env` or Secret Managers. Never hardcode keys. |
| **Least Privilege** | Database users must only have permissions for required tables. |
| **Static Analysis** | All PRs must pass `cargo clippy` with zero warnings. |
| **Error Masking** | Never return raw DB errors to the client; use custom `Exceptions`. |

---

## 📈 Incident Response Plan

In the event of a confirmed breach:
1. **Containment**: Immediate isolation of affected service nodes.
2. **Analysis**: Forensic log review via `tracing-subscriber`.
3. **Notification**: Affected users notified within the legal timeframe (GDPR/Local laws).
4. **Remediation**: Post-mortem analysis and patch deployment.

---

<p align="center">
  <img src="https://img.shields.io/badge/Secure_Coding-OWASP_Top_10-blueviolet?style=flat-square" />
  <img src="https://img.shields.io/badge/Data_Retention-Encrypted-emerald?style=flat-square" />
</p>

<p align="center">
  Released under the <b>MIT License</b>.<br>
  For emergency security concerns, contact the maintainers directly.
</p>