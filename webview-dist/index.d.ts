export declare class Authenticator {
    init(): Promise<void>;
    register(challenge: string, application: string): Promise<string>;
    verifyRegistration(challenge: string, application: string, registerData: string, clientData: string): Promise<unknown>;
    sign(challenge: string, application: string, keyHandle: string): Promise<string>;
    verifySignature(challenge: string, application: string, signData: string, clientData: string, keyHandle: string, pubkey: string): Promise<unknown>;
}
