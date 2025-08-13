#!/usr/bin/env python3
import pickle

from sklearn.ensemble import RandomForestRegressor
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_squared_error, r2_score


with open('data/test.pkl', 'rb') as file:
    test = pickle.load(file)

with open('data/train.pkl', 'rb') as file:
    train = pickle.load(file)


result = collection.get(include=['embeddings', 'documents', 'metadatas'])
vectors = np.array(result['embeddings'])
documents = result['documents']
prices = [metadata['price'] for metadata in result['metadatas']]

def find_similars(item):
    results = collection.query(query_embeddings=vector(item).astype(float).tolist(), n_results=5)
    documents = results['documents'][0][:]
    prices = [m['price'] for m in results['metadatas'][0][:]]
    return documents, prices


####
rf_model = RandomForestRegressor(n_estimators=100, random_state=42, n_jobs=-1)
rf_model.fit(vectors, prices)

joblib.dump(rf_model, 'data/random_forest_model.pkl')

rf_model = joblib.load('data/random_forest_model.pkl')


####
from agents.specialist_agent import SpecialistAgent
from agents.frontier_agent import FrontierAgent
from agents.random_forest_agent import RandomForestAgent

specialists = []
frontiers = []
random_forests = []
prices = []

for item in tqdm(test[1000:1250]):
    text = description(item)
    specialists.append(specialist.price(text))
    frontiers.append(frontier.price(text))
    random_forests.append(random_forest.price(text))
    prices.append(item.price)

mins = [min(s,f,r) for s,f,r in zip(specialists, frontiers, random_forests)]
maxes = [max(s,f,r) for s,f,r in zip(specialists, frontiers, random_forests)]

X = pd.DataFrame({
    'Specialist': specialists,
    'Frontier': frontiers,
    'RandomForest': random_forests,
    'Min': mins,
    'Max': maxes,
})

# Convert y to a Series
y = pd.Series(prices)

joblib.dump(lr, 'ensemble_model.pkl')

np.random.seed(42)

lr = LinearRegression()
lr.fit(X, y)

feature_columns = X.columns.tolist()

for feature, coef in zip(feature_columns, lr.coef_):
    print(f"{feature}: {coef:.2f}")

print(f"Intercept={lr.intercept_:.2f}")

joblib.dump(lr, 'data/ensemble_model.pkl')


####
from agents.ensemble_agent import EnsembleAgent
ensemble = EnsembleAgent(collection)

ensemble.price(product)

def ensemble_pricer(item):
    return max(0, ensemble.price(description(item)))

Tester.test(ensemble_pricer, test)
