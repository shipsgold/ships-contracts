import { keyStores } from "near-api-js";
import os from "os"
import fs from "fs-extra"

export const getDevDeployAccount = async (): Promise<string> => (await fs.readFile(`${__dirname}/../../neardev/dev-account`)).toString('utf-8')
export const getDefaultKeystore =(): keyStores.KeyStore => {
  return new keyStores.UnencryptedFileSystemKeyStore(`${os.homedir()}/.near-credentials`)
}