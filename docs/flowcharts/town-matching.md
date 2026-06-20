# Town Matching Algorithm

This document describes the 6-pass town name matching algorithm implemented in the tokenizer.

## Pre-matching Normalization
Before the 6-pass cascade, the following normalizations are applied:
1. Fullwidth to halfwidth conversion (for numerals).
2. Arabic chōme (e.g., `1丁目`) conversion if `丁目` exists in the string.

## 6-Pass Matching Cascade

```mermaid
graph TD
    A[Start] --> B[Normalization]
    B --> C{Pass 1: As-is}
    C -- Match Found --> Z[Success]
    C -- Not Found --> D{Pass 2: Informal Notation}
    D -- Match Found --> Z
    D -- Not Found --> E{Pass 3: Prepend 大字}
    E -- Match Found --> Z
    E -- Not Found --> F{Pass 4: Prepend 字}
    F -- Match Found --> Z
    F -- Not Found --> G{Pass 5: Informal + 大字}
    G -- Match Found --> Z
    G -- Not Found --> H{Pass 6: Informal + 字}
    H -- Match Found --> Z
    H -- Not Found --> I[Failure]
```

## Prioritization Logic
The `find_town` function performs the actual candidate matching. To improve accuracy:
1. Candidates containing `丁目` (indicating a municipality with house numbering) are prioritized and checked first.
2. `OrthographicalVariantAdapter` is applied at each candidate check to handle common variations (e.g., `ッ` vs `ツ`).

## References
- Town matching implementation: `core/src/tokenizer/read_town.rs`
- Normalization and transformation formatters: `core/src/formatter/`
- Orthographical variants: `core/src/adapter/orthographical_variant_adapter.rs`
