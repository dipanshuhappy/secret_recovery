export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'encrypted_ibe_decryption_key_email' : IDL.Func(
        [IDL.Vec(IDL.Nat8), IDL.Text],
        [IDL.Text],
        [],
      ),
    'encrypted_ibe_decryption_key_for_caller' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Text],
        [],
      ),
    'encrypted_symmetric_key_for_caller' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Text],
        [],
      ),
    'ibe_encryption_key' : IDL.Func([], [IDL.Text], []),
    'symmetric_key_verification_key' : IDL.Func([], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
