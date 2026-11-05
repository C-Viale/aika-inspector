import { writeFile } from "#actions/files";
import { Button } from "#components/button";
import { ProgressBar } from "#components/progress-bar";
import { save } from "@tauri-apps/plugin-dialog";
import { useContext, useEffect, useRef, useState } from "react";
import { BiVolumeFull, BiVolumeLow, BiVolumeMute } from "react-icons/bi";
import { PiPause, PiPlay } from "react-icons/pi";
import "./styles.css";
import { SettingsContext } from "#contexts/settings-context/context";
import { INITIAL_VOLUME, STORAGE_KEYS } from "#utils/constants";

interface AudioViewerProps {
  selectedFileName: string;
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

const MAX_VOLUME = 0.7;

function normalizeVolume(volume: number) {
  return volume ** 2.5;
}

export function AudioViewer({ src, selectedFileName }: AudioViewerProps) {
  const { autoplayAudio, volume, setVolume } = useContext(SettingsContext);

  const [duration, setDuration] = useState(0);
  const [currentTime, setCurrentTime] = useState(0);
  const [paused, setPaused] = useState(true);
  const [muted, setMuted] = useState(false);

  const ref = useRef<HTMLAudioElement>(null);

  const exportAsMp3 = async () => {
    const path = await save({
      defaultPath: selectedFileName,
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
      ref.current.volume = normalizeVolume(newVolume);
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
      setPaused(!target.autoplay);
      setDuration(target.duration);
    };

    const onPause = () => {
      setPaused(true);
    };

    const onPlay = () => {
      setPaused(false);
    };

    const controller = new AbortController();

    const initialVolume = +(localStorage.getItem(STORAGE_KEYS.volume) || INITIAL_VOLUME);
    ref.current.volume = normalizeVolume(initialVolume);
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
      <audio ref={ref} src={src} autoPlay={autoplayAudio} hidden />

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
            {muted ? <BiVolumeMute /> : <>{volume > 0.25 ? <BiVolumeFull /> : <BiVolumeLow />}</>}
          </button>
          <ProgressBar
            min={0}
            max={MAX_VOLUME}
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
