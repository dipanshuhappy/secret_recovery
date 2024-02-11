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
    $('#get_otp_button').text("Registering Email, Please Wait..");
    
      try {
        $('#get_otp_button').text("Getting Otp Email, Please Wait..");
        const otp = await dkim_actor.register_email($('#recovery_email_input').val());
        if(otp.Err){
            startWindToast("Error", `${otp.Err}`, "error", 30, "right")
        }
        $('#after_otp').show(1000);
        $('#otp_text').text(`${otp.Ok}`);
        $('#get_otp_button').text("Get Otp");
      }
     catch (e) {
      console.error(e)
      startWindToast("Error", `${e}`, "error", 30, "right")
    }
})

$('#register').click(async function(event) {
    event.preventDefault()
    $('#register').text("Registering, Please Wait..");
    try {
      $('#register').text("Creating CipherTexxt");
      const ibe_ciphertext = await ibe_encrypt_by_email($('#secret').val());
      console.log(ibe_ciphertext)
      $('#register').text("Registering Email");
      const result  = await dkim_actor.finalize_secret_with_email($('#raw_email').val(),ibe_ciphertext);
      if(result.Err){
              console.error(result.Err)
              startWindToast("Error", `${result.Err}`, "error", 30, "right")
      }
      startWindToast("Done", `Your Secret has been registered`, "success", 30, "right")
      $('#after_otp').hide(1000)
    }
    catch (e) {
      console.error(e)
      startWindToast("Error", `${e}`, "error", 30, "right")
    }
})


async function ibe_encrypt_by_email(message) {
  console.log("hiii")
  $('#register').text("Fetching IBE encryption key...")
  const pk_bytes_hex = await app_backend_actor.ibe_encryption_key().catch((e) => console.error(e));

  $('#register').text("Preparing IBE-encryption...")
  console.log("hiiii")
  const message_encoded = new TextEncoder().encode(message);
  console.log(message_encoded)
  const seed = window.crypto.getRandomValues(new Uint8Array(32));
  let email = $('#recovery_email_input').val()
  console.log(email,"eeeeeeeeeeeeeeeeeeeeeeeeeee")
  $('#register').text("Encrypting using email" + email + "...")
  const ibe_ciphertext = vetkd.IBECiphertext.encrypt(
    hex_decode(pk_bytes_hex),
    stringToUint8Array(email),
    message_encoded,
    seed
  );
  return hex_encode(ibe_ciphertext.serialize());
}

