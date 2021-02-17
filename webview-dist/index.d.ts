export declare class Authenticator {
    init(): Promise<void>;
    register(challenge: string, application: string): Promise<string>;
    sign(challenge: string, application: string, keyHandle: string): Promise<string>;
}
