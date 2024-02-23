import cors from "cors";
import express from "express";
import { existsSync } from "fs";
import { join } from "path";
const app = express();
const port = 8010;

const isVercel = process.env.VERCEL === "1";
const directory = isVercel
  ? join(__dirname, "..", "..", "public")
  : "/workspace/.soroban";

app.use(cors());

app.get("/", (req, res) => {
  res.send(`healthy`);
});

app.get("/api/tokens", (req, res) => {
  const tokensFile = join(directory, "tokens.json");
  if (existsSync(tokensFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: "file not found" });
});

app.get("/api/random_tokens", (req, res) => {
  const tokensFile = join(directory, "random_tokens.json");
  if (existsSync(tokensFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: "file not found" });
});

app.get("/api/factory", (req, res) => {
  const factoryFile = `${directory}/factory.json`;

  if (existsSync(factoryFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(factoryFile);
  }

  res.status(404).send({ error: "file not found" });
});

app.get("/api/keys", (req, res) => {
  const keysFile = `${directory}/token_admin_keys.json`;

  if (existsSync(keysFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(keysFile);
  }

  res.status(404).send({ error: "file not found" });
});

app.get("/api/pairs", (req, res) => {
  const tokensFile = `${directory}/pairs.json`;

  if (existsSync(tokensFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: "file not found" });
});

app.get("/api/router", (req, res) => {
  const routerFile = `${directory}/router.json`;

  if (existsSync(routerFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(routerFile);
  }

  res.status(404).send({ error: "file not found" });
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
