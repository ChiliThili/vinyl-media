import {
  getCurrentWindow,
  currentMonitor,
  LogicalPosition
} from "@tauri-apps/api/window";

export async function positionWidget() {
  const monitor = await currentMonitor();

  if (!monitor) return;

  const width = 320;
  const margin = 20;

  const x =
    monitor.size.width / monitor.scaleFactor - width - margin;

  await getCurrentWindow().setPosition(
    new LogicalPosition(x, 60)
  );
}
