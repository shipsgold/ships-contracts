import { keyStores,utils, InMemorySigner } from "near-api-js"
import {setupNear} from "./index"
import * as testutil from './testutil'
import {contract} from "./index";
import ShipsContract from "./generated/client";
import { sign } from "crypto";

    

const convertStringToUint8Array = (data: string) => {
  const array = new Uint8Array(data.length)
  array.forEach((_v,idx)=> {
   array[idx] = data.charCodeAt(idx);
  })
  return array
}
describe("it tests contract connection", () => {
  let devAccountId: string;
  let keystore: keyStores.KeyStore;
  let keyPair: utils.KeyPairEd25519;
  let guestKeystore: keyStores.KeyStore;

  beforeAll(async ()=>{
    devAccountId = await testutil.getDevDeployAccount()
    keystore = await testutil.getDefaultKeystore()
    keyPair = utils.KeyPairEd25519.fromRandom() 
    console.log(keyPair.getPublicKey().toString())
    guestKeystore = new keyStores.InMemoryKeyStore();
    await guestKeystore.setKey("testnet",devAccountId,keyPair)
    console.log(await guestKeystore.getKey("testnet", "asdf"))
  })

  it("can call a view contract function", async () => {
    const near = await setupNear("testnet", keystore)
    const nearGuest = await setupNear("testnet", guestKeystore);
    const devAcct = await near.account(devAccountId)
    const guestAcct = await nearGuest.account(devAccountId);
    const contract = new ShipsContract({
      account: devAcct,
      contractId: devAccountId
    })
    const guestContract = new ShipsContract({
      account: guestAcct,
      contractId: devAccountId
    })
   console.log( guestContract.contract.account.accountId)
    try {
     await contract.new(devAcct.accountId, devAcct.accountId, 10)
    }catch(e){
      console.warn(e)
    }
    //console.log(await contract.get_count())
   // console.log(await contract.increment({amount:"1"}))
   //TODO temp
    /*console.log(await contract.add_guest(keyPair.getPublicKey().toString(), "funnyuser"))
    console.log(await guestContract.guest_code_id(keyPair.getPublicKey().toString()))
    console.log(await guestContract.increment()) 
    console.log(await guestContract.get_count())
    */
//    const contractAccount = getContract(devAcct, devAccountId) as any;
  //  await contractAccount.new({owner_id:"dev-1622302692883-89460864297760" , count: 10})
  //  const value = await contractAccount.new_owner({args: {owner_id: devAccountId }},)
   // console.log("-========================")
   // console.log(value)
    //console.log("-========================")
/*    console.log(await contractAccount.get_count())
    console.log(await contractAccount.increment())
    console.log(await contractAccount.get_count())
    */
  })
})

