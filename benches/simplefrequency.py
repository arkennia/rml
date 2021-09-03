# Copyright 2021 Jonathan Manly.

# This file is part of rml.

# rml is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# rml is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with rml.  If not, see <https://www.gnu.org/licenses/>.

from sklearn.feature_extraction.text import CountVectorizer
from sklearn.feature_extraction.text import TfidfTransformer

from nltk.tokenize import TreebankWordTokenizer
import csv


features = []
with open("./data/test_data/IMDB_Dataset.csv", newline='', encoding='utf-8') as f:
    reader = csv.reader(f)
    for row in reader:
        if len(row) > 0:
            features.append(row[0])

# Normal


def vectorize(i):
    global features
    count_vect = CountVectorizer(max_features=i)
    tokenizer = TreebankWordTokenizer()
    count_vect.set_params(tokenizer=tokenizer.tokenize)
    count_vect.set_params(stop_words='english')

    count_vect.set_params(ngram_range=(1, 2))
    X_counts = count_vect.fit_transform(features)
    tfidf_transformer = TfidfTransformer()
    X_counts = tfidf_transformer.fit_transform(X_counts)


if __name__ == '__main__':
    import timeit
    import gc  # noqa:
    print(timeit.timeit("vectorize(50)",
                        setup="gc.enable()", number=1, globals=globals()))
    print(timeit.timeit("vectorize(100)",
                        setup="gc.enable()", number=1, globals=globals()))
    print(timeit.timeit("vectorize(1000)",
                        setup="gc.enable()", number=1, globals=globals()))
    print(timeit.timeit("vectorize(10000)",
                        setup="gc.enable()", number=1, globals=globals()))
