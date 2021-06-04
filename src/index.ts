import { Account, Near, Contract} from "near-api-js"
import { KeyStore } from "near-api-js/lib/key_stores"
import * as config from "./config"
import {contractMethods} from "./methods"
export * as contract from "./generated/client"

export const setupNear = async (networkID :config.SupportedNetworkType, keyStore: KeyStore) : Promise<Near> => {
  const cfg = config.getConnectionConfig(networkID)
  return new Near({...cfg, deps: {keyStore}})
}

export const getContract = (account: Account, contractName: string) => {
    return new Contract(account,contractName, contractMethods)
}


/*

  const near = setupNear()
  const contractAccount = new Account(connection, contractName);
contractAccount.addAccessKey = (publicKey) => contractAccount.addKey(publicKey, contractName, contractMethods.changeMethods, parseNearAmount('0.1'));
const contract = new Contract(contractAccount, contractName, contractMethods);



*/