import { create } from "zustand";
import type { BookRow, UnitRow, BuiltinBookDef } from "../types";
import * as api from "../lib/api";

interface BookStore {
  books: BookRow[];
  builtinBooks: BuiltinBookDef[];
  selectedBookId: number | null;
  loading: boolean;
  fetchBooks: () => Promise<void>;
  selectBook: (id: number) => Promise<void>;
  importBuiltin: (index: number) => Promise<void>;
  importLocal: (path: string, title: string) => Promise<void>;
  deleteSelected: () => Promise<void>;
}

export const useBookStore = create<BookStore>((set, get) => ({
  books: [],
  builtinBooks: [],
  selectedBookId: null,
  loading: false,

  fetchBooks: async () => {
    set({ loading: true });
    try {
      const [books, builtin] = await Promise.all([
        api.listBooks(),
        api.getBuiltinBookList(),
      ]);
      const activeBook = await api.getActiveBook();
      set({
        books,
        builtinBooks: builtin,
        selectedBookId: activeBook?.id ?? (books[0]?.id ?? null),
        loading: false,
      });
    } catch {
      set({ loading: false });
    }
  },

  selectBook: async (id: number) => {
    await api.setActiveBook(id);
    set({ selectedBookId: id });
  },

  importBuiltin: async (index: number) => {
    await api.addBuiltinBook(index);
    await get().fetchBooks();
  },

  importLocal: async (path: string, title: string) => {
    await api.addBookFromFile(title, path);
    await get().fetchBooks();
  },

  deleteSelected: async () => {
    const id = get().selectedBookId;
    if (id === null) return;
    await api.deleteBook(id);
    await get().fetchBooks();
  },
}));

interface PlanStore {
  units: UnitRow[];
  selectedUnitId: number | null;
  planDays: number;
  loading: boolean;
  fetchUnits: (planId: number) => Promise<void>;
  setDays: (days: number) => void;
  selectUnit: (id: number) => void;
}

export const usePlanStore = create<PlanStore>((set) => ({
  units: [],
  selectedUnitId: null,
  planDays: 7,
  loading: false,

  fetchUnits: async (planId: number) => {
    set({ loading: true });
    try {
      const units = await api.listUnits(planId);
      set({
        units,
        selectedUnitId: units[0]?.id ?? null,
        loading: false,
      });
    } catch {
      set({ loading: false });
    }
  },

  setDays: (days: number) => set({ planDays: days }),
  selectUnit: (id: number) => set({ selectedUnitId: id }),
}));

interface StudyStore {
  overlayLabel: string;
  overlayUnitId: number | null;
  isOverlayOpen: boolean;
  overlayAutoRunning: boolean;
  setOverlayOpen: (unitId: number, label: string) => void;
  setOverlayClosed: () => void;
  setAutoRunning: (running: boolean) => void;
}

export const useStudyStore = create<StudyStore>((set) => ({
  overlayLabel: "overlay-main",
  overlayUnitId: null,
  isOverlayOpen: false,
  overlayAutoRunning: true,
  setOverlayOpen: (unitId, label) => set({ overlayUnitId: unitId, overlayLabel: label, isOverlayOpen: true }),
  setOverlayClosed: () => set({ isOverlayOpen: false, overlayUnitId: null }),
  setAutoRunning: (running) => set({ overlayAutoRunning: running }),
}));

interface SettingsStore {
  autoInterval: number;
  studyAlpha: number;
  fetchSettings: () => Promise<void>;
  saveInterval: (val: number) => Promise<void>;
  saveAlpha: (val: number) => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  autoInterval: 3,
  studyAlpha: 0.93,

  fetchSettings: async () => {
    const [interval, alpha] = await Promise.all([
      api.getSetting("auto_interval", "3"),
      api.getSetting("study_alpha", "0.93"),
    ]);
    set({ autoInterval: Number(interval), studyAlpha: Number(alpha) });
  },

  saveInterval: async (val) => {
    await api.setSetting("auto_interval", String(val));
    set({ autoInterval: val });
  },

  saveAlpha: async (val) => {
    await api.setSetting("study_alpha", String(val));
    set({ studyAlpha: val });
  },
}));