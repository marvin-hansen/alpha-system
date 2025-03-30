/* tslint:disable */
/* eslint-disable */
/**
 * @param {Request} req
 * @param {any} env
 * @param {any} ctx
 * @returns {Promise<Response>}
 */
export function fetch(req: Request, env: any, ctx: any): Promise<Response>;
/**
 * Configuration options for Cloudflare's image optimization feature:
 * <https://blog.cloudflare.com/introducing-polish-automatic-image-optimizati/>
 */
export enum PolishConfig {
  Off = 0,
  Lossy = 1,
  Lossless = 2,
}
export enum RequestRedirect {
  Error = 0,
  Follow = 1,
  Manual = 2,
}
export class IntoUnderlyingByteSource {
  free(): void;
  /**
   * @param {ReadableByteStreamController} controller
   */
  start(controller: ReadableByteStreamController): void;
  /**
   * @param {ReadableByteStreamController} controller
   * @returns {Promise<any>}
   */
  pull(controller: ReadableByteStreamController): Promise<any>;
  cancel(): void;
  readonly autoAllocateChunkSize: number;
  readonly type: string;
}
export class IntoUnderlyingSink {
  free(): void;
  /**
   * @param {any} chunk
   * @returns {Promise<any>}
   */
  write(chunk: any): Promise<any>;
  /**
   * @returns {Promise<any>}
   */
  close(): Promise<any>;
  /**
   * @param {any} reason
   * @returns {Promise<any>}
   */
  abort(reason: any): Promise<any>;
}
export class IntoUnderlyingSource {
  free(): void;
  /**
   * @param {ReadableStreamDefaultController} controller
   * @returns {Promise<any>}
   */
  pull(controller: ReadableStreamDefaultController): Promise<any>;
  cancel(): void;
}
/**
 * Configuration options for Cloudflare's minification features:
 * <https://www.cloudflare.com/website-optimization/>
 */
export class MinifyConfig {
  free(): void;
  css: boolean;
  html: boolean;
  js: boolean;
}
export class R2Range {
  free(): void;
  length?: number;
  offset?: number;
  suffix?: number;
}
