import pyrust

with open("images/ararat.png", "rb") as f:
    image_data = f.read()

resized_bytes = pyrust.resize_image_from_bytes(image_data, 100, 100, "png")

with open("images/output_resized.png", "wb") as f:
    f.write(resized_bytes)

print("Done!")
