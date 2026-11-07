export interface CoreGridLayout {
  columns: number;
  rows: number;
}

export function getCoreGridLayout(
  coreCount: number,
  panelWidth: number,
  panelHeight: number,
): CoreGridLayout {
  if (!Number.isInteger(coreCount) || coreCount < 0) {
    throw new RangeError("Core count must be a non-negative integer.");
  }
  if (coreCount === 0) return { columns: 0, rows: 0 };

  const panelRatio = panelWidth > 0 && panelHeight > 0 ? panelWidth / panelHeight : 1;
  let best = { columns: 1, rows: coreCount };
  let bestDistance = Number.POSITIVE_INFINITY;

  for (let columns = 1; columns <= coreCount; columns += 1) {
    const rows = Math.ceil(coreCount / columns);
    const gridRatio = columns / rows;
    const distance = Math.abs(Math.log(gridRatio / panelRatio));
    if (distance < bestDistance) {
      best = { columns, rows };
      bestDistance = distance;
    }
  }

  return best;
}
