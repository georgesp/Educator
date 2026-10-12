import type { Animation } from "../../api/client";
import { CalcChain } from "./animations/CalcChain";
import { RecapCard, TitleCard } from "./animations/Cards";
import { NumberLine } from "./animations/NumberLine";
import { TensUnits } from "./animations/TensUnits";

type LessonAnimationProps = {
  animation: Animation;
  /** Called when the child has done what an interactive animation asks. */
  onComplete?(): void;
};

/** The animation of a slide, from the library of the lessons (SUB000117), in the Grand Prix theme. */
export function LessonAnimation({ animation, onComplete }: LessonAnimationProps) {
  switch (animation.type) {
    case "titleCard":
      return <TitleCard title={animation.title} subtitle={animation.subtitle} />;
    case "recapCard":
      return <RecapCard text={animation.text} example={animation.example} />;
    case "calcChain":
      return <CalcChain chains={animation.chains} mode={animation.mode} onComplete={onComplete} />;
    case "numberLine":
      return (
        <NumberLine
          from={animation.from}
          to={animation.to}
          step={animation.step}
          start={animation.start}
          jumps={animation.jumps}
          interactive={animation.interactive}
          hoverValues={animation.hoverValues}
          onComplete={onComplete}
        />
      );
    case "tensUnits":
      return <TensUnits hundreds={animation.hundreds} tens={animation.tens} units={animation.units} missing={animation.missing} />;
  }
}
