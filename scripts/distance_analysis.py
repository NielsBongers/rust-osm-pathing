from datetime import datetime
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


def plot_angles(df: pd.DataFrame): 
    
    angle_increment = 5
    
    bins = np.arange(-180, 181, angle_increment)
    df['bearing_bin'] = pd.cut(df['bearing'], bins=bins, labels=bins[:-1])

    mean_path_ratio = df.groupby('bearing_bin', observed=True)['path_ratio'].mean().dropna()
    bin_centers = np.deg2rad(mean_path_ratio.index.astype(float))

    ax = plt.subplot(111, polar=True)
    
    ax.bar(bin_centers, mean_path_ratio, width=np.deg2rad(angle_increment), alpha=0.8)

    ax.set_theta_zero_location('N')
    ax.set_theta_direction(-1)

    ax.set_title("Path and great circle distance ratios")
    
    plt.savefig(f"results/figures/{datetime.now().strftime('%d%m%Y - ')}OpenStreetMap - directional plot for Erdenet.png", dpi=300)
    plt.show()
    
def ratio_histogram(df: pd.DataFrame): 
    
    percentile = df["path_ratio"].quantile(0.999)
    
    df = df[(df['path_ratio'] >= 1.0) & (df['path_ratio'] <= percentile)]
    
    plt.hist(df["path_ratio"], bins=100)
    plt.xlabel("Ratio")
    plt.ylabel("Count")
    plt.xlim([1, 2])
    # plt.semilogy()
    plt.savefig(f"results/figures/{datetime.now().strftime('%d%m%Y - ')}OpenStreetMap - ratio histograms Erdenet.png", dpi=300)
    plt.show()

def main(): 
    analysis_path = Path("results/analysis/analysis_results.parquet")
    df = pd.read_parquet(analysis_path)
    
    print(df.describe())
    
    print(df["path_ratio"].median())
    
    plot_angles(df)
    ratio_histogram(df)

if __name__ == "__main__": 
    main() 