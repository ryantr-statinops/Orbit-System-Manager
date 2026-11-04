import { useEffect, useMemo, useRef } from "react";
import * as echarts from "echarts/core";
import { LineChart } from "echarts/charts";
import { GridComponent, TooltipComponent } from "echarts/components";
import { CanvasRenderer } from "echarts/renderers";
import { toCpuHistoryData } from "./chartData";
import type { FixtureSample } from "./fixtures";
import { getRenderPixelRatio } from "./renderPolicy";

echarts.use([LineChart, GridComponent, TooltipComponent, CanvasRenderer]);

interface CpuHistoryChartProps {
  samples: FixtureSample[];
}

function CpuHistoryChart({ samples }: CpuHistoryChartProps) {
  const chartElement = useRef<HTMLDivElement>(null);
  const chartData = useMemo(() => toCpuHistoryData(samples), [samples]);
  const cpuAvailable = samples.some((sample) => sample.cpuTotalPercent !== null);

  useEffect(() => {
    const element = chartElement.current;
    if (!element) return;

    const chart = echarts.init(element, null, {
      renderer: "canvas",
      devicePixelRatio: getRenderPixelRatio(window.devicePixelRatio),
    });
    chart.setOption({
      animation: false,
      grid: { top: 14, right: 18, bottom: 28, left: 42 },
      tooltip: {
        trigger: "axis",
        valueFormatter: (value: unknown) =>
          value == null ? "Unavailable" : `${Number(value).toFixed(1)}%`,
      },
      xAxis: {
        type: "value",
        min: 0,
        max: 60,
        interval: 15,
        axisLabel: { color: "#71839a", formatter: "{value}s" },
        axisLine: { lineStyle: { color: "#2a394b" } },
        splitLine: { show: false },
      },
      yAxis: {
        type: "value",
        min: 0,
        max: 100,
        interval: 25,
        axisLabel: { color: "#71839a", formatter: "{value}%" },
        axisLine: { show: false },
        splitLine: { lineStyle: { color: "#202b3a" } },
      },
      series: [
        {
          name: "CPU total",
          type: "line",
          data: chartData,
          showSymbol: chartData.length === 1,
          symbolSize: 7,
          connectNulls: false,
          lineStyle: { color: "#69bafa", width: 2 },
          itemStyle: { color: "#69bafa" },
          areaStyle: { color: "rgba(68, 153, 220, 0.12)" },
        },
      ],
    });

    const observer = new ResizeObserver(() => chart.resize());
    observer.observe(element);

    return () => {
      observer.disconnect();
      chart.dispose();
    };
  }, [chartData]);

  return (
    <div className="chart-plot">
      <div
        ref={chartElement}
        className="chart-canvas"
        role="img"
        aria-label="CPU usage history chart"
      />
      {samples.length === 0 && <p className="chart-overlay">No samples in this fixture.</p>}
      {samples.length > 0 && !cpuAvailable && (
        <p className="chart-overlay">CPU data unavailable for this fixture.</p>
      )}
    </div>
  );
}

export default CpuHistoryChart;
