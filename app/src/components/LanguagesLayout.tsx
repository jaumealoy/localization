"use client";

import { Language } from "@/client/TranslationClient";
import { FC, useState } from "react";
import { Button, Form, FormCheck, FormControl, FormGroup, InputGroup } from "react-bootstrap";
import Card from "react-bootstrap/Card";
import Container from "react-bootstrap/Container";

type LanguageLayoutProps = {
    languages: Language[];
    onSaveChanges?: (languages: Language[]) => Promise<boolean>;
};

type ExtendedLanguage = Language & { created: boolean; }

const Layout: FC<LanguageLayoutProps> = (props) => {
    const [languages, setLanguages] = useState<ExtendedLanguage[]>(props.languages.map(x => ({ ...x, created: true })) || []);

    const setDefault = (language: string) => {
        setLanguages(languages.map(x => ({
            ...x,
            default: x.code == language
        })));
    };

    const changeCode = (index: number, value: string) => {
        languages[index].code = value;
        setLanguages([...languages]);
    };
    
    const saveHandler = () => {
        if (props.onSaveChanges) {
            const mappedLanguages = languages.map(x => ({
                code: x.code,
                default: x.default
            }));

            props.onSaveChanges(mappedLanguages)
                .then(result => {
                    if (result) {
                        setLanguages(languages.map(x => ({ ...x, created: true })));
                    }
                })
        }
    }

    return (
        <Container className="my-3">
            <Card>
                <Card.Header>Languages</Card.Header>
                <Card.Body>
                    <p>Manage the available languages available. Make sure that there is always a default language.</p>

                    <Form>
                        <div className="d-flex flex-column gap-2">
                            {languages.map((language, index) => 
                                <InputGroup>
                                    <InputGroup.Text>
                                        <FormCheck 
                                            onChange={(e) => {
                                                if (e.currentTarget.value === 'on') {
                                                    setDefault(language.code);
                                                }
                                            }}
                                            checked={language.default} />
                                    </InputGroup.Text>
                                    <FormControl 
                                        value={language.code}
                                        onChange={e => changeCode(index, e.currentTarget.value)}
                                        disabled={language.created} />
                                </InputGroup>    
                            )}
                        </div>

                        <div className="mt-2 d-flex justify-content-between">
                            <Button onClick={() => setLanguages([...languages, { code: '', default: false, created: false}])}>
                                Add new language
                            </Button>

                            <Button variant="success" onClick={saveHandler}>
                                Save changes
                            </Button>
                        </div>
                        
                    </Form>
                </Card.Body>
            </Card>
        </Container>
    );
};

export default Layout;