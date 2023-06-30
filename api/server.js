const express = require('express');
const fs = require('fs');
const app = express();
const cors = require('cors');
const port = 8010;

app.use(cors());

app.get('/api/tokens', (req, res) => {
  const tokensFile = '/workspace/.soroban/tokens.json' 

  if (fs.existsSync(tokensFile)) {
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ 'error': 'file not found'})
});

app.get('/api/factory', (req, res) => {
  const factoryFile = '/workspace/.soroban/factory.json' 

  if (fs.existsSync(factoryFile)) {
    return res.sendFile(factoryFile);
  }

  res.status(404).send({ 'error': 'file not found'})
});

app.get('/api/keys', (req, res) => {
  const addressFile = '/workspace/.soroban/token_admin_address' 
  const secretFile = '/workspace/.soroban/token_admin_secret' 
  
  if (fs.existsSync(addressFile) && fs.existsSync(secretFile)) {
    const address = fs.readFileSync(addressFile, 'utf8').trim();
    const secret = fs.readFileSync(secretFile, 'utf8').trim();
    return res.json({ admin_public: address, admin_secret: secret });
  }

  res.status(404).send({ 'error': 'file not found'})
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
