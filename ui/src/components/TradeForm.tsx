import { Box, TextField, Button, ToggleButton, ToggleButtonGroup } from "@mui/material";
import { useForm, Controller } from "react-hook-form";
import { useState } from "react";

type FormValues = {
  amount: number;
  price: number;
};

interface TradeFormProps {
    token: string
}

export default function TradeForm() {
  const { control, handleSubmit } = useForm<FormValues>({
    defaultValues: {
      amount: 0,
      price: 0,
    },
  });

  const [side, setSide] = useState<"buy" | "sell">("buy");

  const onSubmit = (data: FormValues) => {
    console.log("ORDER:", {
      side,
      ...data,
    });

    // later:
    // send to program / API
  };

  return (
    <Box
      component="form"
      onSubmit={handleSubmit(onSubmit)}
      sx={{ display: "flex", flexDirection: "column", gap: 2, maxWidth: 300 }}
    >
    {/* Buy / Sell */}
    <Box sx={{ display: "flex", gap: 1, width: "100%" }}>
        <Button
            fullWidth
            variant={side === "buy" ? "contained" : "outlined"}
            color="success"
            onClick={() => setSide("buy")}
        >
            Buy
        </Button>

        <Button
            fullWidth
            variant={side === "sell" ? "contained" : "outlined"}
            color="error"
            onClick={() => setSide("sell")}
        >
            Sell
        </Button>
    </Box>

      {/* Amount */}
      <Controller
        name="amount"
        control={control}
        rules={{ required: true, min: 0 }}
        render={({ field }) => (
          <TextField
            {...field}
            label="Amount"
            type="number"
            fullWidth
            size="small"
          />
        )}
      />

      {/* Price */}
      <Controller
        name="price"
        control={control}
        rules={{ required: true, min: 0 }}
        render={({ field }) => (
          <TextField
            {...field}
            label="Price"
            type="number"
            fullWidth
            size="small"
          />
        )}
      />

      {/* Submit */}
      <Button
        type="submit"
        variant="contained"
        color={side === "buy" ? "success" : "error"}
      >
        {side === "buy" ? "Place Buy Order" : "Place Sell Order"}
      </Button>
    </Box>
  );
}