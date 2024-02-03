"use client";

import { FC, useEffect, useState } from "react";
import { Button, Card, Col, FormControl, Row } from "react-bootstrap";

type LiteralListProps = {
    language: string;
    literals: FormLiteral[];
    defaultTranslation?: FormLiteral[];
    saveChanges?: (literals: FormLiteral[]) => Promise<boolean>;
};

export type FormLiteral = {
    key: string;
    value: string;
};

const LiteralList: FC<LiteralListProps> = (props) => {
    const [literals, setLiterals] = useState(props.literals || []);

    useEffect(() => {
        setLiterals(props.literals);
    }, [props.literals]);

    useEffect(() => {
        const finalLiterals = [...props.literals];
        if (props.defaultTranslation) {
            for (const originalLiteral of props.defaultTranslation) {
                if (!finalLiterals.find(x => x.key === originalLiteral.key)) {
                    finalLiterals.push({ key: originalLiteral.key, value: '' });
                }
            }
        }
        setLiterals(finalLiterals);
    }, [props.defaultTranslation])

    const save = async () => {
        if (props.saveChanges) {
            const result = await props.saveChanges(literals);
        }
    };

    return (
        <Card>
            <Card.Body>
                <div className="d-flex flex-column gap-2">
                    <Row>
                        <Col md={3}>Literal key</Col>
                        {props.defaultTranslation ?
                            <Col md={4}>Default value</Col>
                            : null
                        }
                        <Col>Translation</Col>
                    </Row>

                    {literals.map((literal, index) => 
                        <Row key={`${props.language}_${index}`}>
                            <Col md={3}>
                                <FormControl 
                                    value={literal.key}
                                    onChange={(e) => {
                                        literals[index].key = e.currentTarget.value;
                                        setLiterals([...literals]);
                                    }}
                                    disabled={props.defaultTranslation !== undefined} />
                            </Col> 

                            {props.defaultTranslation !== undefined ?
                                <Col md={4}>
                                    <p>{props.defaultTranslation!.find(translation => translation.key === literal.key)!.value}</p>
                                </Col>
                                : null
                            }

                            <Col>
                                <div className="d-flex gap-2">
                                    <FormControl 
                                        value={literal.value}
                                        onChange={(e) => {
                                            literals[index].value = e.currentTarget.value;
                                            setLiterals([...literals]);
                                        }} />
                                    {props.defaultTranslation === undefined ?
                                        <Button variant="danger">X</Button>
                                        : null
                                    }
                                </div>
                            </Col> 
                        </Row>
                    )}
                </div>

                <div className="mt-2 d-flex flex-row justify-content-between">
                    {props.defaultTranslation === undefined ?
                        <Button variant="primary" onClick={() => setLiterals([...literals, { key: '', value: '' }])}>
                            Add new
                        </Button>
                        : null
                    }
                    
                    <Button variant="success" onClick={save}>
                        Save changes
                    </Button>
                </div>
                
            </Card.Body>
        </Card>
    )
};

export default LiteralList;