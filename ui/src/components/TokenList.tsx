import { Box, List, ListItem, ListItemText, TextField } from '@mui/material'
import type { Token } from '../Token';
import { useState } from 'react';
import { useTokenBalances } from '../hooks/useTokenBalances';
import { useWallet } from '@solana/wallet-adapter-react';

interface props {
    tokens: Record<string, Token>
}

function TokenList({ tokens }: props) {
    const [search, setSearch] = useState("");

    const filteredTokens = Object.entries(tokens).filter(([_, token]) =>
        token.name.toLowerCase().includes(search.toLowerCase()) ||
        token.mint.toLowerCase().includes(search.toLowerCase())
    );

    const balances = useTokenBalances(tokens);
    const { publicKey } = useWallet();

    return (
        <Box sx={{ borderRight: "solid 1px #22ffc8" }}>
            <Box sx={{ p: 1 }}>
                <TextField
                    fullWidth
                    size="small"
                    placeholder="Search tokens..."
                    value={search}
                    onChange={(e) => setSearch(e.target.value)}
                    sx={{ input: { color: "#fff" } }}
                />
            </Box>

            <List>
                {filteredTokens.map(([key, token]) => (
                    <ListItem
                        key={key}
                        sx={{
                            background: "#333",
                            borderRadius: 1,
                            cursor: "pointer",
                            "&:hover": { background: "#444" },
                        }}
                    >
                        <ListItemText
                            primary={token.name}
                            secondary={publicKey ? `Balance: ${balances[token.mint] ?? "…"}` : undefined}
                        />
                    </ListItem>
                ))}
            </List>
        </Box>
    )
}

export default TokenList