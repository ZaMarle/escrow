import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { PublicKey } from "@solana/web3.js";
import { useEffect, useState } from "react";
import type { Token } from "../Token";

export function useTokenBalances(tokens: Record<string, Token>): Record<string, number | null> {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balances, setBalances] = useState<Record<string, number | null>>({});

  useEffect(() => {
    if (!publicKey) {
      setBalances({});
      return;
    }

    const fetchBalances = async () => {
      const result: Record<string, number | null> = {};

      await Promise.all(
        Object.values(tokens).map(async (token) => {
          try {
            const accounts = await connection.getParsedTokenAccountsByOwner(publicKey, {
              mint: new PublicKey(token.mint),
            });

            if (accounts.value.length > 0) {
              result[token.mint] = accounts.value[0].account.data.parsed.info.tokenAmount.uiAmount;
            } else {
              result[token.mint] = 0;
            }
          } catch {
            result[token.mint] = null;
          }
        })
      );

      setBalances(result);
    };

    fetchBalances();
  }, [publicKey, connection, tokens]);

  return balances;
}
