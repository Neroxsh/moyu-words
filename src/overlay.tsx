import React, { useEffect, useState, useCallback, useRef } from "react";
import ReactDOM from "react-dom/client";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { OverlayInitPayload } from "./types";
import WordDisplay from "./components/Overlay/WordDisplay";
import ShatterCanvas from "./components/Overlay/ShatterCanvas";

function OverlayApp() {
  const [unitId, setUnitId] = useState<number | null>(null);
  const [queue, setQueue] = useState<{ id: number; wordId: number; word: string; meaning: string }[]>([]);
  const [queueIdx, setQueueIdx] = useState(0);
  const [totalWords, setTotalWords] = useState(0);
  const [autoRunning, setAutoRunning] = useState(true);
  const [interval, setInterval] = useState(3);
  const [shattering, setShattering] = useState(false);
  const [shatterWord, setShatterWord] = useState("");
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const longPressRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const longPressActiveRef = useRef(false);
  const longPressTriggeredRef = useRef(false);
  const mouseDownPos = useRef<{ x: number; y: number } | null>(null);
  const isDraggingRef = useRef(false);
  const DRAG_THRESHOLD = 5;

  const currentWord = queue[queueIdx];

  // Listen for init from main window
  useEffect(() => {
    const unlisten = listen<OverlayInitPayload>("init-overlay", (event) => {
      const { unit_id } = event.payload;
      setUnitId(unit_id);
      loadWords(unit_id, 0);
    });
    return () => { unlisten.then((fn) => fn()); };
  }, []);

  const loadWords = useCallback(async (uid: number, startIdx: number) => {
    try {
      invoke("get_setting", { key: "auto_interval", default: "3" }).then((v: unknown) => {
        setInterval(Number(v) || 3);
      });
      const [pending, all] = await Promise.all([
        invoke<Array<{ id: number; word_id: number; word: string; meaning: string }>>("list_unit_words", { unitId: uid, pendingOnly: true }),
        invoke<Array<{ id: number }>>("list_unit_words", { unitId: uid, pendingOnly: false }),
      ]);
      setTotalWords(all.length);
      setQueue(pending.map((w) => ({ id: w.id, wordId: w.word_id, word: w.word, meaning: w.meaning })));
      setQueueIdx(startIdx < pending.length ? startIdx : 0);
      setAutoRunning(true);
    } catch (e) {
      console.error("Failed to load words:", e);
    }
  }, []);

  const startTimer = useCallback(() => {
    if (timerRef.current) clearTimeout(timerRef.current);
    if (!autoRunning || shattering) return;
    timerRef.current = setTimeout(() => {
      handleNext();
    }, interval * 1000);
  }, [autoRunning, interval, shattering]);

  const handlePrev = useCallback(() => {
    setQueueIdx((prev) => (prev + queue.length - 1) % queue.length);
  }, [queue.length]);

  const handleNext = useCallback(() => {
    setQueueIdx((prev) => (prev + 1) % queue.length);
  }, [queue.length]);

  const handleArchive = useCallback(async () => {
    if (!currentWord) return;
    setShattering(true);
    setShatterWord(currentWord.word);
    // Animation runs in ShatterCanvas, ~3 seconds
    setTimeout(async () => {
      await invoke("mark_unit_word", { unitWordId: currentWord.id, reviewState: "known" });
      await invoke("archive_word", { wordId: currentWord.wordId });
      // Refresh queue
      if (unitId !== null) {
        const pending = await invoke<Array<{ id: number; word_id: number; word: string; meaning: string }>>("list_unit_words", { unitId, pendingOnly: true });
        if (pending.length === 0) {
          // Unit completed
          await invoke("mark_unit_done", { unitId });
          setQueue([]);
          setQueueIdx(0);
        } else {
          setQueue(pending.map((w) => ({ id: w.id, wordId: w.word_id, word: w.word, meaning: w.meaning })));
          setQueueIdx(0);
        }
      }
      setShattering(false);
      setAutoRunning(true);
    }, 3100);
  }, [currentWord, unitId]);

  // Timer effect
  useEffect(() => {
    startTimer();
    return () => { if (timerRef.current) clearTimeout(timerRef.current); };
  }, [queueIdx, autoRunning, shattering, startTimer]);

  // Mouse event handlers (manual drag detection, no data-tauri-drag-region)
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    if (e.button !== 0) return;
    mouseDownPos.current = { x: e.clientX, y: e.clientY };
    isDraggingRef.current = false;
    longPressTriggeredRef.current = false;
    longPressActiveRef.current = true;
    longPressRef.current = setTimeout(() => {
      if (longPressActiveRef.current && !isDraggingRef.current) {
        longPressTriggeredRef.current = true;
        handleArchive();
      }
    }, 3000);
  }, [handleArchive]);

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!mouseDownPos.current || isDraggingRef.current) return;
      const dx = e.clientX - mouseDownPos.current.x;
      const dy = e.clientY - mouseDownPos.current.y;
      if (Math.abs(dx) > DRAG_THRESHOLD || Math.abs(dy) > DRAG_THRESHOLD) {
        isDraggingRef.current = true;
        longPressActiveRef.current = false;
        if (longPressRef.current) clearTimeout(longPressRef.current);
        // Start native window drag
        getCurrentWindow().startDragging();
      }
    };
    const handleMouseUpGlobal = (e: MouseEvent) => {
      const wasLongPress = longPressTriggeredRef.current;
      longPressActiveRef.current = false;
      longPressTriggeredRef.current = false;
      if (longPressRef.current) clearTimeout(longPressRef.current);
      if (isDraggingRef.current) {
        mouseDownPos.current = null;
        return;
      }
      mouseDownPos.current = null;
      if (e.button === 0 && !wasLongPress) {
        handlePrev();
      }
    };
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUpGlobal);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUpGlobal);
    };
  }, [handlePrev]);

  const handleContextMenu = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    handleNext();
  }, [handleNext]);

  // Keyboard handlers
  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === "ArrowLeft") { e.preventDefault(); handlePrev(); }
      if (e.ctrlKey && e.key === "ArrowRight") { e.preventDefault(); handleNext(); }
      if (e.key === " ") { e.preventDefault(); setAutoRunning((p) => !p); }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [handlePrev, handleNext]);

  if (!currentWord) {
    return (
      <div onMouseDown={handleMouseDown} onContextMenu={handleContextMenu}
        style={{ height: "100%", display: "flex", alignItems: "center", justifyContent: "center",
          color: "rgba(255,255,255,0.3)", fontSize: 14, fontFamily: "sans-serif", cursor: "default" }}>
        {queue.length === 0 ? "本单元已完成" : "加载中..."}
      </div>
    );
  }

  const progressText = `${totalWords - queue.length} / ${totalWords}`;

  return (
    <div
      onMouseDown={handleMouseDown}
      onContextMenu={handleContextMenu}
      style={{
        height: "100%", display: "flex", alignItems: "center", justifyContent: "center",
        cursor: "default", background: "transparent", position: "relative",
        padding: "8px 16px",
      }}
    >
      {shattering ? (
        <ShatterCanvas word={shatterWord} />
      ) : (
        <WordDisplay word={currentWord.word} meaning={currentWord.meaning} progress={progressText} />
      )}
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <OverlayApp />
  </React.StrictMode>
);