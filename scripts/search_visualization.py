from datetime import datetime

import contextily as ctx
import geopandas as gpd
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.animation import FuncAnimation
from matplotlib.cm import ScalarMappable
from matplotlib.colors import Normalize
from pyproj import Transformer

# Load the CSV file into a DataFrame
csv_file = "results/pathing/search_coordinates.csv"
df = pd.read_csv(csv_file)

# df = df.head(1000)

transformer = Transformer.from_crs("EPSG:4326", "EPSG:3857", always_xy=True)
df['x'], df['y'] = transformer.transform(df['lon'].values, df['lat'].values)
gdf = gpd.GeoDataFrame(df, geometry=gpd.points_from_xy(df['x'], df['y']), crs="EPSG:3857")

minx, miny, maxx, maxy = gdf.total_bounds
fig, ax = plt.subplots(figsize=(10, 10), dpi=150)
padding = 10000
ax.set_xlim(minx - padding, maxx + padding)
ax.set_ylim(miny - padding, maxy + padding)
ctx.add_basemap(ax, source="https://tile.openstreetmap.org/{z}/{x}/{y}.png", zoom=10)
ax.set_axis_off()

# Number of points to add per frame
points_per_frame = 100

# Number of frames to add points in batches
num_frames = (len(gdf) + points_per_frame - 1) // points_per_frame

# Create a colormap object
cmap = plt.get_cmap('rainbow')
norm = Normalize(vmin=0, vmax=len(gdf))
sm = ScalarMappable(norm=norm, cmap=cmap)

# Initialize scatter plot with an empty list of offsets and colors
scat = ax.scatter([], [], c=[], cmap=cmap, marker='.', s=5, norm=norm)

def init():
    scat.set_offsets(np.empty((0, 2)))  # Correctly initialized as empty 2D array
    scat.set_array(np.array([]))  # Initialize with no colors
    return scat,

def update(frame):
    # Calculate the end index for this frame
    start = frame * points_per_frame
    end = start + points_per_frame
    
    print(f"Frame {frame}")
    
    # Ensure the end index does not exceed the total number of points
    if end > len(gdf):
        end = len(gdf)
    
    current_data = gdf.iloc[:end]
    offsets = current_data[['x', 'y']].values
    
    # Set the offsets
    scat.set_offsets(offsets)
    
    # Generate a color for each point based on its overall position
    colors = np.linspace(0, end, end)
    scat.set_array(colors)
    
    return scat,


print(f"Running for {int(len(df) / points_per_frame)} frames")

anim = FuncAnimation(fig, update, frames=num_frames, init_func=init, interval=20, blit=True, repeat=False)
anim.save('results/animations/a-star-search-groningen.mp4', writer='ffmpeg')

plt.show()

fig.savefig(f"results/figures/{datetime.now().strftime('%d%m%Y - ')}OpenStreetMap - A-star example - Groningen.png", dpi=300)
