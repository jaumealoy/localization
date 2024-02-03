"use client";

import Navbar from "react-bootstrap/Navbar";
import Container from "react-bootstrap/Container";
import Nav from "react-bootstrap/Nav";

export default () => {
    return (
        <Navbar sticky="top" bg="dark" data-bs-theme="dark">
			<Container>
				<Nav>
					<Nav.Link href="/">Pages</Nav.Link>
					<Nav.Link href="/languages">Languages</Nav.Link>
				</Nav>
			</Container>
		</Navbar>
    )
}