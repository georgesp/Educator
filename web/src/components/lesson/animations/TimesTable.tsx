import { motion } from "motion/react";
import type { LessonOperator, TableCell, TableHighlight } from "../../../api/client";
import { fr } from "../../../i18n/fr";
import { useMove, useSteps } from "../useSteps";

type TimesTableProps = {
  operator: LessonOperator;
  size: number;
  cells: TableCell[][];
  highlights: TableHighlight[];
};

/**
 * `timesTable`: the table of addition or multiplication from 1 to `size`; its cells light up group
 * after group (the table of 2, the doubles, the friends of 10), each group in its own color.
 */
export function TimesTable({ operator, size, cells, highlights }: TimesTableProps) {
  const lit = useSteps(highlights.length, cells);
  const move = useMove();
  const header = Array.from({ length: size }, (_, i) => i + 1);

  return (
    <div className="lesson-table">
      <table className="times-table" aria-label={fr.lessons.table(operator, size)}>
        <thead>
          <tr>
            <th className="times-corner">{fr.lessons.sign[operator]}</th>
            {header.map((n) => (
              <th key={n} scope="col">
                {n}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {cells.map((row, r) => (
            <tr key={r}>
              <th scope="row">{r + 1}</th>
              {row.map((cell) => {
                const on = cell.group !== null && cell.group < lit;
                return (
                  <td key={cell.col} className={on ? `times-cell times-group-${cell.group}` : "times-cell"}>
                    {on ? (
                      <motion.span initial={{ scale: 0.6 }} animate={{ scale: 1 }} transition={move}>
                        {cell.value}
                      </motion.span>
                    ) : (
                      cell.value
                    )}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
      <ul className="times-legend">
        {highlights.slice(0, lit).map((h, i) => (
          <li key={i} className={`times-legend-item times-group-${i}`}>
            {fr.lessons.tableHighlight(h.kind, h.value)}
          </li>
        ))}
      </ul>
    </div>
  );
}
