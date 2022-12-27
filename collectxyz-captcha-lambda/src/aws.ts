import aws from "aws-sdk";
import { memoize } from "lodash";

const secrets = new aws.SecretsManager({ region: process.env.AWS_REGION });

export const getSecret = memoize(async (secretId: string): Promise<string> => {
  const result = await secrets.getSecretValue({ SecretId: secretId }).promise();
  return result.SecretString!;
});
