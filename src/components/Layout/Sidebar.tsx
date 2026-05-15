import { useState } from "react";
import { useBookStore, usePlanStore } from "../../store";
import * as api from "../../lib/api";

type Tab = "units" | "archive" | "study" | "settings";

interface Props {
  activeTab: Tab;
  onTabChange: (tab: Tab) => void;
}

export default function Sidebar({ activeTab, onTabChange }: Props) {
  const { books, selectedBookId, builtinBooks, fetchBooks, selectBook, importBuiltin, importLocal, deleteSelected } = useBookStore();
  const { planDays, setDays } = usePlanStore();
  const [daysInput, setDaysInput] = useState("7");

  const handleCreatePlan = async () => {
    if (selectedBookId === null) return;
    const days = Math.max(1, parseInt(daysInput) || 7);
    try {
      const planId = await api.createPlan(selectedBookId, days);
      const units = await api.listUnits(planId);
      usePlanStore.getState().fetchUnits(planId);
    } catch (e: any) {
      console.error(e);
    }
  };

  const handleImportLocal = async () => {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const result = await open({
        filters: [{ name: "词书", extensions: ["txt", "json", "csv"] }],
        multiple: false,
      });
      if (result) {
        const filePath = result as string;
        const fileName = filePath.split(/[/\\]/).pop()?.replace(/\.[^.]+$/, "") ?? "未命名";
        await importLocal(filePath, fileName);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const tabs: { id: Tab; label: string; icon: string }[] = [
    { id: "units", label: "单元", icon: "📖" },
    { id: "archive", label: "熟识单词", icon: "✅" },
    { id: "study", label: "摸鱼模式", icon: "🐟" },
    { id: "settings", label: "设置", icon: "⚙️" },
  ];

  return (
    <aside className="w-72 bg-bg-secondary border-r border-border flex flex-col flex-shrink-0 overflow-hidden">
      {/* Navigation */}
      <nav className="p-4 space-y-1">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => onTabChange(tab.id)}
            className={`w-full text-left px-4 py-2.5 rounded-lg text-sm font-medium transition-all duration-150 ${
              activeTab === tab.id
                ? "bg-accent-primary/15 text-accent-primary border border-accent-primary/20"
                : "text-text-secondary hover:bg-bg-hover hover:text-text-primary"
            }`}
          >
            <span className="mr-2">{tab.icon}</span>
            {tab.label}
          </button>
        ))}
      </nav>

      <div className="border-t border-border" />

      {/* Book Library */}
      <div className="flex-1 overflow-y-auto p-4">
        <h3 className="text-xs font-semibold text-text-muted uppercase tracking-wider mb-3">词书</h3>

        {books.map((book) => (
          <button
            key={book.id}
            onClick={() => selectBook(book.id)}
            className={`w-full text-left px-3 py-2 rounded-lg text-sm mb-1 transition-all ${
              book.id === selectedBookId
                ? "bg-accent-primary/10 text-accent-primary border border-accent-primary/20"
                : "text-text-secondary hover:bg-bg-hover"
            }`}
          >
            <div className="font-medium truncate">{book.title}</div>
            <div className="text-xs text-text-muted">{book.actual_words} 词</div>
          </button>
        ))}

        {/* Import Buttons */}
        <div className="mt-3 space-y-1">
          <button
            onClick={handleImportLocal}
            className="w-full text-left px-3 py-2 rounded-lg text-sm text-accent-primary hover:bg-accent-primary/10 transition-all"
          >
            + 导入本地词书
          </button>
          <button
            onClick={deleteSelected}
            className="w-full text-left px-3 py-2 rounded-lg text-sm text-danger hover:bg-danger/10 transition-all"
          >
            删除选中词书
          </button>
        </div>

        {/* Builtin Books */}
        <h3 className="text-xs font-semibold text-text-muted uppercase tracking-wider mt-4 mb-2">内置词书</h3>
        <div className="space-y-0.5">
          {builtinBooks.map((book) => (
            <button
              key={book.index}
              onClick={() => importBuiltin(book.index)}
              className="w-full text-left px-3 py-1.5 rounded-lg text-sm text-text-secondary hover:bg-bg-hover hover:text-text-primary transition-all"
            >
              {book.title}
            </button>
          ))}
        </div>
      </div>

      {/* Plan Creator */}
      <div className="border-t border-border p-4">
        <h3 className="text-xs font-semibold text-text-muted uppercase tracking-wider mb-2">学习计划</h3>
        <div className="flex items-center gap-2">
          <input
            type="number"
            min={1}
            max={365}
            value={daysInput}
            onChange={(e) => setDaysInput(e.target.value)}
            className="w-16 bg-bg-card border border-border rounded-lg px-2 py-1.5 text-sm text-text-primary text-center focus:outline-none focus:border-accent-primary"
          />
          <span className="text-xs text-text-muted">天</span>
          <button
            onClick={handleCreatePlan}
            className="ml-auto px-3 py-1.5 bg-accent-primary text-white rounded-lg text-xs font-medium hover:bg-accent-hover transition-all"
          >
            生成单元
          </button>
        </div>
      </div>
    </aside>
  );
}