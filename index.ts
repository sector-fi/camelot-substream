import "reflect-metadata";
import express, { Application, Request, Response } from "express";
import { PrismaClient } from "@prisma/client";
import expressPlayground from "graphql-playground-middleware-express";
import { graphqlHTTP } from "express-graphql";
import { buildSchemaSync } from "type-graphql";
import { resolvers } from "./prisma/generated/type-graphql";

const app: Application = express();
const prisma = new PrismaClient();

app.use("/playground", expressPlayground({ endpoint: "/graphql" }));

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

const schema = buildSchemaSync({
  resolvers,
});

app.use(
  "/graphql",
  graphqlHTTP({
    schema,
    customFormatErrorFn: (err) => {
      // const error = getErrorCode(err.message);
      console.log(err);
      return err;
    },
  })
);

const port: number = 3001;

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
