'use strict';

var primitives = require('@tauri-apps/api/primitives');

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
class Authenticator {
    async init() {
        return await primitives.invoke("plugin:authenticator|init_auth");
    }
    async register(challenge, application) {
        return await primitives.invoke("plugin:authenticator|register", {
            timeout: 10000,
            challenge,
            application,
        });
    }
    async verifyRegistration(challenge, application, registerData, clientData) {
        return await primitives.invoke("plugin:authenticator|verify_registration", {
            challenge,
            application,
            registerData,
            clientData,
        });
    }
    async sign(challenge, application, keyHandle) {
        return await primitives.invoke("plugin:authenticator|sign", {
            timeout: 10000,
            challenge,
            application,
            keyHandle,
        });
    }
    async verifySignature(challenge, application, signData, clientData, keyHandle, pubkey) {
        return await primitives.invoke("plugin:authenticator|verify_signature", {
            challenge,
            application,
            signData,
            clientData,
            keyHandle,
            pubkey,
        });
    }
}

exports.Authenticator = Authenticator;
