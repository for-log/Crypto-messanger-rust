use std::sync::Arc;
use futures::lock::Mutex;
use js_sys::{Object, Uint8Array, Map, Reflect, Array};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, console};
use yew::Callback;
use crate::{Chat, rsa_crypto::RsaCrypto, user::SafeUser, dialogs::MiniDialog};

const EXPONENT: [u8; 3] = [1u8, 0u8, 1u8];
const BITS: usize = 2048;

pub trait Crypt {
    fn get_rsa() -> Arc<Mutex<RsaCrypto>>;
    fn go_crypt(&self, callback: Callback<String>);
    fn send_public_key(&self, callback: Callback<String>);
    fn go_decrypt(&self, data: String, callback: Callback<String>);
    fn parse_user(&self, user: SafeUser, callback: Callback<MiniDialog>);
}

impl Crypt for Chat {
    fn get_rsa() -> Arc<Mutex<RsaCrypto>> {
        let w = window().unwrap();
        let crypt = w.crypto().unwrap();
        let subtle_crypto = crypt.subtle();
        let array = ["encrypt", "decrypt"].iter()
            .map(|x| JsValue::from_str(x))
            .collect::<Array>();
        let algorithm = {
            let a = Map::new();
            a.set(&"name".into(), &"RSA-OAEP".into());
            a.set(&"publicExponent".into(), &Uint8Array::new(
                &EXPONENT.map(|x| JsValue::from(x)).iter().collect::<Array>()));
            a.set(&"modulusLength".into(), &BITS.into());
            a.set(&"hash".into(), &"SHA-256".into());
            Object::from_entries(&a).unwrap()
        };

        let rsa = Arc::new(Mutex::new(RsaCrypto::new()));
        let promise_key = subtle_crypto.generate_key_with_object(&algorithm, true, &array).unwrap();
        let future_key = JsFuture::from(promise_key);

        spawn_local({
            let rsa = rsa.clone();
            async move {
                let mut rsa_lock = rsa.lock().await;
                let keys = future_key.await.unwrap();
                let public_key = Reflect::get(&keys, &JsValue::from_str("publicKey")).unwrap();
                let private_key = Reflect::get(&keys, &JsValue::from_str("privateKey")).unwrap();
                rsa_lock.set_private(private_key.into()).set_public(public_key.into());
            }
        });
        rsa
    }
    fn parse_user(&self, user: SafeUser, callback: Callback<MiniDialog>) {
        let decode_key = base64::decode(user.key).unwrap();
        let object_key = Uint8Array::from(&decode_key[..]);

        let subtle = window()
                .and_then(|x| x.crypto().map(|x| Some(x)).unwrap_or(None))
                .and_then(|x| Some(x.subtle()))
                .unwrap();

        let array = ["encrypt"].iter()
            .map(|x| JsValue::from_str(x))
            .collect::<Array>();
            
        let algorithm = {
            let a = Map::new();
            a.set(&"name".into(), &"RSA-OAEP".into());
            a.set(&"hash".into(), &"SHA-256".into());
            Object::from_entries(&a).unwrap()
        };
        
        let future_key = subtle.import_key_with_object("spki", &object_key, &algorithm, true, &array).unwrap();

        spawn_local(async move {
            let key = JsFuture::from(future_key).await.unwrap();
            callback.emit(MiniDialog::new(user.id, key.into()))
        });
    }
    fn send_public_key(&self, callback: Callback<String>) {
        let subtle = window()
            .and_then(|x| x.crypto().map(|x| Some(x)).unwrap_or(None))
            .and_then(|x| Some(x.subtle()))
            .unwrap();

        let clone_rsa = self.rsa.clone();
        spawn_local(async move {
            let mutex_rsa = clone_rsa.lock().await;
            let public_key = mutex_rsa.get_public().unwrap();
            let export_key = subtle.export_key("spki", public_key).unwrap();
            let array_key = JsFuture::from(export_key).await.unwrap();
            let bytes_key = Uint8Array::new(&array_key);
            callback.emit(base64::encode(bytes_key.to_vec()))
        });
    }
    fn go_decrypt(&self, data: String, callback: Callback<String>) {
        let decode_text = base64::decode(data).unwrap();
        let object_text = Uint8Array::from(&decode_text[..]);
        console::log_2(&JsValue::from_str("receive data:"), &object_text);

        let subtle = window()
            .and_then(|x| x.crypto().map(|x| Some(x)).unwrap_or(None))
            .and_then(|x| Some(x.subtle()))
            .unwrap();


        let clone_rsa = self.rsa.clone();
        spawn_local(async move {
            let mutex_rsa = clone_rsa.lock().await;
            let key = mutex_rsa.get_private().unwrap();
            let result = subtle.decrypt_with_str_and_buffer_source(
                "RSA-OAEP", 
                key,
                &object_text
            ).unwrap();
            let future_result = JsFuture::from(result).await
                .unwrap();
            let bytes_result = Uint8Array::new(&future_result);
            callback.emit(String::from_utf8(bytes_result.to_vec()).unwrap());
        });
    }
    fn go_crypt(&self, callback: Callback<String>) {
        let decode_text = self.text.as_bytes();
        let object_text = Uint8Array::from(&decode_text[..]);
        let dialog_id = match self.dialog_id {
            Some(id) => id,
            None => return
        };
        let public_key = self.dialogs.get(&dialog_id).unwrap().dialog_key.clone();

        let subtle = window()
            .and_then(|x| x.crypto().map(|x| Some(x)).unwrap_or(None))
            .and_then(|x| Some(x.subtle()))
            .unwrap();

        spawn_local(async move {
            let result = subtle.encrypt_with_str_and_buffer_source(
                "RSA-OAEP", 
                &public_key,
                &object_text
            ).unwrap();
            let future_result = JsFuture::from(result).await
                .unwrap();
            let bytes_result = Uint8Array::new(&future_result);
            console::log_2(&JsValue::from_str("send data:"), &bytes_result);
            callback.emit(base64::encode(bytes_result.to_vec()));
        });
    }
}