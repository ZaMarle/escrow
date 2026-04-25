import { Box, AppBar, Toolbar, Typography, TextField, List, ListItem, ListItemText, Tabs, Tab } from "@mui/material";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";
import '@solana/wallet-adapter-react-ui/styles.css';
import rawTokens  from "@db/tokens.json";
import type { Token } from "./Token";
import { useState } from "react";
import TradeForm from "./components/TradeForm";
import { useTokenBalances } from "./hooks/useTokenBalances";

function App() {
  const tokens: Record<string, Token> = rawTokens;
  const { publicKey } = useWallet();
  const balances = useTokenBalances(tokens);
  const [search, setSearch] = useState("");

  const filteredTokens = Object.entries(tokens).filter(([_, token]) =>
    token.name.toLowerCase().includes(search.toLowerCase()) ||
    token.mint.toLowerCase().includes(search.toLowerCase())
  );

  const [tab, setTab] = useState(0);

  return (
    <Box
      sx={{
        display: "grid",
        gridTemplateRows: "64px 1fr",
        height: "100vh",
      }}
      >
      {/* Navbar */}
      <AppBar position="static" style={{backgroundColor: "#252525", borderBottom: "1px solid #22ffc8", borderTop: "1px solid #22ffc8"}}>
        <Toolbar sx={{ display: "flex", justifyContent: "space-between" }}>
          <Typography variant="h6" style={{color: "#22ffc8"}}>Escrow App</Typography>
          <div>
            <WalletMultiButton />
          </div>
        </Toolbar>
      </AppBar>

      <Box sx={{display:"grid", gridTemplateColumns: "300px 3fr 300px", backgroundColor: "#252525", color: "#22ffc8", borderBottom: "1px solid #22ffc8"}}>
        {/* Left panel */}


        {/* Middle main content */}
        <Box
          sx={{
            display: "grid",
            gridTemplateRows: "3fr 1fr",
            borderRight: "1px solid #22ffc8",
          }}
        >
          <Box sx={{ p: 2, borderBottom: "1px solid #22ffc8" }}>
            Chart
          </Box>

          <Box>
            <Tabs
              value={tab}
              onChange={(_, newValue) => setTab(newValue)}
              textColor="primary"
              indicatorColor="primary"
            >
              <Tab label="Orders" />
              <Tab label="History" />
            </Tabs>

            <Box sx={{ mt: 2 }}>
              {tab === 0 && <Box>Orders</Box>}
              {tab === 1 && <Box>History</Box>}
            </Box>
          </Box>
        </Box>

        {/* Right panel */}
        <Box sx={{ gridTemplateRows: 'auto 1fr' }}>
            <Box sx={{ p: 2, borderBottom: "1px solid #22ffc8" }}>
              <TradeForm />
            </Box>
            <Box>
              depth
            </Box>
        </Box>

      </Box>
    </Box>
  );
}

export default App;