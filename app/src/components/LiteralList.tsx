"use client";

import { FC, useEffect, useState } from "react";
import { Button, Card, Col, FormControl, Row } from "react-bootstrap";
import LiteralRow from "./LiteralRow";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";

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
        if (!finalLiterals.find((x) => x.key === originalLiteral.key)) {
          finalLiterals.push({ key: originalLiteral.key, value: "" });
        }
      }
    }
    setLiterals(finalLiterals);
  }, [props.defaultTranslation]);

  const save = async () => {
    if (props.saveChanges) {
      const result = await props.saveChanges(literals);
    }
  };

  const removeLiterals = (indexed: number) => {
    const newLiterals = literals.filter((_, index) => index !== indexed);
    setLiterals(newLiterals);
  };

  return (
    <DndProvider backend={HTML5Backend}>
      <Card>
        <Card.Body>
          <div className="d-flex flex-column gap-2">
            <Row>
              <Col md={3}>Literal key</Col>
              {props.defaultTranslation ? (
                <Col md={4}>Default value</Col>
              ) : null}
              <Col>Translation</Col>
            </Row>

            {literals.map((literal, index) => (
              <LiteralRow
                index={index}
                moveCard={(dragIndex: number, hoverIndex: number) => {
                  setLiterals((prev) => {
                    if (dragIndex === undefined || hoverIndex === undefined) {
                      return prev;
                    }
                    const newLiterals = [...prev];
                    const dragCard = newLiterals[dragIndex];
                    newLiterals[dragIndex] = newLiterals[hoverIndex];
                    newLiterals[hoverIndex] = dragCard;
                    return newLiterals;
                  });
                }}
                key={`${props.language}_${index}`}
                defaultTranslation={
                  props.defaultTranslation?.find(
                    (translation) => translation.key === literal.key
                  )?.value
                }
                onChange={(key: string, value: string) => {
                  literals[index].key = key;
                  literals[index].value = value;
                  setLiterals([...literals]);
                }}
                onDelete={() => removeLiterals(index)}
                value={literal}
              />
            ))}
          </div>

          <div className="mt-2 d-flex flex-row justify-content-between">
            {props.defaultTranslation === undefined ? (
              <Button
                variant="primary"
                onClick={() =>
                  setLiterals([...literals, { key: "", value: "" }])
                }
              >
                Add new
              </Button>
            ) : null}

            <Button variant="success" onClick={save}>
              Save changes
            </Button>
          </div>
        </Card.Body>
      </Card>
    </DndProvider>
  );
};

export default LiteralList;
