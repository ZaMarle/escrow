import { createTheme, alpha } from "@mui/material/styles";

const GREEN = "#22ffc8";
const RED = "#f14832";

export const theme = createTheme({
  palette: {
    mode: "dark",

    primary: { main: GREEN },   // brand
    success: { main: GREEN },   // buy
    error: { main: RED },   // sell

    warning: { main: "#f59e0b" },
    info: { main: "#38bdf8" },

    background: {
      default: "#16171d",
      paper: "#1e1f26",
    },

    text: {
      primary: "#e6fff9",
      secondary: alpha(GREEN, 0.7),
      disabled: "rgba(255,255,255,0.4)",
    },

    divider: "rgba(255,255,255,0.08)",
  },

  components: {
    MuiButton: {
      styleOverrides: {
        root: ({ theme }) => ({
          textTransform: "none",
          borderRadius: 6,
          fontWeight: 500,
        }),

        containedPrimary: ({ theme }) => ({
          color: "#0a0b0f",
          backgroundColor: theme.palette.primary.main,
        }),

        containedError: ({ theme }) => ({
          color: "#0a0b0f",
          backgroundColor: theme.palette.error.main,
        }),
      },
    },

    MuiTabs: {
      styleOverrides: {
        indicator: ({ theme }) => ({
          backgroundColor: theme.palette.primary.main,
        }),
      },
    },

    MuiTab: {
      styleOverrides: {
        root: ({ theme }) => ({
          color: theme.palette.text.secondary,
          "&.Mui-selected": {
            color: theme.palette.primary.main,
          },
        }),
      },
    },

    MuiInputLabel: {
      styleOverrides: {
        root: ({ theme }) => ({
          color: theme.palette.text.secondary,
          "&.Mui-focused": {
            color: theme.palette.primary.main,
          },
        }),
      },
    },

    MuiOutlinedInput: {
      styleOverrides: {
        root: ({ theme }) => ({
          backgroundColor: "rgba(255,255,255,0.02)",

          "& .MuiOutlinedInput-notchedOutline": {
            borderColor: "rgba(255,255,255,0.15)",
          },

          "&:hover .MuiOutlinedInput-notchedOutline": {
            borderColor: theme.palette.primary.main,
          },

          "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
            borderColor: theme.palette.primary.main,
            boxShadow: `0 0 0 1px ${alpha(theme.palette.primary.main, 0.3)}`,
          },
        }),

        input: ({ theme }) => ({
          color: theme.palette.text.primary,
        }),
      },
    },
  },
});