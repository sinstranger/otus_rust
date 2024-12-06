import pyrust


with open("images/ararat.png", "rb") as f:
    image_data = f.read()


resized_bytes = pyrust.resize_image_from_bytes(image_data, 100, 100, "png")

with open("images/output_resized.png", "wb") as f:
    f.write(resized_bytes)


glitched_bytes = pyrust.glitch_effect(image_data, 10, "png")

with open("images/output_glitched.png", "wb") as f:
    f.write(glitched_bytes)


print("Done!")
