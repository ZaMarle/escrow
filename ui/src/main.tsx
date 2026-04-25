import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import {theme} from "./theme.ts";
import { ThemeProvider } from '@mui/material/styles';
import { SolanaProvider } from "./providers/SolanaProvider";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ThemeProvider theme={theme}>
      <SolanaProvider>
        <App />
      </SolanaProvider>
    </ThemeProvider>
  </StrictMode>
)
