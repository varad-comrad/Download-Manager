import os, pathlib, time, select, sys, typing, settings
from watchdog.observers import Observer
from manage_file import img_handler, pdf_handler

def monitor_folder(folder: pathlib.Path) -> None:
    observer = Observer()
    observer.schedule(img_handler, folder)
    observer.schedule(pdf_handler, folder)
    observer.start()
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
        observer.join()

def main():
    folder_to_monitor = pathlib.Path(settings.DOWNLOADS)
    monitor_folder(folder_to_monitor)

if __name__ == '__main__':
    main()
