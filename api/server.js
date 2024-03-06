const express = require("express");
const fs = require("fs");
const app = express();
const cors = require("cors");
const port = 8010;
const path = require("path");

const isVercel = process.env.VERCEL === "1";
const directory = isVercel
  ? path.join(__dirname, "..", "public")
  : "/workspace/.soroban";

  
app.use(cors());

app.get("/", (req, res) => {
  res.send(`healthy`);
});
  
app.get("/api/tokens", (req, res) => {
  const tokensFile = path.join(directory, "tokens.json");
  if (fs.existsSync(tokensFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: "file not found" });
});
  
app.get("/api/random_tokens", (req, res) => {
  const tokensFile = path.join(directory, "random_tokens.json");
  if (fs.existsSync(tokensFile)) {
    res.set("Cache-Control", "no-store");
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: "file not found" });
});
  
app.get("/api/:network/:contractName", (req, res) => {
  const { network, contractName } = req.params;
  const contractsFile = `${directory}/${network}.contracts.json`;

  if (fs.existsSync(contractsFile)) {
    const contractsData = JSON.parse(fs.readFileSync(contractsFile, 'utf8'));

    // Check if the contract name exists in the "ids" section
    if (contractsData.ids && contractsData.ids[contractName]) {
      res.set("Cache-Control", "no-store");
      // Return the contract address
      return res.send({ address: contractsData.ids[contractName] });
    } else {
      // Contract name not found in the file
      return res.status(404).send({ error: "contract name not found" });
    }
  }
  
    // File not found
    res.status(404).send({ error: "file not found" });
  });
  
app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
  
