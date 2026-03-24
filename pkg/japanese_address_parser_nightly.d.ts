/* tslint:disable */
/* eslint-disable */

export function parse_experimental(
address: string,
options: ParseOptions
): Promise<ParsedAddress>;

export interface ParseOptions {
    dataSource: "chimeiruiju" | "geolonia";
    correctIncompleteCityNames: boolean | null;
    verbose: boolean | null;
}

export interface Metadata {
    latitude: number | undefined;
    longitude: number | undefined;
    depth: number;
}

export interface ParsedAddress {
    prefecture: string;
    city: string;
    town: string;
    rest: string;
    metadata: Metadata;
}


export interface ParseResult {
    address: Address;
    error: Error | undefined;
}
export interface Address {
    prefecture: string;
    city: string;
    town: string;
    rest: string;
}
export interface Error {
    error_type: string;
    error_message: string;
}
export class Parser {
    free(): void;
    constructor();
    /**
     * @param {string} address
     * @returns {Promise<ParseResult>}
     */
    parse(address: string): Promise<ParseResult>;
}


export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_parser_free: (a: number, b: number) => void;
    readonly parse_experimental: (a: number, b: number, c: any) => any;
    readonly parser_new: () => number;
    readonly parser_parse: (a: number, b: number, c: number) => any;
    readonly start: () => void;
    readonly wasm_bindgen__closure__destroy__hdc61a9ee18a0a29b: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h75af7e4d0b8fd1e9: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hd1357206d0a49fd9: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h4a9d2138e3739ee5: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h617342383f0a7c96: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
