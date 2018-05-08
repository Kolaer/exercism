def is_pangram(sentence):
    s = sentence.lower()
    s = filter(lambda x: x.isalpha(), s)

    s = set(s)

    return 26 == len(s)
