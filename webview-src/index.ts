import { promisified } from 'tauri/api/tauri'

export class Authenticator {

    init(): Promise<void> {
        return promisified({
            cmd: 'Init',
        })
    }

    register(challenge: string, application: string): Promise<string> {
        return promisified({
            cmd: 'Register',
            timeout: 10000,
            challenge,
            application,
        })
    }

    verifyRegistration(challenge: string, application: string, registerData: string, clientData: string) {
        return promisified({
            cmd: 'VerifyRegistration',
            challenge,
            application,
            registerData,
            clientData,
        })
    }

    sign(challenge: string, application: string, keyHandle: string): Promise<string> {
        return promisified({
            cmd: 'Sign',
            timeout: 10000,
            challenge,
            application,
            keyHandle,
        })
    }

    verifySignature(challenge: string, application: string, signData: string, clientData: string, keyHandle: string, pubkey: string) {
        return promisified({
            cmd: 'VerifySignature',
            challenge,
            application,
            signData,
            clientData,
            keyHandle,
            pubkey
        })
    }

}
