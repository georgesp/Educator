import type { RaceObjectName } from "../../../api/client";
import { CarShape } from "./Cards";

/** Colors of the objects, from the palette of the characters (variables of lesson.css). */
export type ObjectColor = "red" | "blue" | "yellow" | "green";

const fill = (color: ObjectColor) => `var(--object-${color})`;

/** A car seen from the side, in a 48 × 24 box, in one of the colors of the palette. */
export function ColoredCar({ color }: { color: ObjectColor }) {
  if (color === "red") {
    return <CarShape />;
  }

  return (
    <>
      <path d="M2 17l4-6h14l7-5h10l9 7v4z" fill={fill(color)} stroke="var(--object-outline)" strokeWidth="1" />
      <path d="M28 7h8l4 4H24z" fill="#BFE3F5" />
      <circle cx="12" cy="18" r="4.5" fill="var(--text)" />
      <circle cx="37" cy="18" r="4.5" fill="var(--text)" />
    </>
  );
}

/** A racing helmet, in a 48 × 48 box. */
function Helmet({ color }: { color: ObjectColor }) {
  return (
    <>
      <path d="M6 30C6 15 16 6 28 6s16 9 16 22v8H14l-8-6z" fill={fill(color)} stroke="var(--object-outline)" strokeWidth="2" />
      <path d="M26 18h18v10H24z" fill="#BFE3F5" stroke="var(--object-outline)" strokeWidth="2" />
      <path d="M8 34h36" stroke="var(--object-outline)" strokeWidth="3" />
    </>
  );
}

/** A tire, in a 48 × 48 box. */
function Tire() {
  return (
    <>
      <circle cx="24" cy="24" r="20" fill="var(--text)" />
      <circle cx="24" cy="24" r="10" fill="var(--surface)" stroke="var(--object-outline)" strokeWidth="2" />
      <circle cx="24" cy="24" r="3" fill="var(--object-outline)" />
    </>
  );
}

/** A checkered flag, in a 48 × 48 box. */
export function CheckeredFlag() {
  return (
    <>
      <path d="M8 4v42" stroke="var(--object-outline)" strokeWidth="3" strokeLinecap="round" />
      <rect x="9" y="6" width="32" height="22" fill="var(--checker-light)" stroke="var(--object-outline)" strokeWidth="2" />
      {[0, 1, 2, 3].map((col) =>
        [0, 1, 2].map((row) =>
          (col + row) % 2 === 0 ? <rect key={`${col}-${row}`} x={9 + col * 8} y={6 + row * 7.33} width="8" height="7.33" fill="var(--checker-dark)" /> : null,
        ),
      )}
    </>
  );
}

/** A traffic cone, in a 48 × 48 box. */
function Cone() {
  return (
    <>
      <path d="M24 4l12 36H12z" fill="var(--object-orange)" stroke="var(--object-outline)" strokeWidth="2" strokeLinejoin="round" />
      <path d="M17.5 24h13l2 6h-17z" fill="var(--checker-light)" />
      <rect x="6" y="40" width="36" height="5" rx="2" fill="var(--object-outline)" />
    </>
  );
}

type RaceObjectProps = {
  object: RaceObjectName;
  color?: ObjectColor;
  /** Width in pixels (the height follows the shape). */
  size?: number;
  /** Drawn in dotted lines: the object that has left. */
  ghost?: boolean;
};

/** A racing object of the lessons (voiture, casque, pneu, drapeau, cône), drawn in SVG. */
export function RaceObject({ object, color = "red", size = 56, ghost = false }: RaceObjectProps) {
  const car = object === "Car";
  return (
    <svg
      className={ghost ? "race-object race-object-ghost" : "race-object"}
      width={size}
      height={car ? size / 2 : size}
      viewBox={car ? "0 0 48 24" : "0 0 48 48"}
      aria-hidden="true"
    >
      {object === "Car" && <ColoredCar color={color} />}
      {object === "Helmet" && <Helmet color={color} />}
      {object === "Tire" && <Tire />}
      {object === "Flag" && <CheckeredFlag />}
      {object === "Cone" && <Cone />}
    </svg>
  );
}
