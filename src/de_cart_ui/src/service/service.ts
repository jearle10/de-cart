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
        setData(data);
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

export async function encrypt() {
  let public_encryption_key = await de_cart.ibe_encryption_key();
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
