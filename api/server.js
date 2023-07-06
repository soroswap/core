const express = require('express');
const fs = require('fs');
const app = express();
const cors = require('cors');
const port = 8010;

app.use(cors());

app.get('/api/tokens', (req, res) => {
  const tokensFile = '/workspace/.soroban/tokens.json';

  if (fs.existsSync(tokensFile)) {
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/factory', (req, res) => {
  const factoryFile = '/workspace/.soroban/factory.json';

  if (fs.existsSync(factoryFile)) {
    return res.sendFile(factoryFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/keys', (req, res) => {
  const keysFile = '/workspace/.soroban/token_admin_keys.json';

  if (fs.existsSync(keysFile)) {
    return res.sendFile(keysFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/pairs', (req, res) => {
  const tokensFile = '/workspace/.soroban/pairs.json';

  if (fs.existsSync(tokensFile)) {
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
