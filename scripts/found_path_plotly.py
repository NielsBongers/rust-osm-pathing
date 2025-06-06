import webbrowser
from pathlib import Path

import pandas as pd
import plotly.graph_objects as go

# Load the CSV file into a DataFrame
csv_file = "results/analysis/worst_case_map.csv"
df = pd.read_csv(csv_file)

# Ensure lat and lon are provided as lists
latitudes = df['lat'].tolist()  # Extract latitudes as a list
longitudes = df['lon'].tolist()  # Extract longitudes as a list
node_ids = df['node_id'].tolist()  # Extract node_id as a list

# Initialize the figure
fig = go.Figure()

# Add scatter mapbox for points with hover text showing the node_id
fig.add_trace(go.Scattermapbox(
    lat=latitudes,
    lon=longitudes,
    mode='markers',
    marker=go.scattermapbox.Marker(
        size=5,
        color='green'
    ),
    text=[f"Node ID: {node_id}" for node_id in node_ids],  # Add node_id as hover text
    hoverinfo='text',  # Display the text on hover
    name='Points'
))

# Add line for the path between the nodes using latitudes and longitudes directly
fig.add_trace(go.Scattermapbox(
    lat=latitudes,
    lon=longitudes,
    mode='lines+markers',  # Combine lines and markers for visibility
    line=dict(
        width=2,
        color='green'
    ),
    marker=go.scattermapbox.Marker(
        size=5,
        color='green'
    ),
    text=[f"Node ID: {node_id}" for node_id in node_ids],  # Add node_id as hover text for lines
    hoverinfo='text',  # Display the text on hover
    name='Path'
))

# Set layout for the figure with OpenStreetMap style
fig.update_layout(
    mapbox=dict(
        style='open-street-map',  # Use OpenStreetMap style
        zoom=10,
        center=dict(lat=latitudes[len(latitudes) // 2], lon=longitudes[len(longitudes) // 2])
    ),
    margin={"r": 0, "t": 0, "l": 0, "b": 0},
    showlegend=True  # Show legend for clarity
)

# Save the figure to HTML
output_file = Path("results/maps/OpenStreetMap - Plotly map.html").absolute()
fig.write_html(output_file)

webbrowser.open(output_file)