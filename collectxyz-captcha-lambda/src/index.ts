// for local dev config only
require("dotenv").config();

import cors from "cors";
import express from "express";
import { body, validationResult } from "express-validator";
import sls from "serverless-http";
import { logger, loggerMiddleware } from "./logging";
import { verifyRecaptcha } from "./recaptcha";
import { sign } from "./sign";

const app = express();
app.use(express.json());
app.use(loggerMiddleware);
app.use(
  cors({
    origin: process.env.CORS_ORIGIN,
  })
);
app.post(
  "/verify",
  body("recaptchaToken").isString().notEmpty(),
  body("data").notEmpty(),
  async (req, res) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(422).json({ message: "invalid input" });
    }

    try {
      const seemsHuman = await verifyRecaptcha(req.body.recaptchaToken);
      if (seemsHuman) {
        const signature = await sign(req.body.data);
        res.json({ signature });
      } else {
        res.status(401);
        res.json({ message: "recaptcha verification failed" });
      }
    } catch (internalError) {
      logger.error({ internalError });
      res.status(500);
      res.json({ message: "internal server error" });
    }
  }
);

// if this env var is undefined, we're not running in a lambda
if (!process.env.AWS_LAMBDA_FUNCTION_NAME) {
  app.listen(8000);
}

export const handler = sls(app);
