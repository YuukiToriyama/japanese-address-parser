# Architecture

This document describes the layered structure of the core library.

```mermaid
graph TD
    API[Public API Layer] --> Tokenizer[Tokenizer Layer]
    API --> Service[Service Layer]
    Tokenizer --> Domain[Domain Layer]
    Tokenizer --> Utility[Utility Layer]
    Service --> Domain
    Service --> Repository[Repository Layer]
    Repository --> HTTP[HTTP Abstraction Layer]
    HTTP --> Utility
```

## Layers

- **Public API layer**: `core/src/parser.rs`, `core/src/experimental/parser.rs`
- **Tokenizer layer**: `core/src/tokenizer/`
- **Service layer**: `core/src/interactor/`
- **Repository layer**: `core/src/repository/`
- **HTTP abstraction layer**: `core/src/http/`
- **Domain layer**: `core/src/domain/`
- **Utility layer**: `core/src/formatter/`, `core/src/adapter/`, `core/src/util/`

## Note
The `ApiClient` trait abstraction (defined in `core/src/http/`) enables pluggable HTTP clients for flexibility in different environments (e.g., `ReqwestApiClient` for server/WASM, potentially mock clients for testing).
