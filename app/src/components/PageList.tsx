"use client";

import { FC, FormEvent, useState } from "react";
import Button from "react-bootstrap/Button";
import Modal from "react-bootstrap/Modal";
import Form from "react-bootstrap/Form";
import { FormControl, FormGroup } from "react-bootstrap";
import Link from "next/link";
import { useRouter } from "next/navigation";

type Page = string;
type PageListProps = {
    defaultLanguage: string;
    value?: Page[];
    onCreatePage?: (page: Page) => Promise<boolean>;
};

const PageList: FC<PageListProps> = (props) => {
    const [isModalVisible, setModalVisible] = useState(false);
    const [pageId, setPageId] = useState("");
    const router = useRouter()

    const onSubmitForm = (event: FormEvent) => {
        event.preventDefault();

        if (props.onCreatePage) {
            props.onCreatePage(pageId)
                .then(result => {
                    if (result) {
                        router.replace(`/${pageId}?language=${props.defaultLanguage}`);
                        setPageId("");
                    }
                });
        }
    }

    return (
        <>
            <ul>
                {props.value?.map((page, index) => 
                    <li key={index}>
                        <Link href={`/${page}?language=${props.defaultLanguage}`}>
                            {page}
                        </Link>
                    </li>    
                )}
            </ul>

            <div className="d-grid">
                <Button onClick={() => setModalVisible(true)}>
                    Add new page
                </Button>
            </div>

            <Modal show={isModalVisible} onHide={() => setModalVisible(false)}>
                    <Modal.Header closeButton>Add a new page</Modal.Header>
                    <Modal.Body>
                        <Form onSubmit={onSubmitForm}>
                            <FormGroup>
                                <label>Page identifier:</label>
                                <FormControl 
                                    value={pageId}
                                    onChange={(event) => setPageId(event.currentTarget.value)}
                                    type="text"
                                    required />
                            </FormGroup>

                            <div className="d-flex justify-content-end mt-2">
                                <Button type="submit">
                                    Create page
                                </Button>
                            </div>
                        </Form>
                    </Modal.Body>
            </Modal>
        </>
    );
};

export default PageList;