import "./globals.css";
import { Inter } from "next/font/google";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "Halo | Event Ticketing Platform",
  description: "Seamless event creation and ticketing in one app",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="bg-black text-white">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
