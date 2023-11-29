import { Actor, HttpAgent, ActorSubclass } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import {
  createActor,
  de_cart_contract as de_cart,
  canisterId,
} from "../../../declarations/de_cart_contract";
import { Principal } from "@dfinity/principal";
import * as vetkd from "ic-vetkd-utils";
import { useState, useEffect } from "react";
import { _SERVICE } from "../../../declarations/de_cart_contract/de_cart_contract.did";

// - Connect to the backend canister and call the whoami function to test
// - Use the dfinity agent to call the canister from the webpage

// */

export type Service = keyof _SERVICE;

let options = {
  agentOptions: {
    host: `http://127.0.0.1:4943/?canisterId=${canisterId}`,
  },
};

let act = createActor(canisterId, options);

/*
Connect to the contract using an anonymous identity
*/
export function useContract(service: Service) {
  // const contract: any = de_cart_contract;
  const contract: any = act;
  const [products, setData] = useState<any[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    async function fetchData() {
      try {
        let data = await contract[service]();
        setData(data);
        setLoading(false);
      } catch (error) {
        console.error("Error fetching products:", error);
        setLoading(true);
      }
    }
    fetchData();
  }, [contract]);

  return { products, loading };
}

export function useEncryptionKey(service: Service) {
  // const contract: any = de_cart_contract;
  const contract: any = act;
  const [key, setData] = useState<any[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    async function fetchData() {
      try {
        let data = await contract[service]();
        console.log(data);
        setLoading(false);
      } catch (error) {
        console.error("Error fetching key:", error);
        setLoading(true);
      }
    }
    fetchData();
  }, [contract]);

  return { key, loading };
}

export async function getAllProducts() {
  let response = await de_cart.get_all_products();
  console.log(response);
}

export function useEncryption(input: any) {
  const [cipherText, setCipherText] = useState<any>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    async function encryptText() {
      let key = await de_cart.ibe_encryption_key();
      try {
        let data = await encrypt(input, key);
        setCipherText(data);
        setLoading(false);
      } catch (error) {
        console.error("Error fetching key:", error);
        setLoading(true);
      }
    }
    encryptText();
  }, []);

  return { cipherText, loading };
}

export async function encrypt(input: string, key: any): Promise<any> {
  // Transport key
  const seed = window.crypto.getRandomValues(new Uint8Array(32));
  const message_encoded = new TextEncoder().encode(input);
  console.log(message_encoded);
  let principal = Principal.fromText("2vxsx-fae");
  const ibe_ciphertext = vetkd.IBECiphertext.encrypt(
    hex_decode(key),
    principal.toUint8Array(),
    message_encoded,
    seed
  );
  return ibe_ciphertext;
}

export async function authenticatedActor(authClient: AuthClient) {
  const identity = authClient.getIdentity();
  // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
  const agent = new HttpAgent({ identity });
  // Using the interface description of our webapp, we create an actor that we use to call the service methods. We override the global actor, such that the other button handler will automatically use the new actor with the Internet Identity provided delegation.
  //   let actor = createActor(CANISTER_ID, {
  //     agent,
  //   });
  //   app_backend_principal = identity.getPrincipal();
}

export async function backendPrincipal(actor: ActorSubclass) {
  return await Actor.agentOf(actor)?.getPrincipal();
}

export async function authInternetId(setAuthClient: any) {
  let authClient = await AuthClient.create();
  authClient.login({
    onSuccess: () => setAuthClient(authClient),
  });
}

function createAgent(authClient: AuthClient) {
  const identity = authClient.getIdentity();
  // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
  const agent = new HttpAgent({ identity });
  // Using the interface description of our webapp, we create an actor that we use to call the service methods. We override the global actor, such that the other button handler will automatically use the new actor with the Internet Identity provided delegation.
  //   let app_backend_actor = createActor(process.env.APP_BACKEND_CANISTER_ID, {
  //     agent,
  //   });
  //   let app_backend_principal = identity.getPrincipal();
}

const hex_decode = (hexString: any) =>
  Uint8Array.from(
    hexString.match(/.{1,2}/g).map((byte: any) => parseInt(byte, 16))
  );
const hex_encode = (bytes: any) =>
  bytes.reduce(
    (str: any, byte: any) => str + byte.toString(16).padStart(2, "0"),
    ""
  );
