def is_isogram(string):
    s = string.lower()
    s = list(filter(lambda x: x.isalpha(), s))

    return len(s) == len(set(s)) 
