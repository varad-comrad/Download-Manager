import pathlib, os, shutil, settings
from watchdog.events import PatternMatchingEventHandler

pdf_handler = PatternMatchingEventHandler(['*.pdf'], ignore_directories=True, case_sensitive=True)
img_handler = PatternMatchingEventHandler(['*.png', '*.jpeg', '*.jpg', '*.svg'], ignore_directories=True, case_sensitive=True)

def pdf_on_created(event):
    shutil.move(event.src_path, pathlib.Path(settings.PDFS))

def pdf_on_moved(event):
    shutil.move(event.src_path, pathlib.Path(settings.PDFS))

def img_on_created(event):
    shutil.move(event.src_path, pathlib.Path(settings.PICTURES))

def img_on_moved(event):
    shutil.move(event.src_path, pathlib.Path(settings.PICTURES))

pdf_handler.on_created = pdf_on_created
img_handler.on_created = img_on_created
pdf_handler.on_moved = pdf_on_moved
img_handler.on_moved = img_on_moved