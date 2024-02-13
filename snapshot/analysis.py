import json
import numpy as np
import matplotlib.pyplot as plt
from sklearn.decomposition import PCA
from sklearn.preprocessing import StandardScaler
import biplot

# Load the JSON data
with open('snapshot/cleaned/spotify-ukchart-2023-04-24.json') as f:
    data = json.load(f)

# Extract the audio features and ranking from the JSON data
audio_features = []
ranking = []
for track in data:
    features = [track['audio_feature']['acousticness'], 
                track['audio_feature']['danceability'], 
                track['audio_feature']['energy'], 
                track['audio_feature']['instrumentalness'], 
                track['audio_feature']['liveness'], 
                track['audio_feature']['loudness'], 
                track['audio_feature']['speechiness'], 
                track['audio_feature']['tempo'],
                track['popularity']]
    audio_features.append(features)
    ranking.append(track['popularity'])

# Get the name of the audio features
audio_feature_names = ['acousticness', 
                       'danceability', 
                       'energy', 
                       'instrumentalness', 
                       'liveness', 
                       'loudness', 
                       'speechiness', 
                       'tempo',
                       'popularity']

# Convert the audio features and ranking into numpy arrays
audio_features = StandardScaler().fit_transform(np.array(audio_features))
ranking = np.array(ranking)

# Perform PCA
pca = PCA(n_components=2) # dimension
transformed_data = pca.fit_transform(audio_features)

print(pca.explained_variance_ratio_)

# Create a biplot
biplot.biplot(score=transformed_data[:, 0:2],
              coeff=np.transpose(pca.components_[0:2, :]),
              coeff_labels=audio_feature_names,
              cat=ranking,
              cmap='viridis')

plt.show()
