import { useBookStore, usePlanStore, useStudyStore } from "../../store";
import * as api from "../../lib/api";

export default function StudyModeTab() {
  const selectedBookId = useBookStore((s) => s.selectedBookId);
  const { units, selectedUnitId } = usePlanStore();
  const { isOverlayOpen, setOverlayOpen, setOverlayClosed } = useStudyStore();

  const handleOpen = async (unitId: number) => {
    await api.markUnitDoing(unitId);
    try {
      await api.createOverlayWindow("overlay-main", unitId);
    } catch (e) {
      console.error(e);
    }
    setOverlayOpen(unitId, "overlay-main");
  };

  const handleHide = async () => {
    try {
      await api.hideOverlayWindow("overlay-main");
    } catch (e) {
      console.error(e);
    }
    setOverlayClosed();
  };

  const selectedUnit = units.find((u) => u.id === selectedUnitId);

  return (
    <div className="space-y-6 max-w-2xl">
      <h2 className="text-lg font-bold text-text-primary">摸鱼模式</h2>

      {/* Status */}
      <div className="glass-card p-6">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-sm font-semibold text-text-primary mb-1">
              {isOverlayOpen ? "摸鱼窗口已打开" : "摸鱼窗口未打开"}
            </h3>
            {selectedUnit && (
              <p className="text-sm text-text-secondary">
                当前选中：第 {selectedUnit.unit_no} 单元 ({selectedUnit.pending_words} 词待背)
              </p>
            )}
          </div>
          <div className="flex gap-2">
            {isOverlayOpen ? (
              <button onClick={handleHide} className="px-4 py-2 bg-warning/20 text-warning rounded-lg text-sm font-medium hover:bg-warning/30 transition-all">
                收起窗口
              </button>
            ) : selectedUnitId !== null ? (
              <button onClick={() => handleOpen(selectedUnitId)} className="px-4 py-2 bg-accent-primary text-white rounded-lg text-sm font-medium hover:bg-accent-hover transition-all">
                打开浮窗
              </button>
            ) : (
              <span className="text-sm text-text-muted">请先在"单元"页选择一个单元</span>
            )}
          </div>
        </div>
      </div>

      {/* Tips */}
      <div className="glass-card p-6">
        <h3 className="text-sm font-semibold text-text-primary mb-3">操作说明</h3>
        <div className="space-y-2 text-sm text-text-secondary">
          <p><kbd className="px-1.5 py-0.5 bg-bg-card rounded text-xs border border-border">鼠标左键</kbd> 单击 — 上一词</p>
          <p><kbd className="px-1.5 py-0.5 bg-bg-card rounded text-xs border border-border">鼠标右键</kbd> 单击 — 下一词</p>
          <p><kbd className="px-1.5 py-0.5 bg-bg-card rounded text-xs border border-border">鼠标左键长按 3 秒</kbd> — 文字碎裂动画 → 归档单词</p>
          <p><kbd className="px-1.5 py-0.5 bg-bg-card rounded text-xs border border-border">Ctrl+←</kbd> <kbd className="px-1.5 py-0.5 bg-bg-card rounded text-xs border border-border">Ctrl+→</kbd> — 键盘切词</p>
          <p><kbd className="px-1.5 py-0.5 bg-bg-card rounded text-xs border border-border">空格</kbd> — 暂停/继续自动切换</p>
          <p className="text-text-muted text-xs mt-2">摸鱼窗口可拖拽移动、可拖拽边缘缩放、始终置顶、半透明。</p>
        </div>
      </div>
    </div>
  );
}