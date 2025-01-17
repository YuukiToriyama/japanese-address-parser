/* tslint:disable */
/* eslint-disable */
export function start(): void;

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


export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly parse_experimental: (a: number, b: number, c: any) => any;
  readonly start: () => void;
  readonly __wbg_parser_free: (a: number, b: number) => void;
  readonly parser_new: () => number;
  readonly parser_parse: (a: number, b: number, c: number) => any;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly closure124_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure159_externref_shim: (a: number, b: number, c: any, d: any) => void;
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
