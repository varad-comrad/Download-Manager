import pathlib, settings

def handler(src: pathlib.Path) -> None:
    shutil = FolderHandler()
    for element in src.iterdir():
        if element.is_file():
            if element.suffix.split('.')[-1] == 'pdf':
                shutil.move(element, settings.PDFS)
            elif element.suffix.split('.')[-1] in ['png', 'jpg', 'jpeg', 'svg']:
                shutil.move(element, settings.PICTURES)
            else:
                continue
    
class FolderHandler:
    def __init__(self) -> None:
        pass

    def move(self, src: pathlib.Path, dest: pathlib.Path | str) -> None:
        if not isinstance(dest, pathlib.Path):
            dest = pathlib.Path(dest)
        aux = dest / src.name
        src.rename(aux)