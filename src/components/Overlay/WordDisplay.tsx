interface Props {
  word: string;
  meaning: string;
  progress: string;
}

export default function WordDisplay({ word, meaning, progress }: Props) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 16,
        width: "100%",
        height: "100%",
        padding: "4px 8px",
      }}
    >
      {/* Word */}
      <span
        style={{
          color: "rgba(248, 250, 252, 0.95)",
          fontSize: 18,
          fontWeight: 700,
          fontFamily: "'Inter', 'SF Pro Display', -apple-system, sans-serif",
          whiteSpace: "nowrap",
          textShadow: "0 1px 4px rgba(0,0,0,0.3)",
        }}
      >
        {word}
      </span>

      {/* Meaning */}
      <span
        style={{
          color: "rgba(148, 163, 184, 0.85)",
          fontSize: 14,
          fontFamily: "'Inter', 'SF Pro Display', -apple-system, sans-serif",
          flex: 1,
          overflow: "hidden",
          textOverflow: "ellipsis",
          whiteSpace: "nowrap",
        }}
      >
        {meaning}
      </span>

      {/* Progress */}
      <span
        style={{
          color: "rgba(148, 163, 184, 0.5)",
          fontSize: 11,
          fontFamily: "'Inter', sans-serif",
          whiteSpace: "nowrap",
          flexShrink: 0,
        }}
      >
        {progress}
      </span>
    </div>
  );
}