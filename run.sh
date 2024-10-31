#set -x
set -e

export TWEET_PROMPT_TEMPLATE=$(curl $TWEET_PROMPT_TEMPLATE_URL)

# Encumber the account by resetting the password

export TUTA_PASSWORD=$(python3 scripts/tutamail.py)
export TWITTER_PASSWORD=$(python3 scripts/twitter.py)

PAYLOAD="{\"report_data\": \"$(echo -n $TWITTER_ACCOUNT | od -A n -t x1 | tr -d ' \n')\"}"
curl -X POST --unix-socket /var/run/tappd.sock -d "$PAYLOAD" http://localhost/prpc/Tappd.TdxQuote?json | jq .

# Start the oauth client to receive the callback
pushd client
RUST_LOG=info cargo run --release --bin helper &
SERVER=$!
popd

# Do the twitter login
python3 scripts/vireh.py
. cookies.env
export X_AUTH_TOKENS
wait $SERVER

# Start the time release server
bash timerelease.sh &

# Update the environment variables
. client/updated.env
export X_ACCESS_TOKEN X_ACCESS_TOKEN_SECRET

pushd client
RUST_LOG=info cargo run --release --bin helper &
popd

# Run the nous
pushd agent
python3 run_pipeline.py
popd
