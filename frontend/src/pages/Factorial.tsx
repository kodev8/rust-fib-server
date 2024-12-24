import { useState } from "react";
import { Link } from "react-router-dom";

interface FactorialResponse {
  message: string;
  result: string;
  cached: boolean;
}

// Get API URL from environment variable or fallback to localhost
const API_URL = "http://localhost:8000";

export default function Factorial() {
  const [number, setNumber] = useState<string>("");
  const [result, setResult] = useState<FactorialResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const calculateFactorial = async () => {
    if (!number || isNaN(Number(number)) || Number(number) < 0) {
      setError("Please enter a valid non-negative number");
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const response = await fetch(
        `${API_URL}/factorial?num=${parseInt(number)}`,
        {
          method: "GET",
        }
      );
      if (!response.ok) {
        throw new Error("Failed to calculate Factorial number");
      }
      const data: FactorialResponse = await response.json();
      setResult(data);
    } catch (err) {
      console.error(err);
      setError(
        "Failed to fetch result. Make sure the backend server is running."
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="min-h-screen bg-gradient-to-br from-purple-700 via-indigo-800 to-blue-900 p-8 flex items-center justify-center">
      {/* Navigation */}
      <nav className="absolute top-8 right-8 flex gap-4">
        <Link
          to="/tony"
          className="text-white/80 hover:text-white flex items-center gap-2
                     px-4 py-2 rounded-lg bg-white/10 backdrop-blur-sm
                     hover:bg-white/20 transition-all border border-white/20"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path d="M2 6a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zM14.553 7.106A1 1 0 0014 8v4a1 1 0 00.553.894l2 1A1 1 0 0018 13V7a1 1 0 00-1.447-.894l-2 1z" />
          </svg>
          Video
        </Link>
        <Link
          to="/"
          className="text-white/80 hover:text-white flex items-center gap-2
                     px-4 py-2 rounded-lg bg-white/10 backdrop-blur-sm
                     hover:bg-white/20 transition-all border border-white/20"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z" />
          </svg>
          Home
        </Link>
      </nav>

      <div className="w-full max-w-md">
        <div className="backdrop-blur-sm bg-white/10 rounded-2xl shadow-2xl p-8 border border-white/20 transform transition-all hover:scale-[1.02]">
          <div className="mb-8 text-center">
            <h1 className="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-300 to-blue-300 mb-2">
              Factorial Calculator
            </h1>
            <p className="text-blue-100 opacity-80">
              Calculate any Factorial number instantly
            </p>
          </div>

          <div className="space-y-6">
            <div className="relative group">
              <label
                htmlFor="number"
                className="block text-sm font-medium text-blue-100 mb-2 ml-1"
              >
                Enter a number
              </label>
              <input
                id="number"
                type="number"
                value={number}
                onChange={(e) => setNumber(e.target.value)}
                className="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-xl 
                          text-white placeholder-blue-200/50 focus:ring-2 focus:ring-purple-500 
                          focus:border-transparent outline-none transition-all
                          backdrop-blur-sm group-hover:bg-white/20"
                placeholder="Enter a non-negative number"
              />
            </div>

            <button
              onClick={calculateFactorial}
              disabled={loading}
              className={`w-full py-3 px-4 rounded-xl font-semibold text-white
                relative overflow-hidden transition-all duration-300
                ${
                  loading
                    ? "bg-gray-600 cursor-not-allowed"
                    : "bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-500 hover:to-blue-500"
                }
                transform hover:scale-[1.02] active:scale-[0.98]
                shadow-lg hover:shadow-xl`}
            >
              <span className="relative z-10">
                {loading ? (
                  <div className="flex items-center justify-center gap-2">
                    <div className="w-5 h-5 border-t-2 border-b-2 border-white rounded-full animate-spin" />
                    Calculating...
                  </div>
                ) : (
                  "Calculate Factorial"
                )}
              </span>
            </button>

            {error && (
              <div className="p-4 bg-red-500/10 border border-red-500/20 rounded-xl backdrop-blur-sm">
                <p className="text-red-300">{error}</p>
              </div>
            )}

            {result && !error && (
              <div
                className="p-6 bg-green-500/10 border border-green-500/20 rounded-xl backdrop-blur-sm
                            transform transition-all animate-fadeIn"
              >
                <p className="text-green-300 text-sm">{result.message}</p>
                <div className="mt-2 font-mono">
                  <p className="text-sm text-blue-200 mb-1">Result:</p>
                  <p className="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-green-300 to-blue-300">
                    {result.result}
                  </p>
                  <p className="text-sm text-blue-200 mt-2">
                    {result.cached ? "Result from cache" : "Freshly calculated"}
                  </p>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
