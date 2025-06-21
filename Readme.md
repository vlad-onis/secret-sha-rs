Write a readme file for a project that shares ephemeral secrets without ever decrypting them so it's as safe as possible for the parties
# Ephemeral Secrets Sharing
This project provides a secure way to share ephemeral secrets between parties without ever decrypting them. The design ensures that the secrets remain encrypted at all times, minimizing the risk of exposure.
## Features
- **End-to-End Encryption**: Secrets are encrypted before being shared and remain encrypted throughout the process.
- **Ephemeral Nature**: Secrets are designed to be temporary and can be set to expire after a certain period or after being accessed a specified number of times.
- **Zero-Knowledge Proofs**: The system allows for verification of secret validity without revealing the secret itself.
- **Secure Sharing**: Secrets can be shared securely with multiple parties without the need for them to decrypt the secrets.

## Installation
