import { Button, Col, FormControl, Row } from "react-bootstrap";
import { useDrag, useDrop, XYCoord } from "react-dnd";
import {} from "dnd-core";
import { useRef } from "react";

interface DragItem {
  index: number;
  id: string;
  type: string;
}

export default function LiteralRow({
  onDelete,
  onChange,
  defaultTranslation,
  value,
  moveCard,
  index,
}) {
  const [{ handlerId }, drop] = useDrop<
    DragItem,
    void,
    { handlerId: string | null }
  >({
    accept: "card",
    collect(monitor) {
      return {
        handlerId: monitor.getHandlerId(),
      };
    },
    hover(item: DragItem, monitor) {
      if (!ref.current) {
        return;
      }
      const dragIndex = item.index;
      const hoverIndex = index;

      // Don't replace items with themselves
      if (dragIndex === hoverIndex) {
        return;
      }

      // Determine rectangle on screen
      const hoverBoundingRect = ref.current?.getBoundingClientRect();

      // Get vertical middle
      const hoverMiddleY =
        (hoverBoundingRect.bottom - hoverBoundingRect.top) / 2;

      // Determine mouse position
      const clientOffset = monitor.getClientOffset();

      // Get pixels to the top
      const hoverClientY = (clientOffset as XYCoord).y - hoverBoundingRect.top;

      // Only perform the move when the mouse has crossed half of the items height
      // When dragging downwards, only move when the cursor is below 50%
      // When dragging upwards, only move when the cursor is above 50%

      // Dragging downwards
      if (dragIndex < hoverIndex && hoverClientY < hoverMiddleY) {
        return;
      }

      // Dragging upwards
      if (dragIndex > hoverIndex && hoverClientY > hoverMiddleY) {
        return;
      }

      // Time to actually perform the action
      moveCard(dragIndex, hoverIndex);

      // Note: we're mutating the monitor item here!
      // Generally it's better to avoid mutations,
      // but it's good here for the sake of performance
      // to avoid expensive index searches.
      item.index = hoverIndex;
    },
  });

  const [{ isDragging }, drag] = useDrag({
    type: "card",
    item: () => {
      return { id: value.key };
    },
    collect: (monitor: any) => ({
      isDragging: monitor.isDragging(),
    }),
  });

  const ref = useRef<HTMLDivElement | null>(null);

  drag(drop(ref));

  return (
    <Row ref={ref} data-handler-id={handlerId}>
      <Col md={3}>
        <div className="d-flex gap-2">
          {defaultTranslation === undefined && (
            <Button variant={isDragging ? "danger" : "primary"}>-</Button>
          )}
          <FormControl
            value={value.key}
            onChange={(e) => {
              onChange(e.currentTarget.value, value.value);
            }}
            disabled={defaultTranslation !== undefined}
          />
        </div>
      </Col>

      {defaultTranslation !== undefined ? (
        <Col md={4}>
          <p>{defaultTranslation}</p>
        </Col>
      ) : null}

      <Col>
        <div className="d-flex gap-2">
          <FormControl
            value={value.value}
            onChange={(e) => {
              onChange(value.key, e.currentTarget.value);
            }}
          />
          {defaultTranslation === undefined ? (
            <Button variant="danger" onClick={onDelete}>
              X
            </Button>
          ) : null}
        </div>
      </Col>
    </Row>
  );
}
