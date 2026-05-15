import { useEffect, useState } from "react";
import { useBookStore } from "../../store";
import * as api from "../../lib/api";
import type { WordRow } from "../../types";

export default function ArchiveTab() {
  const selectedBookId = useBookStore((s) => s.selectedBookId);
  const [words, setWords] = useState<WordRow[]>([]);
  const [statusMsg, setStatusMsg] = useState("");

  const loadWords = async () => {
    if (selectedBookId === null) { setWords([]); return; }
    const data = await api.getArchivedWords(selectedBookId);
    setWords(data);
  };

  useEffect(() => { loadWords(); }, [selectedBookId]);

  const handleRestore = async (wordId: number) => {
    await api.restoreWord(wordId);
    setStatusMsg("已恢复");
    await loadWords();
  };

  const handleExport = async () => {
    if (selectedBookId === null) return;
    try {
      const csv = await api.exportArchivedWordsCsv(selectedBookId);
      setStatusMsg(`CSV 已生成 (${words.length} 词 — 右键此处查看，后续版本将支持文件保存)`);
    } catch (e) {
      console.error(e);
    }
  };

  if (selectedBookId === null) {
    return <p className="text-text-muted text-center py-12">请先选择词书</p>;
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-bold text-text-primary">熟识单词</h2>
        <div className="flex gap-2">
          <button onClick={handleExport} className="px-4 py-2 bg-accent-primary/20 text-accent-primary rounded-lg text-sm font-medium hover:bg-accent-primary/30 transition-all">
            导出 CSV
          </button>
          <button onClick={loadWords} className="px-4 py-2 glass-card text-text-secondary rounded-lg text-sm hover:text-text-primary transition-all">
            刷新
          </button>
        </div>
      </div>

      {statusMsg && <p className="text-sm text-accent-primary">{statusMsg}</p>}

      <div className="glass-card overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-border text-text-muted text-xs uppercase tracking-wider">
              <th className="text-left px-4 py-3">单词</th>
              <th className="text-left px-4 py-3">释义</th>
              <th className="text-center px-4 py-3">序号</th>
              <th className="text-center px-4 py-3">操作</th>
            </tr>
          </thead>
          <tbody>
            {words.map((w) => (
              <tr key={w.id} className="border-b border-border/50 hover:bg-bg-hover">
                <td className="px-4 py-2.5 text-text-primary font-medium">{w.word}</td>
                <td className="px-4 py-2.5 text-text-secondary">{w.meaning}</td>
                <td className="px-4 py-2.5 text-center text-text-muted">{w.seq}</td>
                <td className="px-4 py-2.5 text-center">
                  <button
                    onClick={() => handleRestore(w.id)}
                    className="px-3 py-1 bg-accent-primary/20 text-accent-primary rounded text-xs hover:bg-accent-primary/30 transition-all"
                  >
                    恢复
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {words.length === 0 && (
          <p className="text-center py-8 text-text-muted">暂无熟识单词，开始背词吧</p>
        )}
      </div>
    </div>
  );
}