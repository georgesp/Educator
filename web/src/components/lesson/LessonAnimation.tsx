import type { Animation } from "../../api/client";
import { ArrayGrid } from "./animations/ArrayGrid";
import { BarModel } from "./animations/BarModel";
import { CalcChain } from "./animations/CalcChain";
import { RecapCard, TitleCard } from "./animations/Cards";
import { ColumnOperation } from "./animations/ColumnOperation";
import { CompareNumbers } from "./animations/CompareNumbers";
import { Equation } from "./animations/Equation";
import { FractionBar } from "./animations/FractionBar";
import { GroupObjects } from "./animations/GroupObjects";
import { LetterTiles } from "./animations/LetterTiles";
import { NumberLine } from "./animations/NumberLine";
import { PlaceValueTable } from "./animations/PlaceValueTable";
import { SequencePattern } from "./animations/SequencePattern";
import { SoundSort } from "./animations/SoundSort";
import { TenFrame } from "./animations/TenFrame";
import { TensUnits } from "./animations/TensUnits";
import { TimesTable } from "./animations/TimesTable";

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
          flags={animation.flags}
          onComplete={onComplete}
        />
      );
    case "tensUnits":
      return (
        <TensUnits
          value={animation.value}
          hundreds={animation.hundreds}
          tens={animation.tens}
          units={animation.units}
          missing={animation.missing}
          steps={animation.steps}
          change={animation.change}
          result={animation.result}
          parts={animation.parts}
          interactive={animation.interactive}
          task={animation.task}
          interactiveStart={animation.interactiveStart}
          target={animation.target}
          onComplete={onComplete}
        />
      );
    case "columnOperation":
      return (
        <ColumnOperation
          operator={animation.operator}
          operands={animation.operands}
          result={animation.result}
          layouts={animation.layouts}
          computedColumns={animation.computedColumns}
          interactive={animation.interactive}
          onComplete={onComplete}
        />
      );
    case "equation":
      return (
        <Equation
          left={animation.left}
          operator={animation.operator}
          right={animation.right}
          result={animation.result}
          missing={animation.missing}
          missingValue={animation.missingValue}
          equalities={animation.equalities}
          interactive={animation.interactive}
          choices={animation.choices}
          onComplete={onComplete}
        />
      );
    case "compareNumbers":
      return <CompareNumbers {...animation} onComplete={onComplete} />;
    case "sequencePattern":
      return <SequencePattern {...animation} onComplete={onComplete} />;
    case "arrayGrid":
      return <ArrayGrid {...animation} onComplete={onComplete} />;
    case "timesTable":
      return <TimesTable {...animation} />;
    case "placeValueTable":
      return <PlaceValueTable {...animation} onComplete={onComplete} />;
    case "groupObjects":
      return <GroupObjects {...animation} onComplete={onComplete} />;
    case "tenFrame":
      return <TenFrame {...animation} onComplete={onComplete} />;
    case "fractionBar":
      return <FractionBar {...animation} onComplete={onComplete} />;
    case "barModel":
      return <BarModel {...animation} onComplete={onComplete} />;
    case "letterTiles":
      return <LetterTiles {...animation} onComplete={onComplete} />;
    case "soundSort":
      return <SoundSort {...animation} onComplete={onComplete} />;
  }
}
