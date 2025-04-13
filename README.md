# kapsule-rt
A lightweight, sandboxed WebAssembly runtime for executing trust-verified logic kapsules locally — offline, deterministically, and securely.

**`kapsule-rt`** is the **WASM-based capsule runtime** used to:

- Execute cryptographically signed, portable logic modules (like `trust.wasm`, `rank.wasm`)
- Verify signatures and enforce capsule integrity
- Decrypt capsule payloads locally
- Log, audit, and securely manage system and storage interactions
- Operate in a secure, sandboxed environment

> It’s a **deterministic, secure, embeddable runtime** tailored for **local-first** and **trust-aware execution of capsules**.

![image](https://github.com/user-attachments/assets/d8ec0b9e-8eb3-4edc-ae5c-62002f690c0f)

---

## 🧱 Core Responsibilities of `kapsule-rt`

### 1. **Capsule Execution Engine**
- Loads `.kapsule` files containing:
  - Signed metadata
  - Encrypted payload
  - Optional `.wasm` logic reference
- Verifies capsule authenticity via digital signatures (Ed25519, ECDSA, etc.)
- Executes WASM logic in an isolated sandbox

### 2. **WASM Sandbox**
- A secure execution environment for logic modules like:
  - `trust.wasm`: trust graph evaluation
  - `rank.wasm`: feed ranking
  - `blocklist.wasm`: moderation filters
- All modules run under **strict hostcall control**, ensuring no arbitrary system access

### 3. **Decryption & Verification**
- Decrypts capsule payloads using local channel private key
- Only authorized users (channel owners) can read capsule content
- Ensures data integrity before executing embedded logic

### 4. **Memory & Storage Manager**
- Manages access to local **SQLite** storage for:
  - Capsule DAGs
  - Metadata caching
  - Snapshot syncing
- Ensures WASM modules can’t access anything outside defined sandbox scope

### 5. **Logging & Auditing**
- Captures every interaction (e.g., logic execution, capsule acceptance/rejection)
- Logs are tamper-resistant and may be used for:
  - Analytics
  - Policy audits
  - Security reviews

### 6. **Hostcall Interface**
- Defines a strict set of functions WASM logic can access
- Examples include:
  - `get_capsule_metadata()`
  - `submit_capsule_result()`
  - `get_trust_score(pubkey)`
- Prevents any I/O, filesystem, or network access unless explicitly whitelisted

---

## 🔐 Security Principles

| Property               | Enforced by `kapsule-rt`                        |
|------------------------|-------------------------------------------------|
| **Sandboxing**         | WASM modules can’t escape runtime boundaries    |
| **Signature Verification** | Ensures capsules are untampered & authentic |
| **End-to-end Encryption** | Payload is only decrypted locally             |
| **Access Control**     | Only specific hostcalls are allowed             |
| **Auditability**       | Logs all capsule interactions safely            |

---

## 🧪 Typical Execution Flow

1. **Receive capsule**
2. **Load & verify** signature
3. **Decrypt** payload (if authorized)
4. **Execute** embedded WASM logic (e.g., `trust.wasm`)
5. **Store** or update capsule DAG in SQLite
6. **Log** interaction for audit or promotion policy

---

## 💡 Real-World Use Cases

| Capsule Type       | WASM Logic (via `kapsule-rt`)    | Outcome                                           |
|--------------------|----------------------------------|---------------------------------------------------|
| `post.capsule`     | `rank.wasm`                      | Calculate trending/promoted score                 |
| `comment.capsule`  | `blocklist.wasm`                 | Filter based on content policies                  |
| `trust.capsule`    | `trust.wasm`                     | Update DAG score for author/channel               |
| `agent.capsule`    | `agent_logic.wasm`               | Perform goal-driven automation on device          |
| `ledger.capsule`   | `transfer.wasm`                  | Validate transaction, update balances             |

---

## 📦 Deployment

- Available as a portable runtime embeddable in:
  - Android/iOS apps
  - Desktop apps (Electron, native)
  - CLI tools for capsule inspection or batch analysis

---

## 🧭 Design Goals

| Goal                     | How `kapsule-rt` Achieves It                                      |
|--------------------------|-------------------------------------------------------------------|
| **Decentralized**        | No central server needed for validation or logic execution        |
| **Private-by-default**   | Decryption only on authorized devices                             |
| **Portable & Lightweight** | WASM execution engine runs across all major platforms            |
| **Policy-compatible**    | Works with federation relays via capsule flags, not content peeking |
