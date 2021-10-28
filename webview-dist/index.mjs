import { invoke } from '@tauri-apps/api/tauri';

class Authenticator {
    init() {
        return invoke('plugin:authenticator|init');
    }
    register(challenge, application) {
        return invoke('plugin:authenticator|register', {
            timeout: 10000,
            challenge,
            application,
        });
    }
    verifyRegistration(challenge, application, registerData, clientData) {
        return invoke('plugin:authenticator|verify_registration', {
            challenge,
            application,
            registerData,
            clientData,
        });
    }
    sign(challenge, application, keyHandle) {
        return invoke('plugin:authenticator|sign', {
            timeout: 10000,
            challenge,
            application,
            keyHandle,
        });
    }
    verifySignature(challenge, application, signData, clientData, keyHandle, pubkey) {
        return invoke('plugin:authenticator|verify_signature', {
            challenge,
            application,
            signData,
            clientData,
            keyHandle,
            pubkey
        });
    }
}

export { Authenticator };
