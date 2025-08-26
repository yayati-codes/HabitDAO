solana config set --url devbnet 

echo "Creating Token-2022 mint with metadata..."
MINT=$(spl-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb create-token \
--decimals 6 \
--enable-metadata \
--output json | jq -r '.mint')

echo "Mint created: $MINT"

echo "Setting token metadata..."
spl-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb initialize-metadata \
$MINT "HabitDAO Token" "HABIT" "https://habitdao.com/token-metadata.json"

echo "HABIT_TOKEN_MINT=$MINT" > .env.local
echo "TOKEN_PROGRAM_ID=TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb" >> .env.local

echo "Token setup complete. Mint: $MINT"
echo "Saved to .env.local"