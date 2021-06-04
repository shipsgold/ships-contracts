import { ContractMethods } from "near-api-js/lib/contract"

const viewMethods: string[] = [
  "get_count"
]

const changeMethods:string[] = [
 "increment",
 "new_owner",
 "new"
]

export const contractMethods: ContractMethods = {
  viewMethods,
  changeMethods
}