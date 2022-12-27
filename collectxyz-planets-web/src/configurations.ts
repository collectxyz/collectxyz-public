export enum Environments {
  Dev = 'dev',
  Stage = 'stage',
  Prod = 'prod'
}

export interface EnvConfig {
  ENVIRONMENT: Environments
}

export const defaultEnvConfig: EnvConfig = {
  ENVIRONMENT: Environments.Dev,
}
