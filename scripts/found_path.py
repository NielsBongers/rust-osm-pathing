from datetime import datetime

import contextily as ctx
import geopandas as gpd
import matplotlib.pyplot as plt
import pandas as pd
from pyproj import Transformer
from shapely.geometry import LineString

# Load the CSV file into a DataFrame
csv_file = "results/analysis/worst_case_map.csv"
df = pd.read_csv(csv_file)

# Transform the coordinates from EPSG:4326 to EPSG:3857
transformer = Transformer.from_crs("EPSG:4326", "EPSG:3857", always_xy=True)
df['x'], df['y'] = transformer.transform(df['lon'].values, df['lat'].values)

# Create a GeoDataFrame
gdf = gpd.GeoDataFrame(df, geometry=gpd.points_from_xy(df['x'], df['y']), crs="EPSG:3857")

# Get the bounding box of the GeoDataFrame
minx, miny, maxx, maxy = gdf.total_bounds

# Create a plot
# fig, ax = plt.subplots(figsize=(10, 10), dpi=150)
ax = plt.gca() 

# Set padding for the plot
padding = 100
ax.set_xlim(minx - padding, maxx + padding)
ax.set_ylim(miny - padding, maxy + padding)

# Add a basemap from OpenStreetMap
ctx.add_basemap(ax, source="https://tile.openstreetmap.org/{z}/{x}/{y}.png", zoom=19)

# Plot the GeoDataFrame points
gdf.plot(ax=ax, marker='o', color='red', markersize=5)

# Create a LineString from the points
line = LineString(gdf.geometry.tolist())

# Plot the line with green color and alpha of 0.5
gpd.GeoSeries([line], crs="EPSG:3857").plot(ax=ax, color='green', linewidth=2, alpha=0.5)

# Remove axis for better visualization
ax.set_axis_off()

plt.savefig(f"results/figures/{datetime.now().strftime('%d%m%Y - ')}OpenStreetMap - result map.png", dpi=300)

# Display the plot
plt.show()
