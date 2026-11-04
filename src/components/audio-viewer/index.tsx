import { writeFile } from "#actions/files";
import { Button } from "#components/button";
import { ProgressBar } from "#components/progress-bar";
import { getFileNameFromPath } from "#utils/functions/path";
import { save } from "@tauri-apps/plugin-dialog";
import { useEffect, useRef, useState } from "react";
import { BiVolume, BiVolumeMute } from "react-icons/bi";
import { PiPause, PiPlay } from "react-icons/pi";
import "./styles.css";

interface AudioViewerProps {
  selectedFilePath: string;
  src: string;
}

function formatTime(seconds: number) {
  const minutes = Math.floor(seconds / 60);
  const restSeconds = Math.floor(seconds % 60)
    .toString()
    .padStart(2, "0");

  const mili = (seconds * 1000) % 1000;
  return `${minutes}:${restSeconds}.${mili.toString()[0]}`;
}

export function AudioViewer({ src, selectedFilePath }: AudioViewerProps) {
  const [volume, setVolume] = useState(0.3);

  const [duration, setDuration] = useState(0);
  const [currentTime, setCurrentTime] = useState(0);
  const [paused, setPaused] = useState(true);
  const [muted, setMuted] = useState(false);

  const ref = useRef<HTMLAudioElement>(null);

  const exportAsMp3 = async () => {
    const path = await save({
      defaultPath: getFileNameFromPath(selectedFilePath, false),
      filters: [
        {
          name: "mp3",
          extensions: ["mp3"],
        },
      ],
    });

    if (!path) return;

    let buffer = await fetch(src)
      .then((r) => r.blob())
      .then((b) => b.bytes());

    await writeFile(path, buffer);
  };

  const toggleMuted = () => {
    setMuted((prev) => {
      if (ref.current) ref.current.muted = !prev;
      return !prev;
    });
  };

  const togglePlayPause = () => {
    if (ref.current?.paused) {
      ref.current?.play();
    } else {
      ref.current?.pause();
    }
  };

  const changeCurrentTime = (value: number) => {
    setCurrentTime(value);
    if (ref.current) ref.current.currentTime = value;
  };

  const changeVolume = (newVolume: number) => {
    setVolume(newVolume);
    setMuted(false);
    if (ref.current) {
      ref.current.volume = newVolume ** 2.5;
      ref.current.muted = false;
    }
  };

  useEffect(() => {
    if (!ref.current) return;

    const onTimeUpdate = (e: Event) => {
      const target = e.target as HTMLAudioElement;
      setCurrentTime(target.currentTime);
    };

    const onLoadMetadata = (e: Event) => {
      const target = e.target as HTMLAudioElement;
      target.volume = 0.2;
      setPaused(!target.autoplay);
      setVolume(0.2);
      setDuration(target.duration);
    };

    const onPause = () => {
      setPaused(true);
    };

    const onPlay = () => {
      setPaused(false);
    };

    const controller = new AbortController();

    ref.current.volume = 0.5;

    ref.current.addEventListener("timeupdate", onTimeUpdate, { signal: controller.signal });
    ref.current.addEventListener("loadeddata", onLoadMetadata, { signal: controller.signal });
    ref.current.addEventListener("pause", onPause, { signal: controller.signal });
    ref.current.addEventListener("play", onPlay, { signal: controller.signal });

    return () => {
      controller.abort();
    };
  }, []);

  return (
    <div className="audio-viewer">
      <audio ref={ref} src={src} autoPlay hidden />

      <div className="audio-player">
        <button type="button" className="audio-btn" onClick={togglePlayPause}>
          {paused ? <PiPlay /> : <PiPause />}
        </button>

        <div className="audio-time">
          <span>{formatTime(currentTime)}</span>
        </div>

        <ProgressBar
          min={0}
          max={duration}
          step="0.01"
          value={currentTime}
          onChange={changeCurrentTime}
          width="300px"
        />

        <div className="audio-time">
          <span>{formatTime(duration)}</span>
        </div>

        <div className="audio-volume">
          <button className="audio-btn" onClick={toggleMuted}>
            {muted ? <BiVolumeMute /> : <BiVolume />}
          </button>
          <ProgressBar
            min={0}
            max={1}
            step="0.01"
            value={volume}
            onChange={changeVolume}
            width="100px"
          />
        </div>
      </div>
      <div className="export-container">
        <Button onClick={exportAsMp3}>Export as MP3</Button>
      </div>
    </div>
  );
}
