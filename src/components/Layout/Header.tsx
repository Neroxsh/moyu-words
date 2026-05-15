import { useBookStore } from "../../store";
import type { BookRow } from "../../types";

export default function Header() {
  const books = useBookStore((s) => s.books);
  const selectedBookId = useBookStore((s) => s.selectedBookId);
  const selectedBook = books.find((b) => b.id === selectedBookId);

  return (
    <header className="h-16 bg-bg-secondary border-b border-border flex items-center px-6 flex-shrink-0">
      <div className="flex items-center gap-3">
        <span className="text-xl font-bold text-accent-primary">摸鱼背词</span>
        <span className="text-xs text-text-muted bg-bg-card px-2 py-0.5 rounded">v0.1.0</span>
      </div>
      <div className="ml-auto flex items-center gap-4">
        <div className="glass-card px-4 py-1.5 text-sm">
          <span className="text-text-secondary mr-2">词书:</span>
          <span className="text-text-primary font-medium">{selectedBook?.title ?? "未选择"}</span>
        </div>
        <div className="glass-card px-4 py-1.5 text-sm">
          <span className="text-text-secondary mr-2">单词:</span>
          <span className="text-text-primary font-medium">{selectedBook?.actual_words ?? 0}</span>
        </div>
      </div>
    </header>
  );
}