import { ConnectionProvider, useWallet, WalletProvider } from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { useEffect, useMemo, type ReactElement } from "react";

function AccountChangeListener() {
    const {wallet} = useWallet();
    
    useEffect(() => {
        const solana = (window as any).solana;
        if(!solana || !wallet) return;

        const handleAccountChange = async () => {
            try {
                await wallet.adapter.disconnect();
                await wallet.adapter.connect();
            } catch (e) {
                console.error("failed to reconnect on account change:", e);
            }
        };

        solana.on("accountChanged", handleAccountChange);
        return () => solana.off("accountChanged", handleAccountChange);
    }), [wallet]

    return null;
}

export const SolanaProvider = ({ children }: { children: ReactElement }) => {
  const endpoint = "http://127.0.0.1:8899";

  const wallets = useMemo(() => {
    return [new PhantomWalletAdapter()];
  }, []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <AccountChangeListener />
          {children}
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};