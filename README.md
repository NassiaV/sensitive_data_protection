# Sensitive Data Protection (Rust + Python, Adversarial Evaluation)

This project explores a **two-track pipeline for protecting sensitive data** using **Rust** for high-performance, safe transformations and **Python (PyTorch)** for adversarial evaluation.  

The goal is to combine **practical data protection mechanisms** (masking & encryption) with a **red-team inspired evaluation**: testing whether masked/encrypted data still leaks recoverable information under machine learning analysis.

---

## Pipeline Overview

**Input**: Structured dataset in JSON Lines format (`.jsonl`), containing typical sensitive fields such as names, emails, phone numbers, financial IDs, and free-text notes.

**Output**: Two parallel protected versions of the dataset:
1. **Masked dataset** → PII fields are redacted or partially preserved (e.g., keeping last digits). This keeps the file human-readable while limiting leakage.
2. **Encrypted dataset** → Full AES-256-GCM encryption per record, producing ciphertext and nonces. This ensures strong confidentiality but removes readability.

```
Raw JSONL ──► [Masking] ──► masked.jsonl
         └──► [AES Encryption] ──► encrypted.jsonl
```

The **Rust core** is planned to implement this pipeline:
- Masking with regex-based redaction and deterministic rules.  
- AES-GCM encryption with per-record nonces and a shared key from environment variables.  
- File I/O for large-scale JSONL input/output handling.

---

## Adversarial Evaluation (Python / PyTorch)

Masking is often considered "good enough" for data sharing, but adversaries may exploit context to reconstruct hidden fields. To study this risk, a second dataset (with disjoint samples) will be used for training and evaluating adversarial models.

The **PyTorch adversary** is planned to simulate a potential attacker:
- Input: Masked records (with partial or redacted fields).  
- Target: Predict hidden values (e.g., last digits of phone numbers, domains of emails, or tokens in free-text notes).  
- Model: Simple neural architectures (MLP, sequence models) trained to guess missing fields from contextual information.  

Evaluation will report metrics such as:
- Character-level accuracy of reconstructed fields.  
- Precision/Recall for correctly guessed tokens.  
- Overall reconstruction risk as a measure of masking robustness.  

---

## Current Status
- Synthetic dataset schema and generator prepared.  
- Rust pipeline (masking & encryption) in progress.  
- Python adversarial evaluation (PyTorch models) in progress.  
- Documentation in progress.  

---

## Roadmap
- Complete Rust pipeline for masking & encryption.  
- Implement and test adversarial evaluation models in PyTorch.  
- Improve masking strategies (tokenization, format-preserving encryption).  
- Explore adversarial models beyond baselines (transformers, contextual embeddings).  
- Benchmark against larger synthetic datasets.  
- Publish evaluation reports to measure practical leakage risk.  
