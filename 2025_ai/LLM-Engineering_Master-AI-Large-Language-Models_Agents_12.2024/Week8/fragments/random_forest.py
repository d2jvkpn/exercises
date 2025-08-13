#!/usr/bin/env python3
import os, pickle
os.environ['HF_HUB_OFFLINE'] = 'True'

import joblib
import numpy as np
from sklearn.model_selection import GridSearchCV
from sklearn.ensemble import RandomForestRegressor
from sentence_transformers import SentenceTransformer
from sklearn.metrics import mean_absolute_error, mean_squared_error, r2_score

model = SentenceTransformer('./data/all-MiniLM-L6-v2')
#model = model.to("gpu")


def y_metrics(y_test, y_pred):
    mae = mean_absolute_error(y_test, y_pred)
    mse = mean_squared_error(y_test, y_pred)
    rmse = np.sqrt(mse)
    r2 = r2_score(y_test, y_pred)

    return round(mae, 3), round(mse, 3), round(float(rmse), 3), round(r2, 3)


with open('data/test.pkl', 'rb') as file:
    test = pickle.load(file)

with open('data/train.pkl', 'rb') as file:
    train = pickle.load(file)

sents = train['prompt'].apply(lambda x: x.split('\n\n', 1)[1].rsplit('\n\n')[0]).to_list()
vectors = model.encode(sents)

pickle.dump(vectors, 'data/train.vectors.pkl')

prices = train['price'].to_list()

rf_model = RandomForestRegressor(n_estimators=100, random_state=42, n_jobs=-1)
rf_model.fit(vectors, prices)

joblib.dump(rf_model, 'data/random_forest.model.pkl')

predicate = rf_model.predict(vectors)

print(y_metrics(prices, predicate))
# (13.063, 800.134, 28.287, 0.93)


####
test_sents = test['prompt'].apply(lambda x: x.split('\n\n', 1)[1].rsplit('\n\n')[0]).to_list()
test_vectors = model.encode(test_sents)
test_prices = test['price'].to_list()
test_predict = rf_model.predict(test_vectors)

print(y_metrics(test_prices, test_predict))
# (36.034, 6209.451, 78.8, 0.494)

####
param_grid = {
    'n_estimators': [50, 100, 200],
    'max_depth': [None, 10, 20, 30],
    'min_samples_split': [2, 5, 10],
    'min_samples_leaf': [1, 2, 4]
}

os.environ['TOKENIZERS_PARALLELISM'] = "True"

grid_search = GridSearchCV(
    estimator=RandomForestRegressor(random_state=42),
    param_grid=param_grid,
    cv=5, n_jobs=-1, verbose=2,
)

grid_search.fit(vectors, prices)
print("Best parameters:", grid_search.best_params)
gs_rf = grid_search.best_params


predicate = rf_model.predict(vectors)
print(y_metrics(prices, predicate))

test_predict = rf_model.predict(test_vectors)
print(y_metrics(test_prices, test_predict))
# (36.034, 6209.451, 78.8, 0.494)
