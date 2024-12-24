import { Link } from "react-router-dom";

export default function Video() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-purple-700 via-indigo-800 to-blue-900 p-8 flex items-center justify-center">
      <nav className="absolute top-8 right-8">
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
            <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" />
            <path
              fillRule="evenodd"
              d="M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z"
              clipRule="evenodd"
            />
          </svg>
          Calculator
        </Link>
      </nav>

      <div className="w-full max-w-3xl">
        <div className="backdrop-blur-sm bg-white/10 rounded-2xl shadow-2xl p-8 border border-white/20 transform transition-all hover:scale-[1.02]">
          <div className="mb-8 text-center">
            <h1 className="text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-300 to-blue-300 mb-2">
              Video Player
            </h1>
            <p className="text-blue-100 opacity-80">
              Watch our featured content
            </p>
          </div>

          <div className="relative aspect-video w-full rounded-xl overflow-hidden shadow-2xl border border-white/20">
            <iframe
              width="100%"
              height="100%"
              src="https://www.youtube.com/embed/aAkMkVFwAoo?si=kKuJTctugE7DFDFt"
              title="YouTube video player"
              frameBorder="0"
              allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
              referrerPolicy="strict-origin-when-cross-origin"
              allowFullScreen
              className="absolute inset-0 w-full h-full"
            ></iframe>
          </div>
        </div>
      </div>
    </main>
  );
}
