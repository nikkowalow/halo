"use client";
import { useState } from "react";
import '../styles/fonts.css';

export default function Home() {
  const [loading, setLoading] = useState(false);

  const handleClick = async () => {
    setLoading(true);
    try {
      const response = await fetch("http://127.0.0.1:8080/users");
      if (!response.ok) throw new Error("Failed to fetch");
      const result = await response.json();
      console.log("API response:", result);
    } catch (err) {
      console.error("Error:", err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="min-h-screen bg-black text-white relative overflow-hidden">
      <nav className="w-full px-6 py-4 flex justify-between items-center border-b border-gray-800 z-10 bg-black/70 backdrop-blur-md fixed top-0">
        <h1 className="text-2xl font-bold tracking-widest">HALO</h1>
        <ul className="flex gap-6 text-sm text-gray-400">
          <li className="hover:text-white cursor-pointer">About</li>
          <li className="hover:text-white cursor-pointer">Events</li>
          <li className="hover:text-white cursor-pointer">Contact</li>
        </ul>
      </nav>

      {/* === HERO SECTION === */}
      <section className="flex flex-col items-center justify-center text-center pt-32 pb-20 px-6">
        <h2 className="text-6xl sm:text-7xl font-extrabold mb-2 font-blur tracking-tight">Host. Sell. Simplify.</h2>
        <p className="text-sm text-green-400 mb-6 italic">Coming soon...</p>

        <p className="text-gray-400 max-w-xl text-lg mb-10">
          Empower your events with secure ticketing and seamless hosting — powered by HALO.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 mb-6">
          <button
            onClick={handleClick}
            disabled={loading}
            className="bg-green-500 hover:bg-green-600 text-black font-semibold px-6 py-3 rounded-lg transition-all disabled:opacity-50"
          >
            {loading ? "Loading..." : "Fetch Users"}
          </button>
          <button className="border border-white px-6 py-3 rounded-lg hover:bg-white hover:text-black transition-all">
            Create Event
          </button>
        </div>

        <a
          href="#"
          className="bg-white text-black px-6 py-3 rounded-lg font-semibold hover:bg-gray-200 transition-all mt-2 text-sm"
        >
          📱 Download Mobile App
        </a>

        <div className="relative mt-16 w-full max-w-4xl h-64 sm:h-80 bg-gradient-to-br from-green-400/20 to-blue-500/10 rounded-6xl blur-xl opacity-50 z-0"></div>
        <img
          src="/images/ring.png"
          alt="Abstract Visual"
          className="absolute top-20 right-[-25px] w-[600px] opacity-50 pointer-events-none"
        />
      </section>

      <footer className="w-full border-t border-gray-800 px-6 py-8 text-sm text-gray-500 text-center space-y-2 bg-black z-10">
        <div className="flex flex-col sm:flex-row justify-center gap-4 mb-2">
          <a href="#" className="hover:text-white">Licensing</a>
          <a href="#" className="hover:text-white">Privacy Policy</a>
          <a href="#" className="hover:text-white">Terms of Service</a>
        </div>
        <p className="text-xs">&copy; {new Date().getFullYear()} Halo Events. Built with passion.</p>
      </footer>
    </main>
  );
}
