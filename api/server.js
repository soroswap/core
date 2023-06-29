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

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
