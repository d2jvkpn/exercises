#!/usr/bin/env python3
from importlib.metadata import version

import tiktoken
print("tiktoken version:", version("tiktoken"))


class SimpleTokenizer:
    def __init__(self, vocab):
        self.str_to_int = vocab
        self.int_to_str = { i:s for s, i in vocab.items()}

    def encode(self, text):
        preprocessed = re.split(r'([,.:;?_!"()\']|--|\s)', text)
        preprocessed = [item.strip() for item in preprocessed if item.strip()]

    preprocessed = [item if item in self.str_to_int else "<|unk|>" for item in preprocessed]

    ids = [self.str_to_int[s] for s in preprocessed]
    return ids

    def decode(self, ids):
        text = " ".join([self.int_to_str[i] for i in ids])
        text = re.sub(r'\s+([,.:;?!"()\'])', r'\1', text)
        return text


def test_split_tokens(size, max_len, stride):
    assert size >= max_len

    starts = list(range(0, size - max_len, stride))
    return [list(range(s, s + max_len)) for s in starts]

    # print(split_tokens(100, 10, 4))

def get_stats(ids, counts=None):
    """
    Given a list of integers, return a dictionary of counts of consecutive pairs
    Example: [1, 2, 3, 1, 2] -> {(1, 2): 2, (2, 3): 1, (3, 1): 1}
    Optionally allows to update an existing dictionary of counts
    """
    counts = {} if counts is None else counts
    for pair in zip(ids, ids[1:]):  # iterate consecutive elements
        counts[pair] = counts.get(pair, 0) + 1
    return counts


def merge(ids, pair, idx):
    """
    In the list of integers (ids), replace all consecutive occurrences
    of pair with the new integer token idx
    Example: ids=[1, 2, 3, 1, 2], pair=(1, 2), idx=4 -> [4, 3, 4]
    """
    i, newids = 0, []
    while i < len(ids):
        # if not at the very last position AND the pair matches, replace it
        if i < len(ids) - 1 and (ids[i], ids[i+1]) == pair:
            newids.append(idx)
            i += 2
        else:
            newids.append(ids[i])
            i += 1

    return newids
