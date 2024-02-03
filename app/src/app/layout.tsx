import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.scss";
import Navbar from "@/components/Navbar";
import { SSRProvider } from "react-bootstrap";



const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
	title: "Translations"
};

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en" data-bs-theme="light">
			<body className={inter.className}>
				<Navbar />
				{children}
			</body>
		</html>
	);
}
