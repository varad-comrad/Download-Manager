import pathlib, os, shutil, settings

def handler(src: pathlib.Path) -> None:
    for element in src.iterdir():
        if element.is_file():
            if element.suffix.split('.')[-1] == 'pdf':
                shutil.move(element, settings.PDFS)
            elif element.suffix.split('.')[-1] in ['png', 'jpg', 'jpeg', 'svg']:
                shutil.move(element, settings.PICTURES)
            else:
                continue
    