import { invoke } from "@tauri-apps/api/core";
import type {
  BookRow, BookSummary, WordRow, PlanRow, UnitRow,
  UnitWordRow, CurrentWord, BuiltinBookDef,
} from "../types";

// ── Books ──
export const listBooks = () => invoke<BookRow[]>("list_books");
export const getBook = (bookId: number) => invoke<BookRow | null>("get_book", { bookId });
export const getActiveBook = () => invoke<BookRow | null>("get_active_book");
export const setActiveBook = (bookId: number) => invoke<void>("set_active_book", { bookId });
export const addBookFromFile = (title: string, filePath: string) =>
  invoke<number>("add_book_from_file", { title, filePath });
export const addBuiltinBook = (bookIndex: number) =>
  invoke<number>("add_builtin_book", { bookIndex });
export const deleteBook = (bookId: number) => invoke<void>("delete_book", { bookId });
export const getBookSummary = (bookId: number) =>
  invoke<BookSummary>("get_book_summary", { bookId });

// ── Words ──
export const listWords = (bookId: number, includeArchived = true) =>
  invoke<WordRow[]>("list_words", { bookId, includeArchived });
export const getArchivedWords = (bookId: number) =>
  invoke<WordRow[]>("get_archived_words", { bookId });
export const archiveWord = (wordId: number) => invoke<void>("archive_word", { wordId });
export const restoreWord = (wordId: number) => invoke<void>("restore_word", { wordId });
export const exportArchivedWordsCsv = (bookId: number) =>
  invoke<string>("export_archived_words_csv", { bookId });

// ── Plans ──
export const getActivePlan = (bookId: number) =>
  invoke<PlanRow | null>("get_active_plan", { bookId });
export const createPlan = (bookId: number, days: number) =>
  invoke<number>("create_plan", { bookId, days });

// ── Units ──
export const listUnits = (planId: number) => invoke<UnitRow[]>("list_units", { planId });
export const getUnitInfo = (unitId: number) => invoke<UnitRow | null>("get_unit_info", { unitId });
export const markUnitDoing = (unitId: number) => invoke<void>("mark_unit_doing", { unitId });
export const markUnitDone = (unitId: number) => invoke<void>("mark_unit_done", { unitId });

// ── Unit Words ──
export const listUnitWords = (unitId: number, pendingOnly = false) =>
  invoke<UnitWordRow[]>("list_unit_words", { unitId, pendingOnly });
export const markUnitWord = (unitWordId: number, reviewState: string) =>
  invoke<void>("mark_unit_word", { unitWordId, reviewState });

// ── Settings ──
export const getSetting = (key: string, defaultValue = "") =>
  invoke<string>("get_setting", { key, default: defaultValue });
export const setSetting = (key: string, value: string) =>
  invoke<void>("set_setting", { key, value });

// ── Builtin ──
export const getBuiltinBookList = () => invoke<BuiltinBookDef[]>("get_builtin_book_list");

// ── Overlay Window ──
export const createOverlayWindow = (label: string, unitId: number) =>
  invoke<void>("create_overlay_window", { label, unitId });
export const closeOverlayWindow = (label: string) =>
  invoke<void>("close_overlay_window", { label });
export const hideOverlayWindow = (label: string) =>
  invoke<void>("hide_overlay_window", { label });

// ── Overlay Study ──
export const getCurrentWord = (unitId: number, queueIndex: number) =>
  invoke<CurrentWord | null>("get_current_word", { unitId, queueIndex });
export const goPrevWord = (queueLen: number, currentIdx: number) =>
  invoke<number>("go_prev_word", { queueLen, currentIdx });
export const goNextWord = (queueLen: number, currentIdx: number) =>
  invoke<number>("go_next_word", { queueLen, currentIdx });
export const toggleAuto = (current: boolean) =>
  invoke<boolean>("toggle_auto", { current });