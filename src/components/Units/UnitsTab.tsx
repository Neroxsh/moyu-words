import { useEffect, useState } from "react";
import { useBookStore, usePlanStore, useStudyStore } from "../../store";
import * as api from "../../lib/api";
import type { UnitRow } from "../../types";

export default function UnitsTab() {
  const selectedBookId = useBookStore((s) => s.selectedBookId);
  const { units, selectedUnitId, fetchUnits, selectUnit } = usePlanStore();
  const { setOverlayOpen } = useStudyStore();
  const [statusMsg, setStatusMsg] = useState("");
  const [planId, setPlanId] = useState<number | null>(null);

  useEffect(() => {
    if (selectedBookId === null) return;
    (async () => {
      const plan = await api.getActivePlan(selectedBookId);
      if (plan) {
        setPlanId(plan.id);
        await fetchUnits(plan.id);
      } else {
        setPlanId(null);
        usePlanStore.setState({ units: [] });
      }
    })();
  }, [selectedBookId]);

  const handleOpenOverlay = async () => {
    const unitId = selectedUnitId ?? units[0]?.id;
    if (!unitId) return;
    await api.markUnitDoing(unitId);
    try {
      await api.createOverlayWindow("overlay-main", unitId);
    } catch (e) {
      console.error(e);
    }
    setOverlayOpen(unitId, "overlay-main");
    setStatusMsg("摸鱼窗口已打开");
  };

  const handleMarkDone = async () => {
    if (!selectedUnitId) return;
    await api.markUnitDone(selectedUnitId);
    setStatusMsg("单元已打卡");
    if (planId) await fetchUnits(planId);
  };

  if (!planId) {
    return (
      <div className="flex items-center justify-center h-64 text-text-muted">
        <div className="text-center">
          <p className="text-lg mb-2">还没有学习计划</p>
          <p className="text-sm">先选择词书，在侧边栏设置天数后生成单元</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-bold text-text-primary">单元列表</h2>
        <div className="flex gap-2">
          <button onClick={handleMarkDone} className="px-4 py-2 bg-success/20 text-success rounded-lg text-sm font-medium hover:bg-success/30 transition-all">
            标记完成
          </button>
          <button onClick={handleOpenOverlay} className="px-4 py-2 bg-accent-primary text-white rounded-lg text-sm font-medium hover:bg-accent-hover transition-all">
            打开摸鱼窗
          </button>
        </div>
      </div>

      {statusMsg && <p className="text-sm text-accent-primary">{statusMsg}</p>}

      <div className="glass-card overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-border text-text-muted text-xs uppercase tracking-wider">
              <th className="text-left px-4 py-3">单元</th>
              <th className="text-left px-4 py-3">范围</th>
              <th className="text-center px-4 py-3">总词</th>
              <th className="text-center px-4 py-3">待背</th>
              <th className="text-center px-4 py-3">状态</th>
            </tr>
          </thead>
          <tbody>
            {units.map((unit) => (
              <tr
                key={unit.id}
                onClick={() => selectUnit(unit.id)}
                className={`border-b border-border/50 cursor-pointer transition-colors ${
                  unit.id === selectedUnitId ? "bg-accent-primary/10" : "hover:bg-bg-hover"
                }`}
              >
                <td className="px-4 py-2.5 text-text-primary font-medium">第 {unit.unit_no} 单元</td>
                <td className="px-4 py-2.5 text-text-secondary">{unit.start_seq} - {unit.end_seq}</td>
                <td className="px-4 py-2.5 text-center text-text-primary">{unit.total_words}</td>
                <td className="px-4 py-2.5 text-center text-warning">{unit.pending_words}</td>
                <td className="px-4 py-2.5 text-center">
                  <span className={`inline-block px-2 py-0.5 rounded text-xs font-medium ${
                    unit.status === "done" ? "bg-success/20 text-success" :
                    unit.status === "doing" ? "bg-accent-primary/20 text-accent-primary" :
                    "bg-text-muted/20 text-text-muted"
                  }`}>
                    {unit.status === "done" ? "完成" : unit.status === "doing" ? "进行中" : "待开始"}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {units.length === 0 && (
          <p className="text-center py-8 text-text-muted">暂无单元数据</p>
        )}
      </div>
    </div>
  );
}