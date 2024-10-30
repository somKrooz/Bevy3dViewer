import subprocess

input_path = 'D:/Fire/graphics/renders/Render_%d.png' 
output_path = 'D:/Fire/graphics/renders/Output.mp4'  

command = [
    'ffmpeg',
    "-y", 
    '-framerate', '24',
    '-i', input_path,
    '-c:v', 'libx264',
    '-preset', 'slow',  
    '-b:v', '5000k',  
    '-pix_fmt', 'yuv420p',
    output_path
]

try:
    subprocess.run(command, check=True)
    print("Video rendered successfully!")
except subprocess.CalledProcessError as e:
    print("An error occurred:", e)