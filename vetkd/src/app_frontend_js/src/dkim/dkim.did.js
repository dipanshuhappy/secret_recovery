export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat16, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  const TransformArgs = IDL.Record({
    'context' : IDL.Vec(IDL.Nat8),
    'response' : HttpResponse,
  });
  return IDL.Service({
    'finalize_secret_with_email' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
    'get_dkim' : IDL.Func([IDL.Text], [IDL.Text], []),
    'get_otp' : IDL.Func([IDL.Text], [Result_1], []),
    'register_email' : IDL.Func([IDL.Text], [Result_1], []),
    'retrieve_secret' : IDL.Func([IDL.Text], [Result_2], []),
    'transform' : IDL.Func([TransformArgs], [HttpResponse], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
