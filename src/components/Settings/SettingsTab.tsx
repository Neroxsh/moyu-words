import { useSettingsStore } from "../../store";

export default function SettingsTab() {
  const { autoInterval, studyAlpha, saveInterval, saveAlpha } = useSettingsStore();

  return (
    <div className="space-y-6 max-w-2xl">
      <h2 className="text-lg font-bold text-text-primary">设置</h2>

      <div className="glass-card p-6 space-y-6">
        {/* Transparency */}
        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            摸鱼窗口透明度: {studyAlpha.toFixed(2)}
          </label>
          <input
            type="range"
            min={0.7}
            max={1.0}
            step={0.01}
            value={studyAlpha}
            onChange={(e) => saveAlpha(Number(e.target.value))}
            className="w-full h-2 bg-bg-hover rounded-lg appearance-none cursor-pointer accent-accent-primary"
          />
          <div className="flex justify-between text-xs text-text-muted mt-1">
            <span>更透明</span>
            <span>不透明</span>
          </div>
        </div>

        {/* Auto Interval */}
        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            自动切换间隔
          </label>
          <div className="flex items-center gap-3">
            <input
              type="number"
              min={1}
              max={60}
              value={autoInterval}
              onChange={(e) => saveInterval(Math.max(1, Number(e.target.value)))}
              className="w-20 bg-bg-card border border-border rounded-lg px-3 py-2 text-sm text-text-primary text-center focus:outline-none focus:border-accent-primary"
            />
            <span className="text-sm text-text-muted">秒后切换到下一个单词</span>
          </div>
        </div>
      </div>

      {/* About */}
      <div className="glass-card p-6">
        <h3 className="text-sm font-semibold text-text-primary mb-3">关于</h3>
        <div className="text-sm text-text-secondary space-y-1">
          <p>摸鱼背词 v0.1.0</p>
          <p>一款可本地运行、可打包成 macOS App 和 Windows exe 的桌面背单词应用。</p>
          <p className="mt-2 text-text-muted text-xs">
            功能：选词书、定计划、摸鱼透明窗口、鼠标左右键切词、长按归档、文字碎裂动画、自动切换、键盘快捷键。
          </p>
        </div>
      </div>
    </div>
  );
}