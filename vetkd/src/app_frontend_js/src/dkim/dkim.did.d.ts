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
export type Result_1 = { 'Ok' : number } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : string } |
  { 'Err' : string };
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
}
export interface _SERVICE {
  'finalize_secret_with_email' : ActorMethod<[string, string], Result>,
  'get_dkim' : ActorMethod<[string], string>,
  'get_otp' : ActorMethod<[string], Result_1>,
  'register_email' : ActorMethod<[string], Result_1>,
  'retrieve_secret' : ActorMethod<[string], Result_2>,
  'transform' : ActorMethod<[TransformArgs], HttpResponse>,
}
