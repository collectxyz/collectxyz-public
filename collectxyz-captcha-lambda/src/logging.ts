import winston, { LoggerOptions } from "winston";
import expressWinston from "express-winston";

export const loggerConfig: LoggerOptions = {
  transports: [new winston.transports.Console()],
  format: winston.format.json(),
};

export const logger = winston.createLogger(loggerConfig);

export const loggerMiddleware = expressWinston.logger(
  loggerConfig as expressWinston.LoggerOptions
);
