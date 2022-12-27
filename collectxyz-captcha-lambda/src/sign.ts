import crypto from "crypto";
import { getSecret } from "./aws";

export async function sign(data: any): Promise<string> {
  const privateKey = await getSecret(process.env.PRIVATE_KEY_SECRET_ID);

  const signature = crypto.sign("sha256", Buffer.from(JSON.stringify(data)), {
    key: privateKey,
    padding: crypto.constants.RSA_PKCS1_PADDING,
  });

  return signature.toString("base64");
}
