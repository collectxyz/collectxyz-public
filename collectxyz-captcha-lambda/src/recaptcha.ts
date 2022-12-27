import axios from "axios";
import { getSecret } from "./aws";
import { logger } from "./logging";

// see: https://developers.google.com/recaptcha/docs/verify#api-response
interface RecaptchaResponse {
  success: boolean;
  challenge_ts: string;
  hostname: string;
  "error-codes": Array<any>;
}

export async function verifyRecaptcha(token: string): Promise<boolean> {
  const secret = await getSecret(process.env.RECAPTCHA_SECRET_ID);

  const response = await axios.request<RecaptchaResponse>({
    method: "post",
    url: "https://www.google.com/recaptcha/api/siteverify",
    params: { secret, response: token },
  });

  if (!response.data.success) {
    logger.error({ recaptchaError: response.data });
  }

  return response.data.success;
}
