
// Code generated by zane generator DO NOT EDIT.
import { RequestManager, PostMessageWindowTransport, PostMessageIframeTransport, WebSocketTransport, HTTPTransport, Client, JSONRPCError } from "@open-rpc/client-js";
import _ from "lodash";
import { OpenrpcDocument as OpenRPC, MethodObject, ContentDescriptorObject } from "@open-rpc/meta-schema";
import { MethodCallValidator, MethodNotFoundError } from "@open-rpc/schema-utils-js";
import { Account, Near, Contract} from "near-api-js";
import BN from "bn.js"

export type Repo = any;
export type Org = any;
export type OriginType = any;
export type Limit = string;
export type From = string;
export type Reverse = boolean;
export type ReleaseName = string;
export type Minor = number;
export type Major = number;
export type Patch = number;
export interface ObjectOfPatchMinorMajorWEjpKCk5 {
  minor: Minor;
  major: Major;
  patch: Patch;
  [k: string]: any;
}
export type U128 = string;
export type U64 = string;
export type AnyL9Fw4VUO = any;
export type Owner = string;
export type Uri = string;
export type ProjectId = string;
export interface Project {
  name?: AnyL9Fw4VUO;
  owner: Owner;
  uri: Uri;
  id: ProjectId;
  [k: string]: any;
}
export type Projects = [Project];
export type AccountId = string;
export type ReleaseId = string;
export type ReleaseStatus = "ACTIVE" | "CLOSED";
export interface PricingCurve {
  min?: U128;
  max?: U128;
  token_cap?: U128;
  [k: string]: any;
}
export interface ObjectOfObjectOfPatchMinorMajorWEjpKCk5ReleaseStatusAccountIdReleaseIdU128PricingCurveUM7M6NvY {
  version?: ObjectOfPatchMinorMajorWEjpKCk5;
  releaser?: AccountId;
  release_id?: ReleaseId;
  pre_allocation?: U128;
  status?: ReleaseStatus;
  curve?: PricingCurve;
  [k: string]: any;
}
export type Releases = [ObjectOfObjectOfPatchMinorMajorWEjpKCk5ReleaseStatusAccountIdReleaseIdU128PricingCurveUM7M6NvY];
export type UserId = string;
export type PubKey = string;
export type Name = string;
export interface Details {
  repo: Repo;
  org: Org;
  origin_type: OriginType;
  [k: string]: any;
}
export interface PaginationOptions {
  limit: Limit;
  from: From;
  reverse: Reverse;
  [k: string]: any;
}
export type Id = string;
export interface ReleaseDetails {
  name: ReleaseName;
  version: ObjectOfPatchMinorMajorWEjpKCk5;
  [k: string]: any;
}
export interface ReleaseTerms {
  min: U128;
  max: U128;
  pre_allocation: U128;
  [k: string]: any;
}
export type OwnerId = string;
export type VerifierId = string;
export type Count = number;
export type NumberHo1ClIqD = number;
export type IsCreated = boolean;
export type NullQu0Arl1F = null;
export type TempUserResult = boolean;
export interface PaginatedProjects {
  total: U64;
  projects: Projects;
  [k: string]: any;
}
export interface PaginatedReleases {
  total: U64;
  releases: Releases;
}
/**
 *
 * Generated! Represents an alias to any of the provided schemas
 *
 */
export type AnyOfAccountIdUserIdPubKeyUserIdPubKeyPubKeyNameUriDetailsAccountIdPaginationOptionsIdPaginationOptionsIdReleaseDetailsReleaseTermsOwnerIdVerifierIdCountNumberHo1ClIqDIsCreatedNullQu0Arl1FNullQu0Arl1FNullQu0Arl1FTempUserResultIdPaginatedProjectsPaginatedReleasesReleaseIdNumberHo1ClIqDNullQu0Arl1F = AccountId | UserId | PubKey | Name | Uri | Details | PaginationOptions | Id | ReleaseDetails | ReleaseTerms | OwnerId | VerifierId | Count | NumberHo1ClIqD | IsCreated | NullQu0Arl1F | TempUserResult | PaginatedProjects | PaginatedReleases | ReleaseId;
export type Increment = () => Promise<NumberHo1ClIqD>;
export type IsCreatorRegistered = (account_id: AccountId) => Promise<IsCreated>;
export type RegisterCreator = () => Promise<NullQu0Arl1F>;
export type AddTempUser = (user_id: UserId, access_key: PubKey) => Promise<NullQu0Arl1F>;
export type ResetTempUser = (user_id: UserId, access_key: PubKey) => Promise<NullQu0Arl1F>;
export type IsTempUser = (access_key: PubKey) => Promise<TempUserResult>;
export type CreateProject = (name: Name, uri: Uri, details: Details) => Promise<Id>;
export type GetProjects = (owner_id: AccountId, paginationOptions?: PaginationOptions) => Promise<PaginatedProjects>;
export type GetReleases = (project_id: Id, paginationOptions?: PaginationOptions) => Promise<PaginatedReleases>;
export type CreateNewRelease = (project_id: Id, details: ReleaseDetails, terms: ReleaseTerms) => Promise<ReleaseId>;
export type GetCount = () => Promise<NumberHo1ClIqD>;
export type New = (owner_id: OwnerId, verifier_id: VerifierId, count: Count) => Promise<NullQu0Arl1F>;

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTIncrement = [...Parameters<Increment>, ChangeMethodOptions?]
  type RTIncrement = ReturnType<Increment>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTIsCreatorRegistered = [...Parameters<IsCreatorRegistered>, ChangeMethodOptions?]
  type RTIsCreatorRegistered = ReturnType<IsCreatorRegistered>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTRegisterCreator = [...Parameters<RegisterCreator>, ChangeMethodOptions?]
  type RTRegisterCreator = ReturnType<RegisterCreator>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTAddTempUser = [...Parameters<AddTempUser>, ChangeMethodOptions?]
  type RTAddTempUser = ReturnType<AddTempUser>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTResetTempUser = [...Parameters<ResetTempUser>, ChangeMethodOptions?]
  type RTResetTempUser = ReturnType<ResetTempUser>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTIsTempUser = [...Parameters<IsTempUser>, ChangeMethodOptions?]
  type RTIsTempUser = ReturnType<IsTempUser>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTCreateProject = [...Parameters<CreateProject>, ChangeMethodOptions?]
  type RTCreateProject = ReturnType<CreateProject>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTGetProjects = [...Parameters<GetProjects>, ChangeMethodOptions?]
  type RTGetProjects = ReturnType<GetProjects>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTGetReleases = [...Parameters<GetReleases>, ChangeMethodOptions?]
  type RTGetReleases = ReturnType<GetReleases>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTCreateNewRelease = [...Parameters<CreateNewRelease>, ChangeMethodOptions?]
  type RTCreateNewRelease = ReturnType<CreateNewRelease>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTGetCount = [...Parameters<GetCount>, ChangeMethodOptions?]
  type RTGetCount = ReturnType<GetCount>

  /**
   * Generated typings 
   */
  // tslint:disable-next-line:max-line-length
  type GTNew = [...Parameters<New>, ChangeMethodOptions?]
  type RTNew = ReturnType<New>


export interface Options {
    account: Account;
    contractId: string;
}

export type NearNumber = BN | string | number;

export interface ChangeMethodOptions {
  gas?: NearNumber 
  amount?: NearNumber
}

const isMetaObject = (x: any): boolean => {
  if(x && (x.gas || x.amount)) return true
  return false
}

export class ShipsProjectContract {
  public static openrpcDocument: OpenRPC = {"openrpc":"1.2.4","info":{"version":"0.0.1","description":"Ships Project Contract","title":"ShipsProjectContract"},"methods":[{"name":"increment","params":[],"tags":[{"name":"change","description":"change method"}],"result":{"name":"count","schema":{"type":"number","title":"number_Ho1clIqD"}}},{"name":"is_creator_registered","params":[{"name":"account_id","description":"Id for the project creator","schema":{"title":"AccountId","type":"string"},"required":true}],"result":{"name":"CreatorResult","schema":{"title":"isCreated","type":"boolean"},"required":true},"tags":[{"name":"view","description":"view method"}]},{"name":"register_creator","params":[],"description":"Register user for creating projects","result":{"name":"null","schema":{"type":"null","title":"null_Qu0Arl1F"}},"tags":[{"name":"change","description":"change method"}]},{"name":"add_temp_user","params":[{"name":"user_id","schema":{"title":"user_id","type":"string"},"required":true},{"name":"access_key","schema":{"title":"PubKey","type":"string"},"required":true}],"tags":[{"name":"change","description":"change method"}],"result":{"name":"TemporaryUserResult","schema":{"type":"null","title":"null_Qu0Arl1F"}}},{"name":"reset_temp_user","params":[{"name":"user_id","schema":{"title":"user_id","type":"string"},"required":true},{"name":"access_key","schema":{"title":"PubKey","type":"string"},"required":true}],"tags":[{"name":"change","description":"change method"}],"result":{"name":"ResetUser","schema":{"type":"null","title":"null_Qu0Arl1F"}}},{"name":"is_temp_user","params":[{"name":"access_key","schema":{"title":"PubKey","type":"string"},"required":true}],"tags":[{"name":"view","description":"view method"}],"result":{"name":"tempUserResult","schema":{"title":"tempUserResult","type":"boolean"}}},{"name":"create_project","params":[{"name":"name","description":"Name of the project","schema":{"title":"name","type":"string"},"required":true},{"name":"uri","description":"URI for the project","schema":{"title":"uri","type":"string"},"required":true},{"name":"details","description":"Project details","schema":{"title":"details","type":"object","properties":{"repo":{"title":"repo","schema":{"type":"string"}},"org":{"title":"org","schema":{"type":"string"}},"origin_type":{"title":"origin_type","schema":{"type":"string","enum":["one","two","three"]}}},"required":["repo","org","origin_type"]},"required":true}],"tags":[{"name":"change","description":"change method"}],"result":{"name":"projectIdResult","description":"result from creating a project","schema":{"title":"id","type":"string"}}},{"name":"get_projects","description":"get back a paginated list of projects by account_id","params":[{"name":"owner_id","description":"Project owner id","schema":{"title":"AccountId","type":"string"},"required":true},{"name":"paginationOptions","description":"Pagination Options for making request","schema":{"title":"paginationOptions","type":"object","properties":{"limit":{"title":"limit","type":"string"},"from":{"title":"from","type":"string"},"reverse":{"title":"reverse","type":"boolean"}},"required":["limit","from","reverse"]},"required":false}],"result":{"name":"projectsPagination","description":"Result of a project with pagination","schema":{"title":"paginatedProjects","type":"object","properties":{"total":{"title":"u64","type":"string"},"projects":{"title":"Projects","type":"array","items":[{"type":"object","title":"project","properties":{"name":{"title":"any_l9Fw4VUO"},"owner":{"title":"owner","type":"string"},"uri":{"title":"uri","type":"string"},"id":{"title":"project_id","type":"string"}},"required":["owner","uri","id"]}]}},"required":["total","projects"]},"required":true},"tags":[{"name":"view","description":"view method"}]},{"name":"get_releases","description":"get back a paginated list of projects by account_id","params":[{"name":"project_id","description":"Id of the project","schema":{"title":"id","type":"string"},"required":true},{"name":"paginationOptions","description":"Pagination Options for making request","schema":{"title":"paginationOptions","type":"object","properties":{"limit":{"title":"limit","type":"string"},"from":{"title":"from","type":"string"},"reverse":{"title":"reverse","type":"boolean"}},"required":["limit","from","reverse"]},"required":false}],"result":{"name":"releasesPagination","description":"Result of fetching releases with pagination","schema":{"title":"paginatedReleases","type":"object","properties":{"total":{"title":"u64","type":"string"},"releases":{"type":"array","title":"releases","items":[{"type":"object","properties":{"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"},"releaser":{"title":"AccountId","type":"string"},"release_id":{"title":"ReleaseId","type":"string"},"pre_allocation":{"title":"u128","type":"string"},"status":{"title":"ReleaseStatus","type":"string","enum":["ACTIVE","CLOSED"]},"curve":{"title":"PricingCurve","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"token_cap":{"title":"u128","type":"string"}}}},"title":"objectOf_objectOf_patch_minor_major_wEjpKCk5_ReleaseStatus_AccountId_ReleaseId_u128_PricingCurve_uM7M6NvY"}]}},"additionalProperties":false,"required":["total","releases"]},"required":true},"tags":[{"name":"view","description":"view method"}]},{"name":"create_new_release","description":"Create a new release for project id","params":[{"name":"project_id","description":"Id of the project","schema":{"title":"id","type":"string"},"required":true},{"name":"details","schema":{"title":"ReleaseDetails","type":"object","properties":{"name":{"title":"ReleaseName","type":"string"},"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"}},"required":["name","version"]},"required":true},{"name":"terms","schema":{"title":"ReleaseTerms","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"pre_allocation":{"title":"u128","type":"string"}},"required":["min","max","pre_allocation"]},"required":true}],"tags":[{"name":"change","description":"change method"}],"result":{"name":"createNewReleaseResult","description":"result from creating a new release the release id","schema":{"title":"ReleaseId","type":"string"}}},{"name":"get_count","params":[],"tags":[{"name":"view","description":"view method"}],"result":{"name":"countResult","schema":{"type":"number","title":"number_Ho1clIqD"}}},{"name":"new","description":"initialization method","tags":[{"name":"change","description":"change method"}],"params":[{"name":"owner_id","required":true,"schema":{"title":"owner_id","type":"string"}},{"name":"verifier_id","required":true,"schema":{"title":"verifier_id","type":"string"}},{"name":"count","required":true,"schema":{"title":"count","type":"number"}}],"result":{"name":"InitResult","required":false,"schema":{"type":"null","title":"null_Qu0Arl1F"}}}],"components":{"tags":{"change":{"name":"change","description":"change method"},"view":{"name":"view","description":"view method"}},"contentDescriptors":{"pubKey":{"name":"access_key","schema":{"title":"PubKey","type":"string"},"required":true},"TempUserResult":{"name":"tempUserResult","schema":{"title":"tempUserResult","type":"boolean"}},"CreatorId":{"name":"account_id","description":"Id for the project creator","schema":{"title":"AccountId","type":"string"},"required":true},"ProjectOwnerId":{"name":"owner_id","description":"Project owner id","schema":{"title":"AccountId","type":"string"},"required":true},"UserId":{"name":"user_id","schema":{"title":"user_id","type":"string"},"required":true},"ProjectId":{"name":"project_id","description":"Id of the project","schema":{"title":"id","type":"string"},"required":true},"ProjectName":{"name":"name","description":"Name of the project","schema":{"title":"name","type":"string"},"required":true},"ProjectUri":{"name":"uri","description":"URI for the project","schema":{"title":"uri","type":"string"},"required":true},"ProjectDetails":{"name":"details","description":"Project details","schema":{"title":"details","type":"object","properties":{"repo":{"title":"repo","schema":{"type":"string"}},"org":{"title":"org","schema":{"type":"string"}},"origin_type":{"title":"origin_type","schema":{"type":"string","enum":["one","two","three"]}}},"required":["repo","org","origin_type"]},"required":true},"CreateProjectResult":{"name":"projectIdResult","description":"result from creating a project","schema":{"title":"id","type":"string"}},"ReleaseDetails":{"name":"details","schema":{"title":"ReleaseDetails","type":"object","properties":{"name":{"title":"ReleaseName","type":"string"},"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"}},"required":["name","version"]},"required":true},"ReleaseTerms":{"name":"terms","schema":{"title":"ReleaseTerms","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"pre_allocation":{"title":"u128","type":"string"}},"required":["min","max","pre_allocation"]},"required":true},"CreateNewReleaseResult":{"name":"createNewReleaseResult","description":"result from creating a new release the release id","schema":{"title":"ReleaseId","type":"string"}},"GetReleasesResult":{"name":"releasesPagination","description":"Result of fetching releases with pagination","schema":{"title":"paginatedReleases","type":"object","properties":{"total":{"title":"u64","type":"string"},"releases":{"type":"array","title":"releases","items":[{"type":"object","properties":{"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"},"releaser":{"title":"AccountId","type":"string"},"release_id":{"title":"ReleaseId","type":"string"},"pre_allocation":{"title":"u128","type":"string"},"status":{"title":"ReleaseStatus","type":"string","enum":["ACTIVE","CLOSED"]},"curve":{"title":"PricingCurve","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"token_cap":{"title":"u128","type":"string"}}}},"title":"objectOf_objectOf_patch_minor_major_wEjpKCk5_ReleaseStatus_AccountId_ReleaseId_u128_PricingCurve_uM7M6NvY"}]}},"additionalProperties":false,"required":["total","releases"]},"required":true},"GetProjectsResult":{"name":"projectsPagination","description":"Result of a project with pagination","schema":{"title":"paginatedProjects","type":"object","properties":{"total":{"title":"u64","type":"string"},"projects":{"title":"Projects","type":"array","items":[{"type":"object","title":"project","properties":{"name":{"title":"any_l9Fw4VUO"},"owner":{"title":"owner","type":"string"},"uri":{"title":"uri","type":"string"},"id":{"title":"project_id","type":"string"}},"required":["owner","uri","id"]}]}},"required":["total","projects"]},"required":true},"PaginationOptions":{"name":"paginationOptions","description":"Pagination Options for making request","schema":{"title":"paginationOptions","type":"object","properties":{"limit":{"title":"limit","type":"string"},"from":{"title":"from","type":"string"},"reverse":{"title":"reverse","type":"boolean"}},"required":["limit","from","reverse"]},"required":false}},"schemas":{"U128":{"title":"u128","type":"string"},"U64":{"title":"u64","type":"string"},"Projects":{"title":"Projects","type":"array","items":[{"type":"object","title":"project","properties":{"name":{"title":"any_l9Fw4VUO"},"owner":{"title":"owner","type":"string"},"uri":{"title":"uri","type":"string"},"id":{"title":"project_id","type":"string"}},"required":["owner","uri","id"]}]},"PaginatedProjects":{"title":"paginatedProjects","type":"object","properties":{"total":{"title":"u64","type":"string"},"projects":{"title":"Projects","type":"array","items":[{"type":"object","title":"project","properties":{"name":{"title":"any_l9Fw4VUO"},"owner":{"title":"owner","type":"string"},"uri":{"title":"uri","type":"string"},"id":{"title":"project_id","type":"string"}},"required":["owner","uri","id"]}]}},"required":["total","projects"]},"PaginatedReleases":{"title":"paginatedReleases","type":"object","properties":{"total":{"title":"u64","type":"string"},"releases":{"type":"array","title":"releases","items":[{"type":"object","properties":{"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"},"releaser":{"title":"AccountId","type":"string"},"release_id":{"title":"ReleaseId","type":"string"},"pre_allocation":{"title":"u128","type":"string"},"status":{"title":"ReleaseStatus","type":"string","enum":["ACTIVE","CLOSED"]},"curve":{"title":"PricingCurve","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"token_cap":{"title":"u128","type":"string"}}}},"title":"objectOf_objectOf_patch_minor_major_wEjpKCk5_ReleaseStatus_AccountId_ReleaseId_u128_PricingCurve_uM7M6NvY"}]}},"additionalProperties":false,"required":["total","releases"]},"PaginationOptions":{"title":"paginationOptions","type":"object","properties":{"limit":{"title":"limit","type":"string"},"from":{"title":"from","type":"string"},"reverse":{"title":"reverse","type":"boolean"}},"required":["limit","from","reverse"]},"AccountId":{"title":"AccountId","type":"string"},"ReleaseId":{"title":"ReleaseId","type":"string"},"TokenId":{"title":"TokenId","type":"string"},"TempUserResult":{"title":"tempUserResult","type":"boolean"},"Uri":{"title":"uri","type":"string"},"ReleaseName":{"title":"ReleaseName","type":"string"},"ReleaseDetails":{"title":"ReleaseDetails","type":"object","properties":{"name":{"title":"ReleaseName","type":"string"},"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"}},"required":["name","version"]},"ReleaseTerms":{"title":"ReleaseTerms","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"pre_allocation":{"title":"u128","type":"string"}},"required":["min","max","pre_allocation"]},"Releases":{"type":"array","title":"releases","items":[{"type":"object","properties":{"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"},"releaser":{"title":"AccountId","type":"string"},"release_id":{"title":"ReleaseId","type":"string"},"pre_allocation":{"title":"u128","type":"string"},"status":{"title":"ReleaseStatus","type":"string","enum":["ACTIVE","CLOSED"]},"curve":{"title":"PricingCurve","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"token_cap":{"title":"u128","type":"string"}}}},"title":"objectOf_objectOf_patch_minor_major_wEjpKCk5_ReleaseStatus_AccountId_ReleaseId_u128_PricingCurve_uM7M6NvY"}]},"Release":{"type":"object","properties":{"version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"},"releaser":{"title":"AccountId","type":"string"},"release_id":{"title":"ReleaseId","type":"string"},"pre_allocation":{"title":"u128","type":"string"},"status":{"title":"ReleaseStatus","type":"string","enum":["ACTIVE","CLOSED"]},"curve":{"title":"PricingCurve","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"token_cap":{"title":"u128","type":"string"}}}},"title":"objectOf_objectOf_patch_minor_major_wEjpKCk5_ReleaseStatus_AccountId_ReleaseId_u128_PricingCurve_uM7M6NvY"},"ReleaseStatus":{"title":"ReleaseStatus","type":"string","enum":["ACTIVE","CLOSED"]},"PricingCurve":{"title":"PricingCurve","type":"object","properties":{"min":{"title":"u128","type":"string"},"max":{"title":"u128","type":"string"},"token_cap":{"title":"u128","type":"string"}}},"ProjectId":{"title":"id","type":"string"},"ProjectName":{"title":"name","type":"string"},"ProjectUri":{"title":"uri","type":"string"},"Version":{"type":"object","properties":{"minor":{"title":"minor","type":"number"},"major":{"title":"major","type":"number"},"patch":{"title":"patch","type":"number"}},"required":["minor","major","patch"],"title":"objectOf_patch_minor_major_wEjpKCk5"},"Project":{"type":"object","title":"project","properties":{"name":{"title":"any_l9Fw4VUO"},"owner":{"title":"owner","type":"string"},"uri":{"title":"uri","type":"string"},"id":{"title":"project_id","type":"string"}},"required":["owner","uri","id"]},"ProjectDetails":{"title":"details","type":"object","properties":{"repo":{"title":"repo","schema":{"type":"string"}},"org":{"title":"org","schema":{"type":"string"}},"origin_type":{"title":"origin_type","schema":{"type":"string","enum":["one","two","three"]}}},"required":["repo","org","origin_type"]},"AccessKey":{"title":"access_key","description":"The account public key","schema":{"type":"string"}},"UserId":{"title":"user_id","type":"string"}}}} ;
  public contract: Contract;
  private validator: MethodCallValidator;

  constructor(options: Options) {
    const {account, contractId} = options;
    this.validator = new MethodCallValidator(ShipsProjectContract.openrpcDocument);
    const changeMethods:string[] = [
      "increment",
      "register_creator",
      "add_temp_user",
      "reset_temp_user",
      "create_project",
      "create_new_release",
      "new",
    ] 

    const viewMethods:string[] = [
      "is_creator_registered",
      "is_temp_user",
      "get_projects",
      "get_releases",
      "get_count",
    ]
    this.contract = new Contract(account,contractId, {changeMethods, viewMethods})
  }

  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public increment(options?: ChangeMethodOptions): RTIncrement { 
    //return this.request("increment", params);
    const paramNames:string[] = [ 
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).increment({args: paramByName, ...options}) as RTIncrement 
    }

    return (this.contract as any).increment({args: paramByName}) as RTIncrement 
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public is_creator_registered(account_id: AccountId): RTIsCreatorRegistered { 
    //return this.request("is_creator_registered", params);
    const paramNames:string[] = [  
      "account_id",
    ]
    const arrArgs = Array.from(arguments); 
    const options = {}
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).is_creator_registered({args: paramByName, ...options}) as RTIsCreatorRegistered 
    }

    return (this.contract as any).is_creator_registered(paramByName) as RTIsCreatorRegistered 
    
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public register_creator(options?: ChangeMethodOptions): RTRegisterCreator { 
    //return this.request("register_creator", params);
    const paramNames:string[] = [ 
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).register_creator({args: paramByName, ...options}) as RTRegisterCreator 
    }

    return (this.contract as any).register_creator({args: paramByName}) as RTRegisterCreator 
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public add_temp_user(user_id: UserId, access_key: PubKey, options?: ChangeMethodOptions): RTAddTempUser { 
    //return this.request("add_temp_user", params);
    const paramNames:string[] = [  
      "user_id", 
      "access_key",
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).add_temp_user({args: paramByName, ...options}) as RTAddTempUser 
    }

    return (this.contract as any).add_temp_user({args: paramByName}) as RTAddTempUser 
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public reset_temp_user(user_id: UserId, access_key: PubKey, options?: ChangeMethodOptions): RTResetTempUser { 
    //return this.request("reset_temp_user", params);
    const paramNames:string[] = [  
      "user_id", 
      "access_key",
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).reset_temp_user({args: paramByName, ...options}) as RTResetTempUser 
    }

    return (this.contract as any).reset_temp_user({args: paramByName}) as RTResetTempUser 
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public is_temp_user(access_key: PubKey): RTIsTempUser { 
    //return this.request("is_temp_user", params);
    const paramNames:string[] = [  
      "access_key",
    ]
    const arrArgs = Array.from(arguments); 
    const options = {}
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).is_temp_user({args: paramByName, ...options}) as RTIsTempUser 
    }

    return (this.contract as any).is_temp_user(paramByName) as RTIsTempUser 
    
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public create_project(name: Name, uri: Uri, details: Details, options?: ChangeMethodOptions): RTCreateProject { 
    //return this.request("create_project", params);
    const paramNames:string[] = [  
      "name", 
      "uri", 
      "details",
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).create_project({args: paramByName, ...options}) as RTCreateProject 
    }

    return (this.contract as any).create_project({args: paramByName}) as RTCreateProject 
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public get_projects(owner_id: AccountId, paginationOptions?: PaginationOptions): RTGetProjects { 
    //return this.request("get_projects", params);
    const paramNames:string[] = [  
      "owner_id", 
      "paginationOptions",
    ]
    const arrArgs = Array.from(arguments); 
    const options = {}
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).get_projects({args: paramByName, ...options}) as RTGetProjects 
    }

    return (this.contract as any).get_projects(paramByName) as RTGetProjects 
    
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public get_releases(project_id: Id, paginationOptions?: PaginationOptions): RTGetReleases { 
    //return this.request("get_releases", params);
    const paramNames:string[] = [  
      "project_id", 
      "paginationOptions",
    ]
    const arrArgs = Array.from(arguments); 
    const options = {}
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).get_releases({args: paramByName, ...options}) as RTGetReleases 
    }

    return (this.contract as any).get_releases(paramByName) as RTGetReleases 
    
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public create_new_release(project_id: Id, details: ReleaseDetails, terms: ReleaseTerms, options?: ChangeMethodOptions): RTCreateNewRelease { 
    //return this.request("create_new_release", params);
    const paramNames:string[] = [  
      "project_id", 
      "details", 
      "terms",
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).create_new_release({args: paramByName, ...options}) as RTCreateNewRelease 
    }

    return (this.contract as any).create_new_release({args: paramByName}) as RTCreateNewRelease 
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public get_count(): RTGetCount { 
    //return this.request("get_count", params);
    const paramNames:string[] = [ 
    ]
    const arrArgs = Array.from(arguments); 
    const options = {}
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).get_count({args: paramByName, ...options}) as RTGetCount 
    }

    return (this.contract as any).get_count(paramByName) as RTGetCount 
    
  }
  

  /**
   * 
   * 
   */
  // tslint:disable-next-line:max-line-length
  public new(owner_id: OwnerId, verifier_id: VerifierId, count: Count, options?: ChangeMethodOptions): RTNew { 
    //return this.request("new", params);
    const paramNames:string[] = [  
      "owner_id", 
      "verifier_id", 
      "count",
    ]
    const arrArgs = Array.from(arguments); 
    
    const args =  options && Object.keys(options).length ? arrArgs.slice(0, arguments.length-1) : arrArgs
    const paramByName = _.zipObject(paramNames, args);
    if (options && Object.keys(options).length) {
    return (this.contract as any).new({args: paramByName, ...options}) as RTNew 
    }

    return (this.contract as any).new({args: paramByName}) as RTNew 
  }
  
}
export default ShipsProjectContract;
