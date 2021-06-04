import { keyStores } from "near-api-js"
import {getContract, setupNear} from "./index"
import * as testutil from './testutil'
import {contract} from "./index";
import TestContractApi from "./generated/client";

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
  beforeAll(async ()=>{
    devAccountId = await testutil.getDevDeployAccount()
    keystore = await testutil.getDefaultKeystore()
  })

  it("can call a view contract function", async () => {
    const near = await setupNear("testnet", keystore)
    const devAcct = await near.account(devAccountId)
    const contract = new TestContractApi({
      account: devAcct,
      contractId: devAccountId
    })
    console.log(await contract.get_count())
    console.log(await contract.increment({amount:"1"}))
    console.log(await contract.get_count())
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

