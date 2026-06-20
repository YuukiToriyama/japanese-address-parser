# Wrapper Integration

This document outlines the three integration paths for the core library.

## Integration Paths

```mermaid
graph TD
    subgraph Core
    ParserSync["Parser (Sync)"]
    ParserAsync["Parser (Async)"]
    ExpParser["Parser (Experimental)"]
    end

    subgraph Python
    PythonWrapper["Python Wrapper"] -->|parse_blocking| ParserSync
    end
    subgraph WASM
    WasmStable["WASM Stable"] -->|parse + ReqwestApiClient| ParserAsync
    WasmNightly["WASM Nightly"] -->|parse_with_options + GlooNetClient| ExpParser
    end
    subgraph MCP
    MCPServer["MCP Server"] -->|parse| ParserAsync
    end
```

## Output Transformations

- **Python**: `ParseResult` → `PyObject`
- **WASM**: `ParseResult` → `JsValue`
- **MCP**: `ParseResult` → `JSON`
