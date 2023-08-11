import pathlib, time, settings
from manage_file import handler


def monitor(folder: pathlib.Path, cache: list[pathlib.Path]) -> None:
    if list(folder.iterdir()) != cache:
        handler(folder)
    cache = list(folder.iterdir())

def watch_dog(folder: pathlib.Path) -> None:
    cache = []
    try:
        while True:
            time.sleep(1)
            monitor(folder, cache)
    except KeyboardInterrupt:
            return
def main():
    folder_to_monitor = pathlib.Path(settings.DOWNLOADS)
    watch_dog(folder_to_monitor)

if __name__ == '__main__':
    main()
