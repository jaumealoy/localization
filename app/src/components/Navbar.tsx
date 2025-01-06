"use client";

import Navbar from "react-bootstrap/Navbar";
import Container from "react-bootstrap/Container";
import Nav from "react-bootstrap/Nav";
import Link from "next/link";

export default () => {
    return (
        <Navbar sticky="top" bg="dark" data-bs-theme="dark">
			<Container>
				<Nav>
					<Nav.Link as={Link} href="/">Pages</Nav.Link>
					<Nav.Link as={Link} href="/languages">Languages</Nav.Link>
				</Nav>
			</Container>
		</Navbar>
    )
}