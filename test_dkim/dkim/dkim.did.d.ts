import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
}
export interface _SERVICE {
  'finalize_secret_with_email' : ActorMethod<[string], Result>,
  'get_dkim' : ActorMethod<[string], string>,
  'transform' : ActorMethod<[TransformArgs], HttpResponse>,
}
