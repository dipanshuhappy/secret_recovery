import $ from "jquery";
import { createActor, app_backend } from "./app_backend";
import {createActor as createDkim, dkim} from "./dkim";
import * as vetkd from "ic-vetkd-utils";
import { AuthClient } from "@dfinity/auth-client"
import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import {startWindToast} from "@mariojgt/wind-notify/packages/index"
let app_backend_actor = app_backend;
let dkim_actor = dkim;

const hex_decode = (hexString) =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));
const hex_encode = (bytes) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');
function stringToUint8Array(str) {
  // Encode string to UTF-8
  const utf8Encoder = new TextEncoder();
  const bytes = utf8Encoder.encode(str);
  
  // Create Uint8Array from the UTF-8 encoded bytes
  const uint8Array = new Uint8Array(bytes);
  
  return uint8Array;
}
// title = you notification title

// message = you notification message

// alertType = warning, success, info , error

// time = How long that notification will be available need to be in frames

// position = top, bottom, right, left


$('#after_otp').hide();
$('#get_otp_button').click(async function() {
    event.preventDefault()
    $('#get_otp_button').text("Registering Email, Please Wait..");
    
      try {
        $('#get_otp_button').text("Getting Otp Email, Please Wait..");
        const otp = await dkim_actor.get_otp($('#recovery_email_input').val());
        if(otp.Err){
            startWindToast("Error", `${otp.Err}`, "error", 30, "right")
        }
        else{
            $('#after_otp').show(1000);
            $('#otp_text').text(`${otp.Ok}`);
            $('#get_otp_button').text("Get Otp");
        }
       
      }
     catch (e) {
      console.error(e)
      startWindToast("Error", `${e}`, "error", 30, "right")
    }
})

$('#retrieve').click(async function(event) {
    event.preventDefault()
    $('#retrieve').text("Registering, Please Wait..");
    try {

      $('#retrieve').text("Getting Cipher Text...");

      const secret = await dkim_actor.retrieve_secret($('#raw_email').val());
        if(secret.Err){
                console.error(secret.Err)
                startWindToast("Error", `${secret.Err}`, "error", 30, "right")
        }
    const plaintext = await ibe_decrypt_by_email(secret.Ok.toString(),$('#recovery_email_input').val());
    $('#final_plaintext').text(plaintext);
    //   const ibe_ciphertext = await ibe_encrypt_by_email($('#secret').val());
    //   console.log(ibe_ciphertext)
    //   $('#retrieve').text("Registering Email");
    //   const result  = await dkim_actor.finalize_secret_with_email($('#raw_email').val(),ibe_ciphertext);
    //   if(result.Err){
    //           console.error(result.Err)
    //           startWindToast("Error", `${result.Err}`, "error", 30, "right")
    //   }
    //   startWindToast("Done", `Your Secret has been registered`, "success", 30, "right")
      $('#after_otp').hide(1000)
    }
    catch (e) {
      console.error(e)
      startWindToast("Error", `${e}`, "error", 30, "right")
    }
})




async function ibe_decrypt_by_email(ibe_ciphertext_hex,email) {
  $('#register').text("Preparing IBE-decryption...")
  const tsk_seed = window.crypto.getRandomValues(new Uint8Array(32));
  const tsk = new vetkd.TransportSecretKey(tsk_seed);
  $('#register').text("Fetching IBE decryption key...")
  const ek_bytes_hex = await app_backend_actor.encrypted_ibe_decryption_key_email(tsk.public_key(),email);
  $('#register').text("Fetching IBE enryption key (needed for verification)...")
  const pk_bytes_hex = await app_backend_actor.ibe_encryption_key();
  console.log({ek_bytes_hex,pk_bytes_hex,email});
  console.log(stringToUint8Array(email),"emaillll")
  const k_bytes = tsk.decrypt(
    hex_decode(ek_bytes_hex),
    hex_decode(pk_bytes_hex),
    stringToUint8Array(email)
  );

  const ibe_ciphertext = vetkd.IBECiphertext.deserialize(hex_decode(ibe_ciphertext_hex));
  const ibe_plaintext = ibe_ciphertext.decrypt(k_bytes);

  console.log({ibe_plaintext},"jjskfldlj")
  const a = new TextDecoder('utf-8').decode(ibe_plaintext);
  console.log(a,"textsldflj")
  return String.fromCharCode.apply(null,ibe_plaintext);
}