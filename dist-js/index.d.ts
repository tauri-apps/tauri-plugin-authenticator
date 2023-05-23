declare global {
    interface Window {
        __TAURI_INVOKE__: <T>(cmd: string, args?: unknown) => Promise<T>;
    }
}
export declare class Authenticator {
    init(): Promise<void>;
    register(challenge: string, application: string): Promise<string>;
    verifyRegistration(challenge: string, application: string, registerData: string, clientData: string): Promise<string>;
    sign(challenge: string, application: string, keyHandle: string): Promise<string>;
    verifySignature(challenge: string, application: string, signData: string, clientData: string, keyHandle: string, pubkey: string): Promise<number>;
}
