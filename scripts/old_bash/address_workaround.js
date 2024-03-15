// TODO - remove this workaround when
// https://github.com/stellar/soroban-tools/issues/661 is resolved.

import SorobanClient from 'soroban-client';

let contractId = process.argv[2] || undefined;
if (contractId) {
  console.log(new SorobanClient.Contract(contractId).contractId());
} 
