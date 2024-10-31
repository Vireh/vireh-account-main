#!/bin/bash

# Convert human-readable time to Unix timestamp
TIMEOUT_READABLE=$(date -d '+2 days')
timeout_unix=$(date -d "$TIMEOUT_READABLE" +%s)

# Continuously check until the target time is reached
while true; do
    # Fetch the latest block's timestamp
    current_timestamp=$(curl -X POST https://api.mainnet-beta.solana.com \
                                -H "Content-Type: application/json" \
                                -d '{
                                    "jsonrpc": "2.0",
                                    "id": 1,
                                    "method": "getLatestBlockhash",
                                    "params": [
                                        {
                                            "commitment": "finalized"
                                        }
                                    ]
                                }' | jq '.result.blockhash' | xargs -I {} curl -X POST https://api.mainnet-beta.solana.com \
                                -H "Content-Type: application/json" \
                                -d "{
                                    \"jsonrpc\": \"2.0\",
                                    \"id\": 1,
                                    \"method\": \"getBlockTime\",
                                    \"params\": [\"{}\"]
                                }")

    # Check if current Solana timestamp has reached the timeout
    if (( current_timestamp >= timeout_unix )); then
        echo "Time release triggered!"
	cat agent/mykey.hex
	echo TWITTER_PASSWORD $TWITTER_PASSWORD
	echo X_PASSWORD $X_PASSWORD
	echo X_AUTH_TOKENS $X_AUTH_TOKENS
	echo TUTAMAIL_PASSWORD $TUTAMAIL_PASSWORD
        break
    else
        echo "Not yet. Waiting until $TIMEOUT_READABLE (Current Solana time: $(date -d @$current_timestamp))"
    fi

    # Wait before checking again
    sleep 600
done
