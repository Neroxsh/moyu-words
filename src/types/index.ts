export interface BookRow {
  id: number;
  title: string;
  source_type: string;
  source_ref: string | null;
  source_url: string | null;
  total_words: number;
  created_at: string;
  updated_at: string;
  active: number;
  actual_words: number;
}

export interface BookSummary {
  id: number;
  title: string;
  source_type: string;
  total_words: number;
  active_words: number;
  archived_words: number;
  reviewed_words: number;
  progress_text: string;
}

export interface WordRow {
  id: number;
  book_id: number;
  seq: number;
  word: string;
  meaning: string;
  raw: string;
  status: string;
  archived_at: string | null;
}

export interface PlanRow {
  id: number;
  book_id: number;
  days: number;
  words_per_day: number;
  created_at: string;
  active: number;
}

export interface UnitRow {
  id: number;
  plan_id: number;
  unit_no: number;
  start_seq: number;
  end_seq: number;
  status: string;
  completed_at: string | null;
  total_words: number;
  pending_words: number;
  reviewed_words: number;
}

export interface UnitWordRow {
  id: number;
  unit_id: number;
  word_id: number;
  position: number;
  review_state: string;
  studied_at: string | null;
  word: string;
  meaning: string;
  word_status: string;
  seq: number;
}

export interface CurrentWord {
  unit_word_id: number;
  word_id: number;
  word: string;
  meaning: string;
  position: number;
  total_words: number;
  reviewed_words: number;
  auto_running: boolean;
}

export interface BuiltinBookDef {
  index: number;
  title: string;
  filename: string;
}

export interface OverlayInitPayload {
  unit_id: number;
}