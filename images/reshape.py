import os
from PIL import Image

def convert_images(directory):
    for filename in os.listdir(directory):
        if filename.endswith('.jpg'):
            filepath = os.path.join(directory, filename)
            with Image.open(filepath) as img:
                new_filepath = os.path.splitext(filepath)[0] + '.png'
                img.save(new_filepath, 'PNG')
                print(f"Converted {filename} to {os.path.basename(new_filepath)}")

            os.remove(filepath)

if __name__ == "__main__":
    directory = './example_1/parts_2/'  
    convert_images(directory)