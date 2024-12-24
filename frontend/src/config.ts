interface Config {
  FIBONACCI_API_URL: string;
  FACTORIAL_API_URL: string;
}

const config: Config = {
  FIBONACCI_API_URL:
    import.meta.env.VITE_FIBONACCI_API_URL || "http://localhost:8080",
  FACTORIAL_API_URL:
    import.meta.env.VITE_FACTORIAL_API_URL || "http://localhost:8000",
};

export default config;
