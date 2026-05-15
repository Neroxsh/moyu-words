/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./overlay.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        bg: {
          primary: "#0f1117",
          secondary: "#161822",
          card: "#1c1e2e",
          hover: "#242638",
        },
        text: {
          primary: "#eaecf4",
          secondary: "#9da3b5",
          muted: "#636882",
        },
        accent: {
          primary: "#6c8cff",
          hover: "#8ba3ff",
          dim: "#5568d6",
        },
        border: {
          DEFAULT: "#2a2d3f",
          light: "#353852",
        },
        success: "#4ade80",
        warning: "#fbbf24",
        danger: "#f87171",
      },
      backdropBlur: {
        glass: "12px",
      },
      fontFamily: {
        sans: [
          '"Inter"',
          '"SF Pro Display"',
          '-apple-system',
          'BlinkMacSystemFont',
          '"Segoe UI"',
          'sans-serif',
        ],
      },
    },
  },
  plugins: [],
};