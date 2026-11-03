import { ChangeEvent, useEffect, useRef, useState } from "react";
import "./styles.css";

interface AudioViewerProps {
  src: string;
}

export function AudioViewer(props: AudioViewerProps) {
  const [volume, setVolume] = useState(0.5);

  const ref = useRef<HTMLAudioElement>(null);

  const handleVolumeChange = (e: ChangeEvent<HTMLInputElement>) => {
    const newVolume = Number(e.target.value);
    setVolume(newVolume);
  };

  useEffect(() => {
    if (ref.current) {
      ref.current.volume = volume ** 2.5;
    }
  }, [volume]);

  return (
    <div className="audio-viewer">
      <audio ref={ref} autoPlay controls src={props.src} />
      <div>
        <input
          type="range"
          min="0"
          max=".5"
          step="0.01"
          value={volume}
          onChange={handleVolumeChange}
        />
      </div>
    </div>
  );
}
