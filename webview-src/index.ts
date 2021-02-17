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

    sign(challenge: string, application: string, keyHandle: string): Promise<string> {
        return promisified({
            cmd: 'Sign',
            timeout: 10000,
            challenge,
            application,
            keyHandle,
        })
    }

}
