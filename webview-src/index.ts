import { invoke } from '@tauri-apps/api/tauri'

export class Authenticator {

    init(): Promise<void> {
        return invoke('plugin:authenticator|init')
    }

    register(challenge: string, application: string): Promise<string> {
        return invoke('plugin:authenticator|register', {
            timeout: 10000,
            challenge,
            application,
        })
    }

    verifyRegistration(challenge: string, application: string, registerData: string, clientData: string): Promise<string> {
        return invoke('plugin:authenticator|verify_registration', {
            challenge,
            application,
            registerData,
            clientData,
        })
    }

    sign(challenge: string, application: string, keyHandle: string): Promise<string> {
        return invoke('plugin:authenticator|sign', {
            timeout: 10000,
            challenge,
            application,
            keyHandle,
        })
    }

    verifySignature(challenge: string, application: string, signData: string, clientData: string, keyHandle: string, pubkey: string): Promise<number> {
        return invoke('plugin:authenticator|verify_signature', {
            challenge,
            application,
            signData,
            clientData,
            keyHandle,
            pubkey
        })
    }

}
