import { useEffect, useRef, useState } from "react";
import type { CSSProperties } from "react";
import { getCoreGridLayout } from "./coreGrid";

interface CoreGridProps {
  values: Array<number | null>;
}

function CoreGrid({ values }: CoreGridProps) {
  const host = useRef<HTMLDivElement>(null);
  const [panelSize, setPanelSize] = useState({ width: 0, height: 0 });
  const [selectedCore, setSelectedCore] = useState(0);
  const layout = getCoreGridLayout(values.length, panelSize.width, panelSize.height);

  useEffect(() => {
    const element = host.current;
    if (!element) return;

    const observer = new ResizeObserver(([entry]) => {
      setPanelSize({ width: entry.contentRect.width, height: entry.contentRect.height });
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    if (selectedCore >= values.length) setSelectedCore(Math.max(0, values.length - 1));
  }, [selectedCore, values.length]);

  if (values.length === 0) {
    return <p className="core-grid-empty">No logical processors in this fixture.</p>;
  }

  const selectedValue = values[selectedCore];
  return (
    <div className="core-grid-panel">
      <div
        ref={host}
        className="core-grid"
        style={{ "--core-columns": layout.columns } as CSSProperties}
        aria-label={`${values.length} logical processor cores`}
      >
        {values.map((value, index) => (
          <button
            className={`core-tile${selectedCore === index ? " selected" : ""}`}
            key={index}
            type="button"
            aria-label={`Core ${index + 1}: ${value == null ? "unavailable" : `${value.toFixed(1)}%`}`}
            aria-pressed={selectedCore === index}
            onClick={() => setSelectedCore(index)}
          >
            <span>{index + 1}</span>
            <strong>{value == null ? "—" : `${Math.round(value)}%`}</strong>
          </button>
        ))}
      </div>
      <p className="core-grid-detail" aria-live="polite">
        Core {selectedCore + 1} · {selectedValue == null ? "Unavailable" : `${selectedValue.toFixed(1)}%`}
      </p>
    </div>
  );
}

export default CoreGrid;
