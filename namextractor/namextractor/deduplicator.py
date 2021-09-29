from typing import List
from textdistance import hamming


def deduplicate(data: List[str]) -> List[str]:
    answer = []

    while len(data) > 0:
        word = data.pop()

        if any([word in x for x in answer]):
            continue

        similar = [word]

        for p_s in data:
            if hamming.normalized_similarity(word, p_s) >= 0.8:
                similar.append(p_s)

        weighted_similar = list(
            map(lambda el: (sum([hamming.normalized_similarity(el, y) for y in similar]), el), similar))

        answer.append(max(weighted_similar)[1])

    return answer
