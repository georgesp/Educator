import type { ComponentProps, ReactNode } from "react";

type BigButtonProps = Omit<ComponentProps<"button">, "children"> & {
  label: string;
  hint?: string;
  icon?: ReactNode;
  /** Small number shown before the icon, like a starting grid position (01, 02…). */
  number?: string;
  variant?: "primary" | "default" | "outline";
  /** Visible but inactive: still focusable and clickable (to explain why), never triggers its action. */
  locked?: boolean;
};

/** Large slanted button of the sport theme: icon + label (+ hint), at least 64 px high. */
export function BigButton({ label, hint, icon, number, variant = "default", locked = false, className, ...props }: BigButtonProps) {
  const classes = ["big-button", `big-button-${variant}`, locked ? "big-button-locked" : "", className ?? ""].filter(Boolean).join(" ");
  return (
    <button type="button" className={classes} aria-disabled={locked || undefined} {...props}>
      {number && (
        <span className="big-button-number" aria-hidden="true">
          {number}
        </span>
      )}
      {icon && <span className="big-button-icon">{icon}</span>}
      <span className="big-button-text">
        <span className="big-button-label">{label}</span>
        {hint && <span className="big-button-hint">{hint}</span>}
      </span>
    </button>
  );
}
