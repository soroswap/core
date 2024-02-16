# Soroswap Aggregator Documentation

This document provides essential information about the Soroswap Aggregator and associated scripts, designed to facilitate deployment, initialization, and testing of the aggregator alongside various protocols.

## Overview

The `deploy_initialize_aggregator.sh` script is responsible for deploying the aggregator and initializing it with a previously deployed router address. This script is executed within `populate_network.sh`, which serves as the primary deployment script for Soroswap.

### Testing and Protocols

Within the `/protocols` directory, you'll find scripts for deploying additional protocols and testing their integration with the aggregator. These scripts are intended for standalone use but are also applicable to other networks. For testnet deployments, it is recommended to utilize the contracts deployed by the protocol itself for real-world use cases, rather than deploying new instances for testing purposes.

## Scripts Description

- **`get_admin.sh`**: Retrieves the current admin of the aggregator.
- **`get_protocols.sh`**: Fetches the protocols currently available within the aggregator.
- **`update_protocols.sh`**: Updates the aggregator with newly deployed protocols (e.g., Phoenix).
- **`swap_test.sh`**: Tests the swap functionality, currently configured for Phoenix.

## Usage Guide

To begin, deploy the necessary components by executing the `populate_network.sh` script. Should you wish to test the swap functionality with Phoenix, proceed as follows:

1. **Deploy Phoenix Contracts**: Run `bash scripts/aggregator/protocols/phoenix_deploy.sh standalone` to deploy Phoenix contracts.
2. **Update Aggregator Protocols**: Execute `bash scripts/aggregator/update_protocols.sh` to register the Phoenix multihop's address with the aggregator.
3. **Verify Protocol Registration**: Check the updated protocols list by running `bash scripts/aggregator/get_protocols.sh`, which should output JSON data similar to the following:

```json
[
  {
    "address": "CBYSEFB3KWLS3RMRPU5YCT7BNRQ5RF3VILWTCP4KLZOFVKLS5EXHEE22",
    "protocol_id": 0
  },
  {
    "address": "CDFD53B7TZBYNN3EYQEZPFQRESJOHNVPCRN3F4QCDIKLYZE4TAVWW4P7",
    "protocol_id": 1
  }
]
```

Here, `protocol_id` 0 refers to Soroswap, and 1 refers to Phoenix.

4. **Testing Swap Functionality**: With Phoenix integrated, you can now run `swap_test.sh` to simulate a token swap operation. This script will create a new user account, mint tokens to it, and perform a swap using the aggregator.

## Submodule Initialization
If you find the `contracts/aggregator/protocols/phoenix/` directory empty after cloning the repository, it indicates that the submodules, including Phoenix, have not been initialized. To properly set up the Phoenix submodule, execute the following command from the root of your repository:

```bash
git submodule update --init
```

This command initializes and fetches the content of the Phoenix submodule along with any other submodules configured within the repository. It's a crucial step to ensure that all necessary code, including external protocols or contracts, is correctly downloaded and available for use in your local development environment.


### Additional Notes

- Ensure all dependencies and environment variables are set before running the scripts.
- Review each script for any required parameters or configuration adjustments specific to your deployment environment.
