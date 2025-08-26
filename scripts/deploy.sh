source .env.local

echo "Building program..."
anchor build

echo "Deploying to devnet..."
anchor deploy --provider.cluster devnet

PROGRAM_ID=$(solana address -k target/deploy/habitdao-keypair.json)
echo "Program deployed at: $PROGRAM_ID"

*# Update Anchor.toml with program ID if needed*

sed -i "s/habitdao = \".*\"/habitdao = \"$PROGRAM_ID\"/" Anchor.toml

echo "Deployment complete!"
echo "Program ID: $PROGRAM_ID"
echo "Token Mint: $HABIT_TOKEN_MINT"