const express = require('express');
const fs = require('fs');
const app = express();
const cors = require('cors');
const port = 8010;
const path = require('path');

const isVercel = process.env.VERCEL === '1';
const directory = isVercel ? path.join(__dirname, '../public') : '/workspace/.soroban';

app.use(cors());

app.get('/', (req, res) => {
  res.send('Hello World!');
});

app.get('/api/tokens', (req, res) => {
  const tokensFile = `${directory}/tokens.json`;

  if (fs.existsSync(tokensFile)) {
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/factory', (req, res) => {
  const factoryFile = `${directory}/factory.json`;

  if (fs.existsSync(factoryFile)) {
    return res.sendFile(factoryFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/keys', (req, res) => {
  const keysFile = `${directory}/token_admin_keys.json`;

  if (fs.existsSync(keysFile)) {
    return res.sendFile(keysFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/pairs', (req, res) => {
  const tokensFile = `${directory}/pairs.json`;

  if (fs.existsSync(tokensFile)) {
    return res.sendFile(tokensFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.get('/api/router', (req, res) => {
  const routerFile = `${directory}/router.json`;

  if (fs.existsSync(routerFile)) {
    return res.sendFile(routerFile);
  }

  res.status(404).send({ error: 'file not found' });
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
