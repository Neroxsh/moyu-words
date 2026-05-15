import { useEffect, useRef } from "react";

interface Props {
  word: string;
}

interface Fragment {
  x: number; y: number;
  vx: number; vy: number;
  rotation: number; angularVelocity: number;
  opacity: number; scale: number;
  width: number; height: number;
  imageData: ImageData | null;
}

export default function ShatterCanvas({ word }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animRef = useRef<number>(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Size canvas to fill parent
    const resize = () => {
      const parent = canvas.parentElement;
      if (parent) {
        canvas.width = parent.clientWidth;
        canvas.height = parent.clientHeight;
      }
    };
    resize();

    // Render word to offscreen canvas to capture bitmap
    const offscreen = document.createElement("canvas");
    const offCtx = offscreen.getContext("2d")!;
    offscreen.width = canvas.width;
    offscreen.height = canvas.height;
    offCtx.font = "bold 18px 'Inter', 'SF Pro Display', -apple-system, sans-serif";
    offCtx.fillStyle = "rgba(248, 250, 252, 0.95)";
    offCtx.textAlign = "center";
    offCtx.textBaseline = "middle";
    const textMetrics = offCtx.measureText(word);
    const textWidth = textMetrics.width;
    const textHeight = 24;
    const textX = canvas.width / 2;
    const textY = canvas.height / 2;
    offCtx.fillText(word, textX, textY);

    // Get text bounding box
    const left = textX - textWidth / 2;
    const top = textY - textHeight / 2;

    // Generate Voronoi-like fragments
    const fragments: Fragment[] = [];
    const seedCount = 8;
    const seeds: { x: number; y: number }[] = [];

    for (let i = 0; i < seedCount; i++) {
      seeds.push({
        x: left + Math.random() * textWidth,
        y: top + Math.random() * textHeight,
      });
    }

    // Create fragments — one per "crack region"
    const fragCount = 35;
    for (let i = 0; i < fragCount; i++) {
      const fx = left + Math.random() * textWidth;
      const fy = top + Math.random() * textHeight;
      const fw = 4 + Math.random() * 20;
      const fh = 3 + Math.random() * 12;

      // Capture the pixel data for this fragment
      try {
        const imageData = offCtx.getImageData(
          Math.max(0, Math.floor(fx)),
          Math.max(0, Math.floor(fy)),
          Math.ceil(fw),
          Math.ceil(fh)
        );
        fragments.push({
          x: fx, y: fy,
          vx: (Math.random() - 0.5) * 120,
          vy: -30 - Math.random() * 80 + Math.random() * 20,
          rotation: (Math.random() - 0.5) * Math.PI * 0.5,
          angularVelocity: (Math.random() - 0.5) * 4,
          opacity: 1,
          scale: 1,
          width: fw, height: fh,
          imageData,
        });
      } catch {
        // Skip fragments outside canvas bounds
      }
    }

    const startTime = performance.now();
    const duration = 3000;

    function animate(now: number) {
      const elapsed = now - startTime;
      const progress = Math.min(elapsed / duration, 1);

      // Ease-out cubic
      const t = 1 - Math.pow(1 - progress, 3);

      ctx!.clearRect(0, 0, canvas!.width, canvas!.height);

      for (const f of fragments) {
        if (f.imageData === null) continue;

        ctx!.save();

        const cx = f.x + f.width / 2;
        const cy = f.y + f.height / 2;

        ctx!.translate(cx + f.vx * t, cy + f.vy * t + 60 * t * t);
        ctx!.rotate(f.rotation + f.angularVelocity * progress * Math.PI);
        ctx!.scale(f.scale * (1 - t * 0.3), f.scale * (1 - t * 0.3));
        ctx!.globalAlpha = f.opacity * (1 - t);

        // Create a small canvas to draw the image data
        const tempCanvas = document.createElement("canvas");
        tempCanvas.width = f.width;
        tempCanvas.height = f.height;
        const tempCtx = tempCanvas.getContext("2d")!;
        tempCtx.putImageData(f.imageData, 0, 0);
        ctx!.drawImage(tempCanvas, -f.width / 2, -f.height / 2);

        ctx!.restore();
      }

      // Add some small sparkle particles
      if (progress < 0.8) {
        ctx!.save();
        ctx!.globalAlpha = (1 - progress / 0.8) * 0.4;
        for (let i = 0; i < 15; i++) {
          const px = textX + (Math.random() - 0.5) * textWidth * (1 + progress * 2);
          const py = textY + (Math.random() - 0.5) * textHeight * (1 + progress * 2);
          ctx!.fillStyle = "rgba(108, 140, 255, 0.8)";
          ctx!.beginPath();
          ctx!.arc(px, py, 1 + Math.random() * 2, 0, Math.PI * 2);
          ctx!.fill();
        }
        ctx!.restore();
      }

      if (progress < 1) {
        animRef.current = requestAnimationFrame(animate);
      }
    }

    animRef.current = requestAnimationFrame(animate);

    return () => {
      if (animRef.current) cancelAnimationFrame(animRef.current);
    };
  }, [word]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        width: "100%",
        height: "100%",
      }}
    />
  );
}