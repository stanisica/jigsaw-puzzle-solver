from PIL import Image
import sys
import os

def slice_image(image_path, vertical_slices, horizontal_slices, output_folder):
    try:

        if not os.path.exists(output_folder):
            os.makedirs(output_folder)

        with Image.open(image_path) as img:
            width, height = img.size

            slice_width = width // vertical_slices
            slice_height = height // horizontal_slices

            extra_width = width % vertical_slices
            extra_height = height % horizontal_slices

            left = 0
            top = 0
            part_num = 0

            for h in range(horizontal_slices):
                for v in range(vertical_slices):
                    right = left + slice_width + (1 if v < extra_width else 0)
                    bottom = top + slice_height + (1 if h < extra_height else 0)

                    slice = img.crop((left, top, right, bottom))
                    slice.save(f"{output_folder}/image{part_num}.png")
                    part_num += 1
                    left = right

                top = bottom
                left = 0

        print("Slicing completed successfully.")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("Usage: python3 image_partitioner.py <image_path> <vertical_slices> <horizontal_slices> <output_folder>")
        sys.exit(1)

    image_path = sys.argv[1]
    vertical_slices = int(sys.argv[2])
    horizontal_slices = int(sys.argv[3])
    output_folder = sys.argv[4]

    slice_image(image_path, vertical_slices, horizontal_slices, output_folder)
