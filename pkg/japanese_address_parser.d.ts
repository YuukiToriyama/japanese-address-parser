/* tslint:disable */
/* eslint-disable */

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
    readonly parser_new: () => number;
    readonly parser_parse: (a: number, b: number, c: number) => any;
    readonly start: () => void;
    readonly wasm_bindgen_248947384d859d17___convert__closures_____invoke___js_sys_bd9a87d1f9a051b1___Function_fn_wasm_bindgen_248947384d859d17___JsValue_____wasm_bindgen_248947384d859d17___sys__Undefined___js_sys_bd9a87d1f9a051b1___Function_fn_wasm_bindgen_248947384d859d17___JsValue_____wasm_bindgen_248947384d859d17___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_248947384d859d17___convert__closures_____invoke___wasm_bindgen_248947384d859d17___JsValue__core_608f92abc48d28da___result__Result_____wasm_bindgen_248947384d859d17___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_248947384d859d17___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
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
